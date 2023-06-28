use anyhow::{Context, Result};
use uuid::Uuid;

use crate::{
    chat_context::ChatContext, chat_response::ChatResponse,
    function_specification::FunctionSpecification, message::Message,
};

const DEFAULT_MODEL: &str = "gpt-3.5-turbo-0613";
const URL: &str = "https://api.openai.com/v1/chat/completions";

/// The ChatGPT object
pub struct ChatGPT {
    client: reqwest::Client,
    pub model: String,
    openai_api_token: String,
    pub session_id: String,
    pub chat_context: ChatContext,
}

impl ChatGPT {
    /// Create a new ChatGPT object
    /// # Arguments
    /// * `openai_api_token` - The API token from OpenAI
    /// * `chat_context` - The context of the chatbot.
    /// Optional. If not provided, it will start a new context with the default model
    /// * `session_id` - The session ID of the chatbot.
    /// Optional. If not provided, it will generate a new session ID. This will be useful to track the conversation history
    /// # Example
    /// ```
    /// use chatgpt_functions::chat_gpt::ChatGPT;
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let key = std::env::var("OPENAI_API_KEY")?;
    ///     let mut gpt = ChatGPT::new(key, None, None, None)?;
    ///     Ok(())
    /// }
    /// ```
    /// # Errors
    /// It returns an error if the API token is not valid
    /// # Panics
    /// It panics if the API token is not provided
    /// # Remarks
    /// The API token can be found on the [OpenAI API keys](https://platform.openai.com/account/api-keys)
    pub fn new(
        openai_api_token: String,
        model: Option<String>,
        chat_context: Option<ChatContext>,
        session_id: Option<String>,
    ) -> Result<ChatGPT> {
        let client = reqwest::Client::new();
        let session_id = if let Some(session_id) = session_id {
            session_id
        } else {
            Uuid::new_v4().to_string()
        };
        let chat_context = if let Some(chat_context) = chat_context {
            chat_context
        } else {
            ChatContext::new(DEFAULT_MODEL.to_string())
        };
        Ok(ChatGPT {
            client,
            model: model.unwrap_or(DEFAULT_MODEL.to_string()),
            openai_api_token,
            session_id,
            chat_context,
        })
    }

    /// Calls the OpenAI API to get a response using the current context
    /// # Arguments
    /// * `message` - The message to send to the AI
    /// # Errors
    /// It returns an error if the API token is not valid
    /// It returns an error if the response from the API is not valid or if the content of the response is not valid
    /// # Panics
    /// It panics if the API token is not provided
    /// # Remarks
    /// The context is updated with the response from the AI
    pub async fn completion(&mut self) -> Result<ChatResponse> {
        let response = self
            .client
            .post(URL)
            .bearer_auth(&self.openai_api_token)
            .header("Content-Type", "application/json")
            // Use Display trait to avoid sending None fields that the API would reject
            .body(self.chat_context.to_string())
            .send()
            .await
            .context(format!("Failed to receive the response from {}", URL))?
            .text()
            .await
            .context("Failed to retrieve the content of the response")?;

        let answer = parse_removing_newlines(response)?;
        Ok(answer)
    }

    /// Calls the OpenAI API to get a response using the current context, adding the content provided by the user
    /// This is the preferred function to use for chat completions that work with context.
    ///
    /// This is a fully managed function, it does update the context with the message provided,
    /// and it does update the context with the response from the AI.
    /// It calls completion_with_user_content_updating_context internally, it's for convenience.
    /// # Arguments
    /// * `content` - The content of the message
    /// # Errors
    /// It returns an error if the API token is not valid
    /// It returns an error if the response from the API is not valid or if the content of the response is not valid
    /// # Panics
    /// It panics if the API token is not provided
    /// # Remarks
    /// This is a fully managed function, it does update the context with the message provided,
    /// and it does update the context with the response from the AI.
    pub async fn completion_managed(&mut self, content: String) -> Result<ChatResponse> {
        self.completion_with_user_content_updating_context(content)
            .await
    }

    /// This function is used to call the openai API, using a Message already prepared.
    /// It requires a Message object as an argument, so access to some internal work of the library.
    /// This gives more flexibility to the user, but it is not recommended to use it directly.
    /// It returns the response from the AI
    /// It does update the context with the message provided,
    /// but it does not update the context with the response from the AI
    /// # Arguments
    /// * `message` - The message to send to the AI
    /// # Errors
    /// It returns an error if the API token is not valid
    /// It returns an error if the response from the API is not valid or if the content of the response is not valid
    /// # Remarks
    /// The context is updated with the message provided
    /// The context is not updated with the response from the AI
    /// This function is used by the other functions of the library
    /// It is not recommended to use it directly
    pub async fn completion_with_message(&mut self, message: Message) -> Result<ChatResponse> {
        self.push_message(message);
        self.completion().await
    }

