mod cli;
mod config;

#[derive(Debug)]
enum AppError {
    CliError(cli::CliError),
    ConfigLoadError(config::ConfigLoadError),
}

fn main() -> Result<(), AppError> {
    config::config_load()
        .map_err(AppError::ConfigLoadError)
        .and_then(|c| {
            cli::cli_validate(c)
                .map_err(AppError::CliError)
        })
        .map(|_| ())
}
