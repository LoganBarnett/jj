use structopt::StructOpt;
use crate::config;

#[derive(StructOpt)]
#[structopt(
    name = "jj",
    about = "Run Jenkins jobs from the command line.",
)]
// Without a structopt declaration, the argument is positional.
pub struct Cli {
    #[structopt(skip = "default")]
    pub server: String,
    pub job: String,
}

#[derive(Debug)]
pub enum CliError {
    ConfigServerMissingError(String),
}

pub struct CliValid {
    pub job: String,
    pub server: config::ConfigServerParsed,
}

pub fn cli_validate(config: config::ConfigParsed) -> Result<CliValid, CliError> {
    let cli = Cli::from_args();
    let server_name = if cli.server == "default" {
        config.default_server
    } else {
        cli.server
    };
    match config.servers.get(&server_name) {
        Some(server) => Ok(CliValid {
            job: cli.job,
            server: server.clone(),
        }),
        None => Err(CliError::ConfigServerMissingError(
            format!("Could not find server '{}' in configuration.", server_name),
        )),
    }
}