    /// This function is used to call the openai API, using a String as the content of the message.
    /// It returns the response from the AI
    /// It does update the context with the message provided,
    /// but it does not update the context with the response from the AI
    /// # Arguments
    /// * `content` - The content of the message
    /// # Errors
    /// It returns an error if the API token is not valid
    /// It returns an error if the response from the API is not valid or if the content of the response is not valid
    /// # Remarks
    /// The context is updated with the message provided
    /// The context is not updated with the response from the AI
    /// This function is used by the other functions of the library
    /// It is not recommended to use it directly
    pub async fn completion_with_user_content(&mut self, content: String) -> Result<ChatResponse> {
        let message = Message::new_user_message(content);
        self.completion_with_message(message).await
    }

    /// This function is used to call the openai API, using content as the content of the message.
    /// It returns the response from the AI
    /// It does update the context with the message provided and the response from the AI
    /// # Arguments
    /// * `content` - The content of the message
    /// # Errors
    /// It returns an error if the API token is not valid
    /// It returns an error if the response from the API is not valid or if the content of the response is not valid
    /// # Remarks
    /// The context is updated with the message provided
    /// The context is updated with the response from the AI
    /// This function is used by the other functions of the library
    /// It assumes that there will only be one choice in the response
    /// It returns the response from the AI
    pub async fn completion_with_user_content_updating_context(
        &mut self,
        content: String,
    ) -> Result<ChatResponse> {
        let message = Message::new_user_message(content);
        self.completion_with_message_updating_context(message).await
    }

    /// This function is used to update the context with the response from the AI
    /// It assumes that there will only be one choice in the response
    /// It returns the response from the AI
    /// It does update the context with the response from the AI
    /// # Arguments
    /// * `message` - The message to send to the AI
    /// # Errors
    /// It returns an error if the API token is not valid
    /// It returns an error if the response from the API is not valid or if the content of the response is not valid
    /// # Remarks
    /// Important: The message received from the AI has to be modified when it is a function
    /// This is because when a function is returned the model still says that it is an assistant message.
    /// This is a bug in the API.
    /// If this is inserted in the context, the next request to the API will fail since it won't conform with the rules of the model.
    /// https://platform.openai.com/docs/api-reference/chat/create#chat/create-messages
    ///
    /// The context is updated with the response from the AI
    /// This function is used by the other functions of the library
    /// It assumes that there will only be one choice in the response
    /// It panics if there is more than one choice in the response
    pub async fn completion_with_message_updating_context(
        &mut self,
        message: Message,
    ) -> Result<ChatResponse> {
        self.push_message(message);
        let response = self.completion().await?;
        self.push_message(response.choices[0].message.clone());
        Ok(response)
    }

    /// This function is used to push a message to the context
    /// This is a low level function, it is not recommended to use it directly
    /// # Arguments
    /// * `message` - The message to push to the context
    /// # Remarks
    /// This function is used by the other functions of the library
    pub fn push_message(&mut self, message: Message) {
        self.chat_context.push_message(message);
    }

    /// This function is used to set all the messages in the context
    /// This will override the current messages in the context
    /// This is a low level function, it is not recommended to use it directly
    /// # Arguments
    /// * `messages` - The messages to set in the context
    /// # Remarks
    /// This function is used by the other functions of the library
    pub fn set_messages(&mut self, messages: Vec<Message>) {
        self.chat_context.set_messages(messages);
    }

    /// This function is used to push a function to the context
    /// This is a low level function, it is not recommended to use it directly
    /// # Arguments
    /// * `function` - The function to push to the context
    /// # Remarks
    /// This function is used by the other functions of the library
    pub fn push_function(&mut self, function: FunctionSpecification) {
        self.chat_context.push_function(function);
    }

    /// This function is used to set all the functions in the context
    /// This will override the current functions in the context
    /// This is a low level function, it is not recommended to use it directly
    /// # Arguments
    /// * `functions` - The vec of functions to set in the context
    /// # Remarks
    /// This function is used by the other functions of the library
    pub fn set_functions(&mut self, functions: Vec<FunctionSpecification>) {
        self.chat_context.set_functions(functions);
    }
}

