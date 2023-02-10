use std::path::PathBuf;

use clap::CommandFactory;
use clap::FromArgMatches;
use clap::Parser;

use chaindb_common::params::NetworkParams;
use chaindb_common::params::SharedParams;
use chaindb_common::utils::generate_node_name;
use chaindb_common::BasePath;
use chaindb_common::Error;

use crate::config::Configuration;

use super::service::new_service;

/// Default sub directory to store network config.
pub(crate) const DEFAULT_NETWORK_CONFIG_PATH: &str = "network";

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub network_params: NetworkParams,

    #[allow(missing_docs)]
    #[clap(flatten)]
    pub shard_params: SharedParams,
}

impl RunCommand {}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {}

#[derive(Debug, clap::Parser)]
pub struct Command {
    #[command(subcommand)]
    pub sub: Option<Subcommand>,

    #[clap(flatten)]
    pub run: RunCommand,
}

impl Command {
    /// Command version
    fn version() -> Option<&'static str> {
        "v0.1.0-dawn".into()
    }

    /// Executable file name.
    ///
    /// Extracts the file name from `std::env::current_exe()`.
    /// Resorts to the env var `CARGO_PKG_NAME` in case of Error.
    fn executable_name() -> String {
        std::env::current_exe()
            .ok()
            .and_then(|e| e.file_name().map(|s| s.to_os_string()))
            .and_then(|w| w.into_string().ok())
            .unwrap_or_else(|| env!("CARGO_PKG_NAME").into())
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> Option<&'static str> {
        env!("CARGO_PKG_AUTHORS").into()
    }

    // fn support_url() -> String {
    // 	"support.anonymous.an".into()
    // }

    // fn copyright_start_year() -> i32 {
    // 	2017
    // }

    fn from_args() -> Self
    where
        Self: Parser + Sized,
    {
        Self::from_iter(&mut std::env::args_os())
    }

    /// Helper function used to parse the command line arguments. This is the equivalent of
    /// [`clap::Parser::parse_from`].
    ///
    /// To allow running the node without subcommand, it also sets a few more settings:
    /// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
    /// [`clap::Command::subcommand_negates_reqs`].
    ///
    /// Creates `Self` from any iterator over arguments.
    /// Print the error message and quit the program in case of failure.
    fn from_iter<I>(iter: I) -> Self
    where
        Self: Parser + Sized,
        I: IntoIterator,
        I::Item: Into<std::ffi::OsString> + Clone,
    {
        let app = <Self as CommandFactory>::command();
        let full_version = Self::version();
        let exec_name = Self::executable_name();
        let author = Self::author();
        let about = Self::description();
        let app = app
            .bin_name(exec_name)
            .author(author)
            .about(about)
            .version(full_version)
            .propagate_version(true)
            .args_conflicts_with_subcommands(true)
            .subcommand_negates_reqs(true);

        let matches = app.try_get_matches_from(iter).unwrap_or_else(|e| e.exit());

        <Self as FromArgMatches>::from_arg_matches(&matches).unwrap_or_else(|e| e.exit())
    }

    pub(crate) fn create_configuration(&self) -> Result<Configuration, Error> {
        let base_path = self
            .run
            .shard_params
            .base_path()?
            .unwrap_or_else(|| BasePath::from_project("", "", &Self::executable_name()));

        let config_dir = base_path.config_dir("todo!");
        let net_config_dir = config_dir.join(DEFAULT_NETWORK_CONFIG_PATH);

        let node_name = generate_node_name();
        let network = self
            .run
            .network_params
            .to_network_config(&node_name, &net_config_dir)?;

        Ok(Configuration { network })
    }
}

pub fn run() -> Result<(), Error> {
    let cli = Command::from_args();
    match &cli.sub {
        Some(_) => unimplemented!(),
        None => do_run_cli(&cli),
    }
}

fn do_run_cli(cli: &Command) -> Result<(), Error> {
    let cfg = cli.create_configuration()?;
    new_service(&cfg)?;

    unimplemented!()
}
