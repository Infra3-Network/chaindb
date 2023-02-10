use std::path::PathBuf;
use std::str::FromStr;

use clap::Args;
use clap::ValueEnum;
use libp2p::core::identity::ed25519;
use primitive_types::H256;

use crate::configs::Ed25519Secret;
use crate::configs::NodeKeyConfig;
use crate::configs::Secret;
use crate::Error;


/// The file name of the node's Ed25519 secret key inside the chain-specific
/// network config directory, if neither `--node-key` nor `--node-key-file`
/// is specified in combination with `--node-key-type=ed25519`.
const NODE_KEY_ED25519_FILE: &str = "secret_ed25519";

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum NodeKeyType {
    /// Use ed25519.
    Ed25519,
}

/// Parameters used to create the `NodeKeyConfig`, which determines the keypair
/// used for libp2p networking.
#[derive(Debug, Clone, Args)]
pub struct NodeKeyParams {
    /// The secret key to use for libp2p networking.
    ///
    /// The value is a string that is parsed according to the choice of
    /// `--node-key-type` as follows:
    ///
    ///   `ed25519`:
    ///   The value is parsed as a hex-encoded Ed25519 32 byte secret key,
    ///   i.e. 64 hex characters.
    ///
    /// The value of this option takes precedence over `--node-key-file`.
    ///
    /// WARNING: Secrets provided as command-line arguments are easily exposed.
    /// Use of this option should be limited to development and testing. To use
    /// an externally managed secret key, use `--node-key-file` instead.
    #[arg(long, value_name = "KEY")]
    pub node_key: Option<String>,

    /// The type of secret key to use for libp2p networking.
    ///
    /// The secret key of the node is obtained as follows:
    ///
    ///   * If the `--node-key` option is given, the value is parsed as a secret key according to
    ///     the type. See the documentation for `--node-key`.
    ///
    ///   * If the `--node-key-file` option is given, the secret key is read from the specified
    ///     file. See the documentation for `--node-key-file`.
    ///
    ///   * Otherwise, the secret key is read from a file with a predetermined, type-specific name
    ///     from the chain-specific network config directory inside the base directory specified by
    ///     `--base-dir`. If this file does not exist, it is created with a newly generated secret
    ///     key of the chosen type.
    ///
    /// The node's secret key determines the corresponding public key and hence the
    /// node's peer ID in the context of libp2p.
    #[arg(long, value_name = "TYPE", value_enum, ignore_case = true, default_value_t = NodeKeyType::Ed25519)]
    pub node_key_type: NodeKeyType,

    /// The file from which to read the node's secret key to use for libp2p networking.
    ///
    /// The contents of the file are parsed according to the choice of `--node-key-type`
    /// as follows:
    ///
    ///   `ed25519`:
    ///   The file must contain an unencoded 32 byte or hex encoded Ed25519 secret key.
    ///
    /// If the file does not exist, it is created with a newly generated secret key of
    /// the chosen type.
    #[arg(long, value_name = "FILE")]
    pub node_key_file: Option<PathBuf>,
}

impl NodeKeyParams {
    /// Create a `NodeKeyConfig` from the given `NodeKeyParams` in the context
    /// of an optional network config storage directory.
    pub fn node_key(&self, net_config_dir: &PathBuf) -> Result<NodeKeyConfig, Error> {
        Ok(match self.node_key_type {
            NodeKeyType::Ed25519 => {
                let secret = if let Some(node_key) = self.node_key.as_ref() {
                    parse_ed25519_secret(node_key)?
                } else {
                    Secret::File(
                        self.node_key_file
                            .clone()
                            .unwrap_or_else(|| net_config_dir.join(NODE_KEY_ED25519_FILE)),
                    )
                };

                NodeKeyConfig::Ed25519(secret)
            }
        })
    }
}

/// Create an error caused by an invalid node key argument.
fn invalid_node_key(e: impl std::fmt::Display) -> Error {
    Error::Input(format!("Invalid node key: {}", e))
}

/// Parse a Ed25519 secret key from a hex string into a `sc_network::Secret`.
fn parse_ed25519_secret(hex: &str) -> Result<Ed25519Secret, Error> {
    H256::from_str(hex)
        .map_err(invalid_node_key)
        .and_then(|bytes| {
            ed25519::SecretKey::from_bytes(bytes)
                .map(Secret::Input)
                .map_err(invalid_node_key)
        })
}
