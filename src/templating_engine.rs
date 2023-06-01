use std::{collections::HashMap, fs};

use crate::utils;

pub fn insert(input: &str, values: &HashMap<&str, &str>) -> String {
    let placeholders: Vec<String> = utils::extract_placeholders(input);

    for placeholder in &placeholders {
        if !values.contains_key(placeholder.as_str()) {
            panic!("Placeholder {} is not provided", placeholder);
        }
    }

    let mut result: String = String::new();
    let mut chars: std::iter::Peekable<std::str::Chars> = input.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '{' && chars.peek() == Some(&'{') {
            chars.next();

            let mut key: String = String::new();

            while let Some(character) = chars.next() {
                if character == '}' && chars.peek() == Some(&'}') {
                    chars.next();
                    break;
                } else {
                    key.push(character);
                }
            }

            if let Some(value) = values.get(key.trim()) {
                result.push_str(value);
            }
        } else {
            result.push(character);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_func() {
        assert_eq!(
            insert("Hello {{ name }}", &HashMap::from([("name", "World")])),
            "Hello World",
            "Test if the function will insert the correct value for one placeholder"
        );

        assert_eq!(
            insert(
                "Hello {{ name }}.\nMy name is {{ my_name }}",
                &HashMap::from([("name", "World"), ("my_name", "John")])
            ),
            "Hello World.\nMy name is John",
            "Test if the function will insert the correct value for multiple placeholders"
        );
    }

    #[test]
    #[should_panic(expected = "Placeholder name is not provided")]
    fn test_insert_panics() {
        insert("Hello {{ name }}", &HashMap::new());
    }
}

pub fn get_template_content(template_name: &str) -> String {
    let nestjs_dockerfile: String =
        include_str!("templates/nestjs/Dockerfile.__template__").to_string();

    let nextjs_dockerfile: String =
        include_str!("templates/nextjs/Dockerfile.__template__").to_string();

    if template_name == "nestjs" {
        return nestjs_dockerfile;
    } else if template_name == "nextjs" {
        return nextjs_dockerfile;
    } else {
        panic!("Template {} is not supported", template_name);
    }
}
