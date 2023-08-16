use validator::ValidationError;

/// Sorted list of characters allowed in `identifierString`.
pub const ALLOWED_CHARACTERS: [char; 71] = [
    '*', '+', '-', '.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', '=', '@', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '|',
];

/// Helper function to validate identifierString
///
/// # identfierString
/// This is a case-insensitive dataType and can only contain characters from the following
/// character set: `a-z`, `A-Z`, `0-9`, `'*'`, `'-'`, `'_'`, `'='`, `':'`, `'+'`, `'|'`, `'@'`, `'.'`
pub fn validate_identifier_string(s: &str) -> Result<(), ValidationError> {
    for c in s.chars() {
        if !ALLOWED_CHARACTERS.contains(&c) {
            return Err(ValidationError::new("Not a valid identifierString"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::validate_identifier_string;

    #[test]
    fn good_case() {
        let good_cases = ["abc123", "A*C_|..", "||||", "ABCabc123:==@"];

        for case in good_cases.iter() {
            dbg!(case);
            validate_identifier_string(case).unwrap();
        }
    }

    #[test]
    fn bad_case() {
        let bad_cases = [
            "abc123/",
            "https://",
            "ABC#123",
            ",,,,",
            "Test test",
            "123 Prøve",
            "123 Test?",
        ];

        for case in bad_cases.iter() {
            dbg!(case);
            validate_identifier_string(case).unwrap_err();
        }
    }
}
