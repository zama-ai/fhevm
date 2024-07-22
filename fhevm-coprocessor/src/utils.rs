use lazy_static::lazy_static;
use crate::types::CoprocessorError;

pub fn check_valid_ciphertext_handle(inp: &str) -> Result<(), CoprocessorError> {
    lazy_static! {
        static ref VALID_HANDLE_REGEX: regex::Regex = regex::Regex::new("^[a-zA-Z0-9]+$").unwrap();
    }

    if inp.len() > 64 {
        return Err(CoprocessorError::CiphertextHandleLongerThan64Bytes);
    }

    if !VALID_HANDLE_REGEX.is_match(inp) {
        return Err(CoprocessorError::InvalidHandle(inp.to_string()));
    }

    Ok(())
}


pub fn db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined")
}