use crate::analyzer::generic_analyzer::YmlAnalyzer;
use std::fmt;
use std::path::Path;

pub enum ValidateSucces {
    Success,
    Unchanged,
}

pub enum SlideValidateError {
    ImageDoesntExists(String),
}

pub enum PresValidateError {
    ImageDoesntExists(String),
}
