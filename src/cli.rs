use std::path::{Path, PathBuf};

use clap::{Parser, ValueHint};

use crate::{
    error::{AppError, ArgsValidationErr, CliErr},
    logging::LogLevel,
};

const ACCEPTABLE_EXTS: [&str; 1] = ["toml"];

/// This module defines the command-line interface (CLI) for the application using the `clap` crate.
/// It includes the `Args` struct which represents the parsed command-line arguments and provides
/// methods for validation of these arguments.
///
/// # Structs
///
/// - `Args`: Represents the command-line arguments and provides methods for parsing and validation.
///
/// # Methods
///
/// - `Args::new() -> Self`: Parses the command-line arguments and returns an instance of `Args`.
/// - `Args::validate(&self) -> Result<(), Vec<AppError>>`: Validate the path of the input file and the path of the output directory, which is the entry point for the validation check.
/// - `Args::validate_input_file_path(&self, path: &Path) -> Result<(), CliError>`: Validates the input file path ensuring it exists and has a valid extension.
/// - `Args::validate_output_dir_path(&self, path: &Path) -> Result<(), CliError>`: Validates that the output directory path exists.
///
/// # Errors
///
/// - `ErrorContext`: Struct representing possible validation errors including IO errors, invalid file extensions, and path type mismatches.
#[derive(Debug, PartialEq, Eq, Clone, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the file describing the file to be converted.
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub input_file_path: PathBuf,

    /// Path of the output directory of the converted file.
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub output_dir_path: PathBuf,

    /// Sets the logging level
    #[clap(short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn validate(&self) -> Result<(), Vec<AppError>> {
        let validations = vec![
            self.validate_input_file_path(&self.input_file_path),
            self.validate_output_dir_path(&self.output_dir_path),
        ];

        let (_, errs): (Vec<_>, Vec<_>) = validations.into_iter().partition(Result::is_ok);

        if errs.is_empty() {
            Ok(())
        } else {
            let all_errors: Vec<AppError> =
                errs.into_iter().map(|e| e.unwrap_err().into()).collect();

            Err(all_errors)
        }
    }

    fn validate_input_file_path(&self, path: &Path) -> Result<(), CliErr> {
        let err: CliErr;

        if !path.exists() {
            err = ArgsValidationErr::PathDoesNotExist(path.to_path_buf()).into();
            return Err(err);
        }

        if !path.is_file() {
            err = ArgsValidationErr::PathIsNotFile(path.to_path_buf()).into();
            return Err(err);
        }

        match path.extension().map(|e| e.to_string_lossy().to_lowercase()) {
            Some(ext) if !ACCEPTABLE_EXTS.contains(&ext.as_str()) => {
                err = ArgsValidationErr::InvalidExtension(ext, ACCEPTABLE_EXTS.join(", ")).into();
                return Err(err);
            }
            None => {
                err = ArgsValidationErr::NoExtension(path.to_path_buf()).into();
                return Err(err);
            }
            _ => {}
        }

        Ok(())
    }

    fn validate_output_dir_path(&self, path: &Path) -> Result<(), CliErr> {
        let err: CliErr;
        if !path.exists() {
            err = ArgsValidationErr::PathDoesNotExist(path.to_path_buf()).into();
            return Err(err);
        }

        if !path.is_dir() {
            err = ArgsValidationErr::PathIsNotDirectory(path.to_path_buf()).into();
            return Err(err);
        }

        Ok(())
    }
}
