use crate::error;
use log::warn;
use serde::{Deserialize, Serialize};
use serde;
use serdeconv;
use std::collections::{HashMap};
use std::env;
use std::fs;
use std::io::ErrorKind::NotFound;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct ConfigFromFile {
    pub default_server: String,
    #[serde(flatten)]
    pub servers: HashMap<String, ConfigServerFromFile>,
}

// See https://serde.rs/lifetimes.html for details regarding how to make this a
// lifetime deserialization.
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigServerFromFile {
    pub host_url: String,
    /**
     * The string to be evaluated using the shell which will provide the token.
     * If you feel comfortable leaving your security token in here directly,
     * just surround it with single quotes, for example: "'my-token'"
     */
    pub token_eval: String,
    pub username: String,
}

#[derive(Clone)]
pub struct ConfigParsed {
    pub default_server: String,
    pub servers: HashMap<String, ConfigServerParsed>,
}

#[derive(Clone)]
pub struct ConfigServerParsed {
    pub name: String,
    pub host_url: String,
    pub username: String,
    pub token: String,
}

fn path(paths: &[&str]) -> std::path::PathBuf
{
    paths.iter().collect()
}

fn config_dir_ensure() -> Result<(), error::AppError> {
    fs::create_dir_all(
        path(&[
            &env::var("HOME").map_err(error::AppError::ConfigVarError)?,
            &".config".to_string(),
            &"jj".to_string(),
        ])
    ).map_err(error::AppError::ConfigIoError)
}

fn config_from_file() -> Result<ConfigFromFile, error::AppError> {
    serdeconv::from_toml_file(
        path(&[
            &env::var("HOME").map_err(error::AppError::ConfigVarError)?,
            &".config".to_string(),
            &"jj".to_string(),
            &"config.toml".to_string(),
        ])
    )
        .or_else(|err| {
            // Better way to do this?  What happens if it is not a
            // std::io::Error?
            match err.concrete_cause::<std::io::Error>() {
                Some(inner) => {
                    if inner.kind() == NotFound {
                        warn!("No configuration file found, assuming empty configuration...");
                        Ok(ConfigFromFile {
                            default_server: "".to_string(),
                            servers: HashMap::new(),
                        })
                    } else {
                        Err(err)
                    }
                },
                None => Err(err),
            }
        })
        .map_err(error::AppError::ConfigDeserializationError)
}

// defaultServer should exist among servers, or something is wrong.
fn config_validate(
    config_from_file: ConfigFromFile,
) -> Result<ConfigParsed, error::AppError> {
    Ok(ConfigParsed {
        default_server: config_from_file.default_server,
        servers: config_from_file.servers.into_iter().map(|(k, v)| {
            Ok((k.clone(), ConfigServerParsed {
                name: k,
                host_url: v.host_url,
                token: token_eval(v.token_eval)?,
                username: v.username,
            }))
        }).collect::<Result<
                HashMap<String, ConfigServerParsed>,
                error::AppError,
            >>()?,
    })
}

// It has been suggested that I could use a lifetime indicator here, but I
// haven't had much luck getting it working. I'm trying to use it in the context
// of function composition. For now I am shelving the endeavor and will return
// after some more advice, wisdom, or pairing muscle.
pub fn config_load() -> Result<ConfigParsed, error::AppError> {
    config_dir_ensure()
        .and_then(|_| config_from_file())
        .and_then(config_validate)
}

// TODO: Maybe use an alias here for the token.
fn token_eval(token_code: String) -> Result<String, error::AppError> {
    // Beware that sh could be a shell you don't exepct in your environment..
    Command::new("sh")
        .args(&["-c", &token_code])
        .output()
        .map_err(error::AppError::ConfigTokenEvalCommandError)
        .and_then(|x| {
            String::from_utf8(x.stdout)
                  .map_err(error::AppError::ConfigTokenEvalBufferReadError)
        })
        .map(|x| x.to_string())
}
