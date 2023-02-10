use chaindb_common::Error;

use crate::{network::NetworkWorker, config::Configuration};

pub fn new_service(cfg: &Configuration) -> Result<(), Error>{
    // build network
    let worker = NetworkWorker::new(cfg.network.clone())?;
    unimplemented!()
    
}