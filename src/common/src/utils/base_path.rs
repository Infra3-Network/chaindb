use std::io;
use std::path::Path;
use std::path::PathBuf;

use tempfile::TempDir;

#[static_init::dynamic(drop, lazy)]
static mut BASE_PATH_TEMP: Option<TempDir> = None;

/// The base path that is used for everything that needs to be written on disk to run a node.
#[derive(Debug)]
pub struct BasePath {
	path: PathBuf,
}

impl BasePath {
	/// Create a `BasePath` instance using a temporary directory prefixed with "substrate" and use
	/// it as base path.
	///
	/// Note: The temporary directory will be created automatically and deleted when the program
	/// exits. Every call to this function will return the same path for the lifetime of the
	/// program.
	pub fn new_temp_dir() -> io::Result<BasePath> {
		let mut temp = BASE_PATH_TEMP.write();

		match &*temp {
			Some(p) => Ok(Self::new(p.path())),
			None => {
				let temp_dir = tempfile::Builder::new().prefix("substrate").tempdir()?;
				let path = PathBuf::from(temp_dir.path());

				*temp = Some(temp_dir);
				Ok(Self::new(path))
			},
		}
	}

	/// Create a `BasePath` instance based on an existing path on disk.
	///
	/// Note: this function will not ensure that the directory exist nor create the directory. It
	/// will also not delete the directory when the instance is dropped.
	pub fn new<P: Into<PathBuf>>(path: P) -> BasePath {
		Self { path: path.into() }
	}

	/// Create a base path from values describing the project.
	pub fn from_project(qualifier: &str, organization: &str, application: &str) -> BasePath {
		BasePath::new(
			directories::ProjectDirs::from(qualifier, organization, application)
				.expect("app directories exist on all supported platforms; qed")
				.data_local_dir(),
		)
	}

	/// Retrieve the base path.
	pub fn path(&self) -> &Path {
		&self.path
	}

	/// Returns the configuration directory inside this base path.
	///
	/// The path looks like `$base_path/chains/$chain_id`
	pub fn config_dir(&self, chain_id: &str) -> PathBuf {
		self.path().join("chains").join(chain_id)
	}
}

impl From<PathBuf> for BasePath {
	fn from(path: PathBuf) -> Self {
		BasePath::new(path)
	}
}
