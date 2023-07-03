use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// The documentation for a function
///
/// # Caveats
/// The documentation, in July 2023 is not accurate
/// https://platform.openai.com/docs/api-reference/chat/create#chat/create-parameters
///
/// It states that the parameters are optional, but they are not:
///
/// curl https://api.openai.com/v1/chat/completions   -H "Content-Type: application/json"   -H "Authorization: Bearer $OPENAI_API_KEY"   -d '{
///     "model": "gpt-3.5-turbo-0613",
///     "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is the weather like in Madrid, Spain?"}],
///     "functions": [{
///         "name": "get_current_weather",
///         "description": "Get the current weather in a given location"
///     }],
///     "function_call": "auto"
/// }'
/// {
///   "error": {
///     "message": "'parameters' is a required property - 'functions.0'",
///     "type": "invalid_request_error",
///     "param": null,
///     "code": null
///   }
/// }
///
/// The library works around it by actually having the parameters as optional in the struct,
/// so the configuration can be parsed correctly, but then printing the object with the parameters
/// and the minimum required fields so the API doesn't complain. This would be by adding the
/// parameteres, with type and empty properties. Like this:
///
/// curl https://api.openai.com/v1/chat/completions   -H "Content-Type: application/json"   -H "Authorization: Bearer $OPENAI_API_KEY"   -d '{
///     "model": "gpt-3.5-turbo-0613",
///     "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is the weather like in Madrid, Spain?"}],
///     "functions": [{
///         "name": "get_current_weather",
///         "description": "Get the current weather in a given location",
///         "parameters": {
///             "type": "object",
///             "properties": {}
///         }
///     }]
/// }'
///
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionSpecification {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Parameters>,
}

// Struct to deserialize parameters using serde
// the type_ field is named type because type is a reserved keyword in Rust
// the anotation will help serde to deserialize the field correctly
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Property {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
}

impl FunctionSpecification {
    pub fn new(
        name: String,
        description: Option<String>,
        parameters: Option<Parameters>,
    ) -> FunctionSpecification {
        FunctionSpecification {
            name,
            description,
            parameters,
        }
    }
}

// ------------------------------------------------------------------------------
// Display functions
// ------------------------------------------------------------------------------

