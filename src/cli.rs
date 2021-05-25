use clap::Clap;
use crate::config;
use crate::error;
use crate::logging;

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
}

pub struct CliValid {
    pub job: String,
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
