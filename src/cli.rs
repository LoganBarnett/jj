use clap::Parser;
use clap_verbosity_flag::Verbosity;
use crate::config;
use crate::error;
use crate::logging;
use std::error::Error;
use std::collections::HashMap;

/// Parse a single key-value pair.
// Shameful rip from:
// https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs#L24-L26
// and
// https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs#L47-L59
fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser)]
#[command(
    name = "jj",
    about = "Run Jenkins jobs from the command line.",
)]
// Without a structopt declaration, the argument is positional.
pub struct Cli {
    pub job: String,
    #[arg(short, long, default_value = "default")]
    pub server: String,
    #[command(flatten)]
    pub verbosity: Verbosity,
    // number_of_values = 1 means --param must be repeated for each pair.
    // Values must be provided with an equals sign separating them.  See
    // https://github.com/clap-rs/clap_derive/blob/master/examples/keyvalue.rs
    // for examples.
    #[arg(
        long = "param",
        short = 'P',
        value_parser = parse_key_val::<String, String>,
        // Forces the user to specify another argument for a new key value pair.
        number_of_values = 1,
    )]
    pub params: Vec<(String, String)>,
}

pub struct CliValid {
    pub job: String,
    pub params: HashMap<String, String>,
    pub server: config::ConfigServerParsed,
    pub verbosity: Verbosity,
}

pub fn cli_validate(
    config: config::ConfigParsed,
) -> Result<CliValid, error::AppError> {
    let cli = Cli::parse();
    logging::init_logger(&cli.verbosity)?;
    let server_name = if cli.server == "default" {
        config.default_server
    } else {
        cli.server
    };
    match config.servers.get(&server_name) {
        Some(server) => Ok(CliValid {
            job: cli.job,
            params: cli.params.into_iter().collect(),
            server: server.clone(),
            verbosity: cli.verbosity,
        }),
        None => Err(error::AppError::CliConfigServerMissingError(
            format!(
                "Could not find server '{}' in configuration.",
                server_name,
            ),
        )),
    }
}
