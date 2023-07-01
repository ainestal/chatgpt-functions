/// Escape a string for JSON.
///
/// This is useful to escape the content of a message, for example.
/// Withouht escaping, the JSON would be invalid and the API would return an error.
pub trait EscapeJson {
    fn escape_json(&self) -> String;
}

impl EscapeJson for str {
    fn escape_json(&self) -> String {
        let mut escaped_string = String::new();
        for c in self.chars() {
            match c {
                '\\' => escaped_string.push_str("\\\\"),
                '\"' => escaped_string.push_str("\\\""),
                '\n' => escaped_string.push_str("\\n"),
                '\r' => escaped_string.push_str("\\r"),
                '\t' => escaped_string.push_str("\\t"),
                '\x08' => escaped_string.push_str("\\b"),
                '\x0C' => escaped_string.push_str("\\f"),
                _ => escaped_string.push(c),
            }
        }
        escaped_string
    }
}

impl EscapeJson for String {
    fn escape_json(&self) -> String {
        self.as_str().escape_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_json() {
        assert_eq!("\"".escape_json(), "\\\"");
        assert_eq!("\\".escape_json(), "\\\\");
        assert_eq!("\n".escape_json(), "\\n");
        assert_eq!("\r".escape_json(), "\\r");
        assert_eq!("\t".escape_json(), "\\t");
        assert_eq!("\x08".escape_json(), "\\b");
        assert_eq!("\x0C".escape_json(), "\\f");
        assert_eq!(
            "\"\\n\\r\\t\x08\x0C".escape_json(),
            "\\\"\\\\n\\\\r\\\\t\\b\\f"
        );
    }

    #[test]
    fn test_escape_json_string() {
        assert_eq!("\"".to_string().escape_json(), "\\\"");
        assert_eq!("\\".to_string().escape_json(), "\\\\");
        assert_eq!("\n".to_string().escape_json(), "\\n");
        assert_eq!("\r".to_string().escape_json(), "\\r");
        assert_eq!("\t".to_string().escape_json(), "\\t");
        assert_eq!("\x08".to_string().escape_json(), "\\b");
        assert_eq!("\x0C".to_string().escape_json(), "\\f");
        assert_eq!(
            "\"\\n\\r\\t\x08\x0C".to_string().escape_json(),
            "\\\"\\\\n\\\\r\\\\t\\b\\f"
        );
    }

    #[test]
    fn test_escape_json_string_with_quotes() {
        assert_eq!("\"\"\"".to_string().escape_json(), "\\\"\\\"\\\"");
        assert_eq!(
            "\"\"\"\"\"".to_string().escape_json(),
            "\\\"\\\"\\\"\\\"\\\""
        );
    }

    #[test]
    fn test_escape_json_string_with_text() {
        let text = "text with \"quotes\" and a \nnewline";
        assert_eq!(
            text.to_string().escape_json(),
            "text with \\\"quotes\\\" and a \\nnewline"
        );
    }
}
