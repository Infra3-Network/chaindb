use super::node::NodeKeyConfig;

/// Network service configuration.
#[derive(Clone, Debug)]
pub struct NetworkConfiguration {
    /// Name of the node. Sent over the wire for debugging purposes.
	pub node_name: String,
    /// The node key configuration, which determines the node's network identity keypair.
	pub node_key: NodeKeyConfig,
}