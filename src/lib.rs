use clap::Parser;
use serde_yaml::{Mapping, Value};
use std::{fs, io::ErrorKind, path::Path};

const SUPPORTED_ENDINGS: [&'static str; 2] = ["yaml", "yml"];

/// A utility to convert a file system of yaml files to single yaml file
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Command {
	/// The root directory
	#[clap()]
	pub root: String,
	/// (optional) Outputs to standard output if not provided, otherwise will output to the file specified
	#[clap()]
	pub output: Option<String>,
}

impl Command {
	pub fn run(&self) -> Result<(), Error> {
		let path = self.root.clone();

		let output = get_yaml_for_dir(path)?;

		match &self.output {
			None => println!("{}", output),
			Some(path) => {
				fs::write(path, output)
					.map_err(|error| Error::handle_io_error(path.clone(), error))?;
			}
		};

		Ok(())
	}
}

pub fn get_yaml_for_dir(path: String) -> Result<String, Error> {
	// Validate directory
	check_valid_dir(path.clone())?;

	let value = get_value_for_dir(path)?;

	Ok(serde_yaml::to_string(&value).unwrap())
}

fn check_valid_dir(string_path: String) -> Result<(), Error> {
	let path = Path::new(&string_path);

	// Attempt to get metadata
	let metadata = path
		.metadata()
		.map_err(|error| Error::handle_io_error(string_path.clone(), error))?;

	// Check if path provided is directory
	if !metadata.is_dir() {
		return Err(Error::IsNotADirectory(string_path.clone()));
	}

	Ok(())
}

/// Given a path, decide how to convert the mapping for it into a Value
fn get_value_for_dir(path: String) -> Result<Value, Error> {
	let map = get_map_for_dir(path)?;

	Ok(Value::Mapping(map))
}

// Given a path, iterate it recursively to generate a Mapping representing it
fn get_map_for_dir(path: String) -> Result<Mapping, Error> {
	// Get paths in directory
	let paths: Vec<std::path::PathBuf> = fs::read_dir(path.clone())
		.unwrap()
		.map(|res| res.unwrap())
		.map(|entry| entry.path())
		.collect();

	// Create a empty map to store our results in
	let mut map = Mapping::new();

	// Iterate through paths
	for path in paths {
		// Get metadata
		let metadata = path
			.metadata()
			.map_err(|error| Error::handle_io_error(path.display().to_string(), error))?;

		// Check for file name
		if let Some(raw_file_name) = path.file_stem() {
			// Convert raw_file_name to a Value
			let file_name = Value::String(raw_file_name.to_str().unwrap().to_string());

			// Handle directory
			if metadata.is_dir() {
				let value = get_value_for_dir(path.display().to_string())?;
				map.insert(file_name, value);

				continue;

			// Handle file
			} else if metadata.is_file() {
				// Check for files with an extension
				if let Some(extension) = path.extension() {
					// Iterate through supported endings
					for ending in SUPPORTED_ENDINGS {
						// Check for supported ending being the one we are checking
						if extension.eq(ending) {
							// Read the raw content
							let raw_content =
								fs::read_to_string(path.clone()).map_err(|error| {
									Error::handle_io_error(path.display().to_string(), error)
								})?;

							// Convert raw content to value
							let value: Value =
								serde_yaml::from_str(&raw_content).map_err(|error| {
									Error::handle_deserialize_error(
										path.display().to_string(),
										error,
									)
								})?;

							// Insert found result to map
							map.insert(file_name, value);

							// Break as we have already found the matching ending
							break;
						}
					}
				}
			}
		}
	}

	// Return map if we got here without errors
	Ok(map)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("`{0}` does not exist")]
	DoesNotExist(String),

	#[error("Insufficient permissions to read `{0}`")]
	InsufficientPermissions(String),

	#[error("`{0}` is not a directory")]
	IsNotADirectory(String),

	#[error("Experienced IO Error when trying to access `{0}`: {1}")]
	IOError(String, String),

	#[error("Experienced an error while deserializing `{0}`: {1}")]
	DeserializeError(String, String),
}

impl Error {
	pub fn handle_io_error(path: String, error: std::io::Error) -> Error {
		match error.kind() {
			ErrorKind::NotFound => Error::DoesNotExist(path),
			ErrorKind::PermissionDenied => Error::InsufficientPermissions(path),
			_ => Error::IOError(path, error.to_string()),
		}
	}

	pub fn handle_deserialize_error(path: String, error: serde_yaml::Error) -> Error {
		Error::DeserializeError(path, format!("{}", error))
	}
}
