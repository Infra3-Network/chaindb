use tracing::info;

use chaindb_common::Error;
use chaindb_common::configs::NetworkConfiguration;

pub struct NetworkWorker {
    
}

impl NetworkWorker {
    pub fn new(mut cfg: NetworkConfiguration) -> Result<Self, Error> {
        let node_identify = cfg.node_key.clone().into_keypair()?;
        let node_public_key = node_identify.public();
        let node_peer_id = node_public_key.to_peer_id();

        println!("🏷  Local node identity is: {}", node_peer_id.to_base58(),);

        Ok(Self {})
    }
}