fn parse_removing_newlines(response: String) -> Result<ChatResponse> {
    let r = response.replace("\n", "");
    let response: ChatResponse = serde_json::from_str(&r)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::function_specification::Parameters;

    use super::*;

    #[test]
    fn test_chat_gpt_new() {
        let chat_gpt = ChatGPT::new("key".to_string(), None, None, None).unwrap();
        assert_eq!(chat_gpt.session_id.len(), 36);
        assert_eq!(chat_gpt.chat_context.model, DEFAULT_MODEL);
        assert_eq!(chat_gpt.model, DEFAULT_MODEL);
    }

    #[test]
    fn test_chat_gpt_new_with_session_id() {
        let session_id = "session_id".to_string();
        let chat_gpt =
            ChatGPT::new("key".to_string(), None, None, Some(session_id.clone())).unwrap();
        assert_eq!(chat_gpt.session_id, session_id);
    }

    #[test]
    fn test_chat_gpt_new_with_chat_context() {
        let chat_context = ChatContext::new("model".to_string());
        let chat_gpt = ChatGPT::new("key".to_string(), None, Some(chat_context), None).unwrap();
        assert_eq!(chat_gpt.chat_context.model, "model");
    }

    #[test]
    fn test_chat_gpt_new_with_session_id_and_chat_context() {
        let session_id = "session_id".to_string();
        let chat_context = ChatContext::new("model".to_string());
        let chat_gpt = ChatGPT::new(
            "key".to_string(),
            None,
            Some(chat_context.clone()),
            Some(session_id.clone()),
        )
        .unwrap();
        assert_eq!(chat_gpt.session_id, session_id);
        assert_eq!(chat_gpt.chat_context.model, "model");
    }

    #[test]
    fn test_chat_gpt_push_message() {
        let mut chat_gpt = ChatGPT::new("key".to_string(), None, None, None).unwrap();
        let message = Message::new_user_message("content".to_string());
        chat_gpt.push_message(message);
        assert_eq!(chat_gpt.chat_context.messages.len(), 1);
    }

    #[test]
    fn test_chat_gpt_set_message() {
        let mut chat_gpt = ChatGPT::new("key".to_string(), None, None, None).unwrap();
        let message = Message::new_user_message("content".to_string());
        chat_gpt.set_messages(vec![message]);
        assert_eq!(chat_gpt.chat_context.messages.len(), 1);
    }

    #[test]
    fn test_chat_gpt_push_function() {
        let mut chat_gpt = ChatGPT::new("key".to_string(), None, None, None).unwrap();
        let function = FunctionSpecification::new("function".to_string(), None, None);
        chat_gpt.push_function(function);
        assert_eq!(chat_gpt.chat_context.functions.len(), 1);
    }

    #[test]
    fn test_chat_gpt_set_function() {
        let mut chat_gpt = ChatGPT::new("key".to_string(), None, None, None).unwrap();
        let function = FunctionSpecification::new(
            "function".to_string(),
            Some("Test function".to_string()),
            Some(Parameters {
                type_: "string".to_string(),
                properties: HashMap::new(),
                required: vec![],
            }),
        );
        chat_gpt.set_functions(vec![function]);
        assert_eq!(chat_gpt.chat_context.functions.len(), 1);

        let function = chat_gpt.chat_context.functions.get(0).unwrap();
        assert_eq!(function.name, "function");
        assert_eq!(function.description.as_ref().unwrap(), "Test function");
        assert_eq!(function.parameters.as_ref().unwrap().type_, "string");
    }

    #[test]
    fn test_parse_removing_newlines() {
        use crate::message::FunctionCall;

        let r = r#"{
    "id": "chatcmpl-7Ut7jsNlTUO9k9L5kBF0uDAyG19pK",
    "object": "chat.completion",
    "created": 1687596091,
    "model": "gpt-3.5-turbo-0613",
    "choices": [
        {
        "index": 0,
        "message": {
            "role": "assistant",
            "content": null,
            "function_call": {
                "name": "get_current_weather",
                "arguments": "{\n  \"location\": \"Madrid, Spain\"\n}"
            }
        },
        "finish_reason": "function_call"
        }
    ],
    "usage": {
        "prompt_tokens": 90,
        "completion_tokens": 19,
        "total_tokens": 109
    }
}"#
        .to_string();
        let response = parse_removing_newlines(r).unwrap();
        let message = response.choices.get(0).unwrap().message.clone();

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content, None);
        assert_eq!(message.name, None);
        assert_eq!(
            message.function_call,
            Some(FunctionCall {
                name: "get_current_weather".to_string(),
                arguments: "{\n  \"location\": \"Madrid, Spain\"\n}".to_string(),
            })
        );
    }

    #[test]
    fn test_fix_context_when_function_replied_with_content() {
        use crate::message::FunctionCall;

        let r = r#"{"id":"chatcmpl-7VneSVRn9qJ1crw3m0V0kmnCq8Pnn","object":"chat.completion","created":1687813384,"choices":[{"index":0,"message":{"role":"assistant","function_call":{"name":"completion_managed","arguments":"{
    \"content\": \"Hi, model!\"
}"}},"finish_reason":"function_call"}],"usage":{"prompt_tokens":61,"completion_tokens":18,"total_tokens":79}}"#.to_string();
        let response = parse_removing_newlines(r).unwrap();
        let message = response.choices.get(0).unwrap().message.clone();

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content, None);
        assert_eq!(message.name, None);
        assert_eq!(
            message.function_call,
            Some(FunctionCall {
                name: "completion_managed".to_string(),
                arguments: "{    \"content\": \"Hi, model!\"}".to_string(),
            })
        );
    }

    // Skip this test because (for now) it requires an API key and a real connection to the API
    // #[tokio::test]
    // async fn test_chat_gpt_completion() {
    //     let mut chat_gpt = ChatGPT::new("key".to_string(), None, None).unwrap();
    //     let message = Message::new_user_message("content".to_string());
    //     chat_gpt.push_message(message);
    //     let response = chat_gpt.completion().await.unwrap();
    //     assert_eq!(response.choices.len(), 1);
    // }
}
