use error::PResult;
mod error;

pub fn ignore_parens(stream: &str) -> String {
    let result = stream.replace("(", "").replace(")", "");
    result
}

pub fn ignore_braces(stream: &str) -> String {
    let result = stream.replace("(", "").replace(")", "");
    result
}

pub fn ignore_brackets(stream: &str) -> String {
    let result = stream.replace("(", "").replace(")", "");
    result
}

/// Use for statements wrapped in a character, like "string literals"
/// Returns the provided &str with every instance of the char removed.
pub fn ignore_wrapper(wrapper: char, stream: &str) -> String {
    let result = stream.replace(wrapper, "");
    result
}

pub fn ignore_ws(stream: &str) -> String {
    let result = stream.replace(" ", "");
    result
}

pub fn take_bytes(bytes: usize, stream: &str) -> PResult<String, &str> {
    let mut buf = String::new();

    // Check that the stream is as long as the amount of bytes taking
    if !stream.len() <= bytes {
        return PResult::Err("attempted to take more bytes than there are in stream");
    }
    for b in stream.as_bytes() {
        if buf.len() < bytes {
            buf.push(*b as char);
        } else {
            return PResult::Ok(buf);
        }
    }
    return PResult::Ok(buf);
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
