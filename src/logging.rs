use crate::error;
use log::*;

pub fn init_logger(verbosity: usize) -> Result<(), error::AppError> {
    let mut logger = stderrlog::new();
    logger
        // module_path doesn't work here. Nothing is logged using it.
        // .module("jj")
        // .module(module_path!())
        // .modules(vec!("cli", "jenkins", "jj", "main"))
        .verbosity(verbosity)
        .init()
        .map_err(error::AppError::LoggingInitializationError)?;
    warn!("Setup up logger with verbosity {}.", verbosity);
    Ok(())
}
