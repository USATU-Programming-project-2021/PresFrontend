use super::validator::{SlideValidateError, ValidateSucces};
use crate::analyzer::generic_analyzer::YmlAnalyzer;
use crate::analyzer::slide::SlideYml;
use std::fmt;
use std::path::Path;

impl fmt::Display for SlideValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlideValidateError::ImageDoesntExists(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

pub fn validate_images(
    slide_images: &Option<Vec<String>>,
) -> Result<ValidateSucces, SlideValidateError> {
    let images: &Vec<String> = match &slide_images {
        Some(image) => image,
        None => return Ok(ValidateSucces::Unchanged),
    };

    for image in images {
        if !Path::new(image).is_file() {
            return Err(SlideValidateError::ImageDoesntExists(format!(
                "File '{}' doesn't exists",
                image
            )));
        };
    }

    Ok(ValidateSucces::Success)
}

fn is_valid(yml_object: &SlideYml) -> Result<ValidateSucces, SlideValidateError> {
    validate_images(&yml_object.slide.images)?;
    Ok(ValidateSucces::Success)
}
