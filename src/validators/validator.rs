use std::path::Path;
use crate::analyzer::generic_analyzer::YmlAnalyzer;
use std::fmt;

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

