#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Io error
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Input error. e.g. user input args.
    #[error("Invalid input: {0}")]
	Input(String),

}
