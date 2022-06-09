use clap::Parser;
use std::path::Path;

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
		check_valid_directory(self.root.clone())?;

		// We now have a confirmed directory
		println!("{}", self.root);

		Ok("wow".to_string())
	}
}

fn check_valid_directory(string_path: String) -> Result<(), Error> {
	let path = Path::new(&string_path);

	// Check path exists
	if !path.exists() {
		return Err(Error::DoesNotExist(string_path));
	}

	// Attempt to get metadata (only fails in inefficient permissions, as path existence is already checked)
	let metadata = match path.metadata() {
		Ok(metadata) => metadata,
		Err(_) => return Err(Error::InsufficientPermissions(string_path.clone())),
	};

	// Check if path provided is directory
	if !metadata.is_dir() {
		return Err(Error::IsNotADirectory(string_path.clone()));
	}

	Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("`{0}` does not exist")]
	DoesNotExist(String),

	#[error("Insufficient permissions to read `{0}`")]
	InsufficientPermissions(String),

	#[error("`{0}` is not a directory")]
	IsNotADirectory(String),
}
