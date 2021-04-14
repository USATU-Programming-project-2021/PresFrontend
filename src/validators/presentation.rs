use super::validator::{PresValidateError, ValidateSucces};
use crate::analyzer::presentation::PresYml;
use std::fmt;

impl fmt::Display for PresValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PresValidateError::ImageDoesntExists(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

pub fn is_valid(pres_yml: &PresYml) -> Result<ValidateSucces, PresValidateError> {
    Ok(ValidateSucces::Success)
}
