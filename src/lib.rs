
pub fn ignore_parens(stream: &str) -> String {
    if stream.contains("(") || stream.contains(")") {
        let result = stream.replace("(", "").replace(")", "");
        result
    } else {
        String::from(stream)
    }
}

pub fn ignore_braces(stream: &str) -> String {
    if stream.contains("{") || stream.contains("}") {
        let result = stream.replace("(", "").replace(")", "");
        result
    } else {
        String::from(stream)
    }
}

pub fn ignore_brackets(stream: &str) -> String {
    if stream.contains("[") || stream.contains("]") {
        let result = stream.replace("(", "").replace(")", "");
        result
    } else {
        String::from(stream)
    }
}

/// Use for statements wrapped in a character, like "string literals"
/// Returns the provided &str with every instance of the char removed.
pub fn ignore_wrapper(wrapper: char, stream: &str) -> String {
    if stream.contains(wrapper) {
        let result = stream.replace(wrapper, "");
        result
    } else {
        String::from(stream)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parens() {
        let test_string = String::from("(hello)");
        assert_eq!(ignore_parens(&test_string), "hello");
    }
}
