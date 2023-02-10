use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use libp2p::core::identity::ed25519;
use libp2p::core::identity::Keypair;
use zeroize::Zeroize;

/// Role of the local node.
#[derive(Debug, Clone)]
pub enum Role {
    /// Full node.
    Full,
    /// Light node.
    Light,
}


/// The configuration of a node's secret key, describing the type of key
/// and how it is obtained. A node's identity keypair is the result of
/// the evaluation of the node key configuration.
#[derive(Clone, Debug)]
pub enum NodeKeyConfig {
    /// A Ed25519 secret key configuration.
    Ed25519(Secret<ed25519::SecretKey>),
}

impl Default for NodeKeyConfig {
    fn default() -> NodeKeyConfig {
        Self::Ed25519(Secret::New)
    }
}

/// The options for obtaining a Ed25519 secret key.
pub type Ed25519Secret = Secret<ed25519::SecretKey>;

/// The configuration options for obtaining a secret key `K`.
#[derive(Clone)]
pub enum Secret<K> {
    /// Use the given secret key `K`.
    Input(K),
    /// Read the secret key from a file. If the file does not exist,
    /// it is created with a newly generated secret key `K`. The format
    /// of the file is determined by `K`:
    ///
    ///   * `ed25519::SecretKey`: An unencoded 32 bytes Ed25519 secret key.
    File(PathBuf),
    /// Always generate a new secret key `K`.
    New,
}

impl<K> fmt::Debug for Secret<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Input(_) => f.debug_tuple("Secret::Input").finish(),
            Self::File(path) => f.debug_tuple("Secret::File").field(path).finish(),
            Self::New => f.debug_tuple("Secret::New").finish(),
        }
    }
}

impl NodeKeyConfig {
    /// Evaluate a `NodeKeyConfig` to obtain an identity `Keypair`:
    ///
    ///  * If the secret is configured as input, the corresponding keypair is returned.
    ///
    ///  * If the secret is configured as a file, it is read from that file, if it exists. Otherwise
    ///    a new secret is generated and stored. In either case, the keypair obtained from the
    ///    secret is returned.
    ///
    ///  * If the secret is configured to be new, it is generated and the corresponding keypair is
    ///    returned.
    pub fn into_keypair(self) -> io::Result<Keypair> {
        use NodeKeyConfig::*;
        match self {
            Ed25519(Secret::New) => Ok(Keypair::generate_ed25519()),

            Ed25519(Secret::Input(k)) => Ok(Keypair::Ed25519(k.into())),

            Ed25519(Secret::File(f)) => get_secret(
                f,
                |mut b| match String::from_utf8(b.to_vec()).ok().and_then(|s| {
                    if s.len() == 64 {
                        array_bytes::hex2bytes(&s).ok()
                    } else {
                        None
                    }
                }) {
                    Some(s) => ed25519::SecretKey::from_bytes(s),
                    _ => ed25519::SecretKey::from_bytes(&mut b),
                },
                ed25519::SecretKey::generate,
                |b| b.as_ref().to_vec(),
            )
            .map(ed25519::Keypair::from)
            .map(Keypair::Ed25519),
        }
    }
}

/// Load a secret key from a file, if it exists, or generate a
/// new secret key and write it to that file. In either case,
/// the secret key is returned.
fn get_secret<P, F, G, E, W, K>(file: P, parse: F, generate: G, serialize: W) -> io::Result<K>
where
    P: AsRef<Path>,
    F: for<'r> FnOnce(&'r mut [u8]) -> Result<K, E>,
    G: FnOnce() -> K,
    E: Error + Send + Sync + 'static,
    W: Fn(&K) -> Vec<u8>,
{
    std::fs::read(&file)
        .and_then(|mut sk_bytes| {
            parse(&mut sk_bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                file.as_ref().parent().map_or(Ok(()), fs::create_dir_all)?;
                let sk = generate();
                let mut sk_vec = serialize(&sk);
                write_secret_file(file, &sk_vec)?;
                sk_vec.zeroize();
                Ok(sk)
            } else {
                Err(e)
            }
        })
}

/// Write secret bytes to a file.
fn write_secret_file<P>(path: P, sk_bytes: &[u8]) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = open_secret_file(&path)?;
    file.write_all(sk_bytes)
}

/// Opens a file containing a secret key in write mode.
#[cfg(unix)]
fn open_secret_file<P>(path: P) -> io::Result<fs::File>
where
    P: AsRef<Path>,
{
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

/// Opens a file containing a secret key in write mode.
#[cfg(not(unix))]
fn open_secret_file<P>(path: P) -> Result<fs::File, io::Error>
where
    P: AsRef<Path>,
{
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
}


