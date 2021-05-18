
#[derive(Debug)]
pub enum AppError {
    CliConfigServerMissingError(String),
    ConfigIoError(std::io::Error),
    ConfigDeserializationError(serdeconv::Error),
    ConfigTokenEvalCommandError(std::io::Error),
    ConfigTokenEvalBufferReadError(std::string::FromUtf8Error),
    ConfigValidationError,
    ConfigVarError(std::env::VarError),
    JenkinsEnqueueError(reqwest::Error),
    JenkinsHeaderError(reqwest::header::ToStrError),
}
