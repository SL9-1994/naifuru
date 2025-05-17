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
#[derive(Error, Debug)]
pub enum AppError {
    #[error("CLI> {0}")]
    Cli(#[from] CliErr),
    #[error("AnalysisConfigFile> {0}")]
    AnalysisConfig(#[from] AnalysisConfigErr),
    #[error("Processor> {0}")]
    Process(#[from] ProcessErr),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Cli(e) => e.exit_code(),
            Self::AnalysisConfig(e) => e.exit_code(),
            Self::Process(e) => e.exit_code(),
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum CliErr {
    #[error("validation> {0}")]
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
#[derive(Error, Debug)]
pub enum ArgsValidationErr {
    #[error("Couldn't find a file extension for: '{0}'")]
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

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum AnalysisConfigErr {
    #[error("validation> {0}")]
    Validation(#[from] ConfigValidationErr),
    #[error("parse> {0}")]
    Parse(#[from] toml::de::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl AnalysisConfigErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => 3,
            Self::Parse(_) => 5,
            Self::Io(_) => 4,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ConfigValidationErr {
    #[error("Unsupported file extension: '{0}'. Expected one of: {1}")]
    InvalidExtension(String, String),

    #[error("Couldn't find a file extension for: '{0}'")]
    NoExtension(PathBuf),

    #[error("Oops! The specified path doesn't exist: '{0}'")]
    PathDoesNotExist(PathBuf),

    #[error("Hmm... this path isn't a file: '{0}'")]
    PathIsNotFile(PathBuf),

    #[error("Hmm... this path isn't a directory: '{0}'")]
    PathIsNotDirectory(PathBuf),

    #[error("The format '{0}' doesn't expect 'acc_axis', but it was set (name: '{1}', id: {2})")]
    MismatchedAccAxis(String, String, usize),

    #[error(
        "The format '{0}' needs all three axes: 'ns', 'ew', and 'ud'. Please check (name: '{1}', id: {2})"
    )]
    DuplicateAccAxis(String, String, usize),

    #[error("Missing 'acc_axis' information (name: '{0}', id: {1})")]
    RequiredAccAxis(String, usize),

    #[error("Duplicate conversion names found. Each name must be unique: {0}")]
    DuplicateNames(String),
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ProcessErr {
    #[error("Data extraction> {0}")]
    Extraction(#[from] DataExtractionErr),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ProcessErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Extraction(_) => 6,
            Self::Io(_) => 4,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum DataExtractionErr {
    #[error("Oops! Failed to determine endianness: {0}")]
    EndianDetectionFailed(PathBuf),

    // 将来的に対応する予定のフォーマット
    #[error("'{0}' format is not supported. (It will be supported in the future)")]
    FormatUnsupported(String),

    #[error("Oops! '{0}' is missing: {1}")]
    MissingFileData(String, PathBuf),

    #[error("Oops! Failed to extract '{0}': {1})")]
    FailedExtraction(String, PathBuf),

    #[error("The extraction pattern of '{0}' was not matched, the data format is invalid: {1}")]
    RegexNotMatched(String, PathBuf),
}
