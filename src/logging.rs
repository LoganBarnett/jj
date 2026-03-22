use crate::error::{AppError};
use clap_verbosity_flag::Verbosity;
use log::*;

pub fn init_logger(verbosity: &Verbosity) -> Result<(), AppError> {
    let mut logger = stderrlog::new();
    logger
        // module_path doesn't work here. Nothing is logged using it.
        // .module("jj")
        // .module(module_path!())
        // .modules(vec!("cli", "jenkins", "jj", "main"))
        .verbosity(
            verbosity
                .log_level()
                .ok_or(
                    AppError::LoggingLogLevelNotFoundError(verbosity.clone()),
                )?,
        )
        .init()
        .map_err(AppError::LoggingInitializationError)?;
    info!("Setup up logger with verbosity {}.", verbosity);
    Ok(())
}
