use std::path::Path;

use log::{debug, error};
use naifuru::{
    analysis_config::{Config, TextOrBinary},
    bail_on_error,
    cli::Args,
    error::AppError,
    extractor::extractor::create_extractor,
    logging::init_logger,
    util::{read_binary, read_text, read_text_as_lines},
};

const DEFAULT_ERROR_EXIT_CODE: i32 = 1;

fn main() {
    if let Err(errors) = run() {
        for error in &errors {
            error!("{}", error);
        }

        // 最初のエラーからexit_codeを決定、また、exit_codeを取得できない場合はDEFAULT_ERROR_EXIT_CODEで終了します。
        let exit_code = errors
            .first()
            .map_or(DEFAULT_ERROR_EXIT_CODE, |e| e.exit_code());

        bail_on_error!(exit_code);
    }
}

fn run() -> Result<(), Vec<AppError>> {
    let args = Args::new();

    init_logger(args.log_level.into()).unwrap(); // デフォルト値の使用やチェックが行われるため，処理失敗は起こりません
    debug!("The logging level has been set successfully.");

    args.validate()?;
    debug!("The CLI args have been validated successfully.");

    let config_toml_str =
        read_text(&args.input_file_path).map_err(|e| vec![AppError::AnalysisConfig(e.into())])?;
    debug!("The analysis configuration file has been loaded successfully.");

    let mut config: Config =
        toml::from_str(&config_toml_str).map_err(|e| vec![AppError::AnalysisConfig(e.into())])?;
    debug!("The analysis configuration file has been parsed successfully.");

    config.validate()?;
    debug!("The analysis configuration file has been validated successfully.");

    // MEMO: グループごとに処理
    for conv_config in &mut config.conversion {
        for group in &mut conv_config.group {
            for file in &mut group.files {
                let file_content: TextOrBinary = match read_text_as_lines(Path::new(&file.path)) {
                    Ok(content) => content,
                    Err(_) => {
                        let binary = read_binary(Path::new(&file.path))
                            .map_err(|e| vec![AppError::Process(e.into())])?;
                        binary
                    }
                };
                file.data = Some(file_content);
            }
        }
    }

    for conv in &config.conversion {
        for file in conv.iter_processable_files() {
            let extractor = create_extractor(file);
            let _extracted = extractor.extract().map_err(|e| vec![e])?; // TODO: エラー時にそのファイルだけスキップする処理
        }
    }
    Ok(())
}
