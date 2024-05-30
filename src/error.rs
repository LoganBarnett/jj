
#[derive(Debug)]
pub enum AppError {
    CliConfigServerMissingError(String),
    ConfigIoError(std::io::Error),
    ConfigDeserializationError(serdeconv::Error),
    ConfigTokenEvalCommandError(std::io::Error),
    ConfigTokenEvalBufferReadError(std::string::FromUtf8Error),
    ConfigVarError(std::env::VarError),
    JenkinsBuildNotFoundError,
    JenkinsBuildParseTextSizeError,
    JenkinsBuildParamSerializeError(serde_url_params::Error),
    JenkinsBuildStreamError(reqwest::Error),
    JenkinsBuildResponseReadError(reqwest::Error),
    JenkinsBuildOutputError(std::io::Error),
    JenkinsDeserializeError(serde_json::Error),
    JenkinsEnqueueError(reqwest::Error),
    JenkinsEnqueueDeserializeError(String),
    JenkinsEnqueueSecondsParseError(std::num::ParseIntError),
    JenkinsEnqueueWaitError(String),
    JenkinsHeaderError(reqwest::header::ToStrError),
    LoggingInitializationError(log::SetLoggerError),
}
