use log::{debug, error};
use naifuru::{bail_on_error, cli::Args, error::AppError, logging::init_logger};

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

    Ok(())
}
