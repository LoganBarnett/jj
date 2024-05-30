use clap::Clap;
use crate::config;
use crate::error;
use crate::logging;
use std::error::Error;
use std::collections::HashMap;

/// Parse a single key-value pair.
// Shameful rip from:
// https://github.com/clap-rs/clap_derive/blob/master/examples/keyvalue.rs
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Clap)]
#[clap(
    name = "jj",
    about = "Run Jenkins jobs from the command line.",
)]
#[clap(setting = clap::AppSettings::ColoredHelp)]
// Without a structopt declaration, the argument is positional.
pub struct Cli {
    pub job: String,
    #[clap(short, long, default_value = "default")]
    pub server: String,
    #[clap(long, short = 'v', parse(from_occurrences))]
    pub verbosity: usize,
    // number_of_values = 1 means --param must be repeated for each pair.
    // Values must be provided with an equals sign separating them.  See
    // https://github.com/clap-rs/clap_derive/blob/master/examples/keyvalue.rs
    // for examples.
    #[clap(
        long = "param",
        short = 'P',
        parse(try_from_str = parse_key_val),
        number_of_values = 1,
    )]
    pub params: Vec<(String, String)>,
}

pub struct CliValid {
    pub job: String,
    pub params: HashMap<String, String>,
    pub server: config::ConfigServerParsed,
    pub verbosity: usize,
}

pub fn cli_validate(
    config: config::ConfigParsed,
) -> Result<CliValid, error::AppError> {
    let cli = Cli::parse();
    logging::init_logger(cli.verbosity)?;
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
