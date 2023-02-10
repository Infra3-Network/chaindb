use chaindb_common::BasePath;
use chaindb_common::configs::NetworkConfiguration;
use chaindb_common::utils::generate_node_name;
use clap::error::Error;

use crate::cli::Command;



/// Chiandb node configuration.
#[derive(Debug)]
pub struct Configuration {
    // Network configuration.
    pub network: NetworkConfiguration,
}