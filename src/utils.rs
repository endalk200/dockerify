use regex::Regex;

pub fn extract_placeholders(input: &str) -> Vec<String> {
    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();

    re.captures_iter(input)
        .map(|cap: regex::Captures| cap[1].to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_placeholders() {
        let input = "This is text with {{ name }} placeholder.";
        let expected: Vec<String> = vec!["name".to_string()];
        assert_eq!(
            extract_placeholders(input),
            expected,
            "Testing single placeholder"
        );

        let input = "This is text with {{ name }} placeholder being replaced with {{ value }} and {{ another_name }}";
        let expected: Vec<String> = vec![
            "name".to_string(),
            "value".to_string(),
            "another_name".to_string(),
        ];
        assert_eq!(
            extract_placeholders(input),
            expected,
            "Testing multiple placeholders"
        );
    }
}
