use std::path::PathBuf;

use clap::Args;

use crate::configs::NetworkConfiguration;
use crate::Error;

use super::node::NodeKeyParams;


/// Parameters used to create the network configuration.
#[derive(Debug, Clone, Args)]
pub struct NetworkParams {
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub node_key_params: Option<NodeKeyParams>,
}

impl NetworkParams {
    pub fn to_network_config(
        &self,
        node_name: &str,
        net_config_dir: &PathBuf,
    ) -> Result<NetworkConfiguration, Error> {
        
        let node_key = self
            .node_key_params
            .as_ref()
            .map(|params| params.node_key(net_config_dir))
            .unwrap_or_else(|| Ok(Default::default()))?;

        
            Ok(NetworkConfiguration {
                node_name: node_name.into(),
                node_key,
            })
    }
}
