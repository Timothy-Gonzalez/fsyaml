use clap::Parser;
use std::{fs, path::Path};

/// A utility to convert a file system of yaml files to single yaml file
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Command {
	/// The root directory
	#[clap()]
	root: String,
}

impl Command {
	pub fn run(&self) -> Result<String, Error> {
		let path = Path::new(&self.root);

		// Check path exists
		if !path.exists() {
			return Err(Error::RootDirectoryDoesNotExist(self.root.clone()));
		}

		// Check is directory
		let metadata = fs::metadata(path)?;

		if !metadata.is_dir() {
			return Err(Error::RootDirectoryIsNotADirectory(self.root.clone()));
		}

		// We now have a confirmed directory

		Ok("wow".to_string())
	}
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("The root directory provided (`{0}`) does not exist")]
	RootDirectoryDoesNotExist(String),

	#[error("Failed to read the metadata of the root directory provided")]
	FailedToReadMetaDataOfRootDirectory(#[from] std::io::Error),

	#[error("The root directory provided (`{0}`) is not a root directory")]
	RootDirectoryIsNotADirectory(String),
}