// Print valid JSON for FunctionSpecification, no commas if last field, no field if None
impl fmt::Display for FunctionSpecification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"name\":\"{}\"", self.name)?;
        if let Some(description) = &self.description {
            write!(f, ",\"description\":\"{}\"", description)?;
        }
        if let Some(parameters) = &self.parameters {
            write!(f, ",\"parameters\":{}", parameters)?;
        } else {
            write!(
                f,
                ",\"parameters\":{{\"type\":\"object\",\"properties\":{{}}}}"
            )?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"type\":\"{}\"", self.type_)?;
        if !self.properties.is_empty() {
            write!(f, ",\"properties\":{{")?;
            for (i, (key, value)) in self.properties.iter().enumerate() {
                write!(f, "\"{}\":{}", key, value)?;
                if i < self.properties.len() - 1 {
                    write!(f, ",")?;
                }
            }
            write!(f, "}}")?;
        }
        if !self.required.is_empty() {
            write!(f, ",\"required\":[")?;
            for (i, required) in self.required.iter().enumerate() {
                write!(f, "\"{}\"", required)?;
                if i < self.required.len() - 1 {
                    write!(f, ",")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"type\":\"{}\"", self.type_)?;
        if let Some(description) = &self.description {
            write!(f, ",\"description\":\"{}\"", description)?;
        }
        if let Some(enum_) = &self.enum_ {
            write!(f, ",\"enum\":[")?;
            for (i, enum_value) in enum_.iter().enumerate() {
                write!(f, "\"{}\"", enum_value)?;
                if i < enum_.len() - 1 {
                    write!(f, ",")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "}}")
    }
}

// ------------------------------------------------------------------------------
// Tests
// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_specification_new() {
        let name = "get_current_weather".to_string();
        let description = "Get the current weather in a given location".to_string();
        let parameters = Parameters {
            type_: "object".to_string(),
            properties: HashMap::new(),
            required: vec![],
        };
        let function_specification = FunctionSpecification::new(
            name.clone(),
            Some(description.clone()),
            Some(parameters.clone()),
        );
        assert_eq!(function_specification.name, name);
        assert_eq!(function_specification.description, Some(description));
        assert_eq!(function_specification.parameters, Some(parameters));
    }

    #[test]
    fn test_deserialize_function_specification() {
        let json = r#"
        {
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    }
                },
                "required": ["location"]
            }
        }
        "#;
        let function_specification: FunctionSpecification = serde_json::from_str(json)
            .expect("Could not parse correctly the function specification");
        assert_eq!(function_specification.name, "get_current_weather");
        assert_eq!(
            function_specification.description,
            Some("Get the current weather in a given location".to_string())
        );
        let params = function_specification.parameters.expect("No parameters");
        assert_eq!(params.type_, "object");
        assert_eq!(params.properties.len(), 2);
        assert_eq!(params.required.len(), 1);

        let location = params
            .properties
            .get("location")
            .expect("Could not find location property");
        assert_eq!(location.type_, "string");
        assert_eq!(
            location.description,
            Some("The city and state, e.g. San Francisco, CA".to_string())
        );

        let unit = params
            .properties
            .get("unit")
            .expect("Could not find unit property");
        assert_eq!(unit.type_, "string");
        assert_eq!(unit.description, None);
        assert_eq!(
            unit.enum_,
            Some(vec!["celsius".to_string(), "fahrenheit".to_string()])
        );
    }

    #[test]
    fn test_display_no_parameters() {
        let function_specification = FunctionSpecification::new(
            "get_current_weather".to_string(),
            Some("Get the current weather in a given location".to_string()),
            None,
        );
        assert_eq!(
            function_specification.to_string(),
            "{\"name\":\"get_current_weather\",\"description\":\"Get the current weather in a given location\",\"parameters\":{\"type\":\"object\",\"properties\":{}}}"
        );
    }

    #[test]
    fn test_display_parameters_with_properties() {
        let mut properties = HashMap::new();
        properties.insert(
            "unit".to_string(),
            Property {
                type_: "string".to_string(),
                description: None,
                enum_: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
            },
        );
        let parameters = Parameters {
            type_: "object".to_string(),
            properties,
            required: vec!["unit".to_string()],
        };
        assert_eq!(
            parameters.to_string(),
            "{\"type\":\"object\",\"properties\":{\"unit\":{\"type\":\"string\",\"enum\":[\"celsius\",\"fahrenheit\"]}},\"required\":[\"unit\"]}"
        );
    }

    #[test]
    fn test_display_parameters_without_properties() {
        let parameters = Parameters {
            type_: "object".to_string(),
            properties: HashMap::new(),
            required: vec!["location".to_string()],
        };
        assert_eq!(
            parameters.to_string(),
            "{\"type\":\"object\",\"required\":[\"location\"]}"
        );
    }

    #[test]
    fn test_display_property_with_description_and_enum() {
        let property = Property {
            type_: "string".to_string(),
            description: Some("The city and state, e.g. San Francisco, CA".to_string()),
            enum_: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
        };
        assert_eq!(
            property.to_string(),
            "{\"type\":\"string\",\"description\":\"The city and state, e.g. San Francisco, CA\",\"enum\":[\"celsius\",\"fahrenheit\"]}"
        );
    }

    #[test]
    fn test_display_property_with_description() {
        let property = Property {
            type_: "string".to_string(),
            description: Some("The city and state, e.g. San Francisco, CA".to_string()),
            enum_: None,
        };
        assert_eq!(
            property.to_string(),
            "{\"type\":\"string\",\"description\":\"The city and state, e.g. San Francisco, CA\"}"
        );
    }

    #[test]
    fn test_display_property_with_enum() {
        let property = Property {
            type_: "string".to_string(),
            description: None,
            enum_: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
        };
        assert_eq!(
            property.to_string(),
            "{\"type\":\"string\",\"enum\":[\"celsius\",\"fahrenheit\"]}"
        );
    }

    #[test]
    fn test_display_function_specification() {
        let mut properties = HashMap::new();
        properties.insert(
            "unit".to_string(),
            Property {
                type_: "string".to_string(),
                description: None,
                enum_: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
            },
        );
        let parameters = Parameters {
            type_: "object".to_string(),
            properties,
            required: vec!["unit".to_string()],
        };
        let function_specification = FunctionSpecification {
            name: "get_current_weather".to_string(),
            description: Some("Get the current weather in a given location".to_string()),
            parameters: Some(parameters),
        };
        assert_eq!(
            function_specification.to_string(),
            "{\"name\":\"get_current_weather\",\"description\":\"Get the current weather in a given location\",\"parameters\":{\"type\":\"object\",\"properties\":{\"unit\":{\"type\":\"string\",\"enum\":[\"celsius\",\"fahrenheit\"]}},\"required\":[\"unit\"]}}"
        );
    }
}
