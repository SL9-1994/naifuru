/// This module defines custom error types and utilities for handling errors in the application.
use std::path::PathBuf;
use thiserror::Error;

#[macro_export]
macro_rules! bail_on_error {
    ($exit_code:expr) => {{
        std::process::exit($exit_code);
    }};
}

// トップレベルカスタムエラー
#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AppError {
    #[error("CLI> {0}")]
    Cli(#[from] CliErr),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Cli(e) => e.exit_code(),
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum CliErr {
    #[error("Args validation error> {0}")]
    Validation(#[from] ArgsValidationErr),
}

impl CliErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => 2,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ArgsValidationErr {
    #[error("Couldn’t find a file extension for: '{0}'")]
    NoExtension(PathBuf),

    #[error("Unsupported file extension: '{0}'. Supported types are: {1}")]
    InvalidExtension(String, String),

    #[error("Oops! The specified path doesn't exist: '{0}'")]
    PathDoesNotExist(PathBuf),

    #[error("The given path is not a file: '{0}'")]
    PathIsNotFile(PathBuf),

    #[error("The given path is not a directory: '{0}'")]
    PathIsNotDirectory(PathBuf),
}
