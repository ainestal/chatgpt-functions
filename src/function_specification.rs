use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionSpecification {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

// Struct to deserialize parameters using serde
// the type_ field is named type because type is a reserved keyword in Rust
// the anotation will help serde to deserialize the field correctly
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "Get the current weather in a given location"
        );
        assert_eq!(function_specification.parameters.type_, "object");
        assert_eq!(function_specification.parameters.properties.len(), 2);
        assert_eq!(function_specification.parameters.required.len(), 1);

        let location = function_specification
            .parameters
            .properties
            .get("location")
            .expect("Could not find location property");
        assert_eq!(location.type_, "string");
        assert_eq!(
            location.description,
            Some("The city and state, e.g. San Francisco, CA".to_string())
        );

        let unit = function_specification
            .parameters
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
}
