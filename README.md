# chatgpt-functions

Lib to interact with gpt in chat mode, using functions

## Example

```bash
curl https://api.openai.com/v1/chat/completions   -H "Content-Type: application/json"   -H "Authorization: Bearer $OPENAI_API_KEY"   -d '{
    "model": "gpt-3.5-turbo-0613",
    "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is the weather like in Madrid, Spain?"}],
    "functions": [{
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]}
                },
                "required": ["location"]
            }
        }],
    "function_call": "auto"
}'



{
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
}
```
