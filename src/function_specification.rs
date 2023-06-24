use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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

impl fmt::Display for FunctionSpecification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"name\":\"{}\",\"description\":\"{}\",\"parameters\":{}}}",
            self.name, self.description, self.parameters
        )
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
            description: "Get the current weather in a given location".to_string(),
            parameters,
        };
        assert_eq!(
            function_specification.to_string(),
            "{\"name\":\"get_current_weather\",\"description\":\"Get the current weather in a given location\",\"parameters\":{\"type\":\"object\",\"properties\":{\"unit\":{\"type\":\"string\",\"enum\":[\"celsius\",\"fahrenheit\"]}},\"required\":[\"unit\"]}}"
        );
    }
}
