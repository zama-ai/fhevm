use lazy_static::lazy_static;
use crate::types::CoprocessorError;

// handle must be serializable to bytes for scalar operations
pub fn check_valid_ciphertext_handle(inp: &str) -> Result<(), CoprocessorError> {
    lazy_static! {
        static ref VALID_HANDLE_REGEX: regex::Regex = regex::Regex::new("^0x[0-9a-f]+$").unwrap();
    }

    // 66 including 0x in front
    if inp.len() > 66 {
        return Err(CoprocessorError::CiphertextHandleLongerThan64Bytes);
    }

    // at least one hex nibble
    if inp.len() < 4 {
        return Err(CoprocessorError::CiphertextHandleMustBeAtLeast4Bytes(inp.to_string()));
    }

    if inp.len() % 2 != 0 {
        return Err(CoprocessorError::CiphertextHandleMustHaveEvenAmountOfHexNibblets(inp.to_string()));
    }

    if !VALID_HANDLE_REGEX.is_match(inp) {
        return Err(CoprocessorError::InvalidHandle(inp.to_string()));
    }

    Ok(())
}


pub fn db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined")
}