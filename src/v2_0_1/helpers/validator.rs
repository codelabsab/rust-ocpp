use regex::Regex;
use validator::ValidationError;

/// Helper function to validate identifierString
///
/// # identfierString
/// This is a case-insensitive dataType and can only contain characters from the following
/// character set: `a-z`, `A-Z`, `0-9`, `'*'`, `'-'`, `'_'`, `'='`, `':'`, `'+'`, `'|'`, `'@'`, `'.'`
pub fn validate_identifier_string(s: &str) -> Result<(), ValidationError> {
    // regex for identifierString as defined by the specification
    let re = Regex::new(r"[a-z]|[A-Z]|[0-9]|[+]|[*]|[-]|[=]|[:]|[0]|[|]|[@]|[.]").unwrap();

    let res = re.is_match(&s);

    match res {
        true => return Ok(()),
        false => return Err(ValidationError::new("Not a valid identifierString")),
    }
}
