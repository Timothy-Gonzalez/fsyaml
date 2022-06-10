use std::{fs, process};

use clap::Parser;
use fsyaml::{Command, Error};

fn main() {
	let command = Command::parse();

	match command.run() {
		Err(error) => {
			eprintln!("{}", error);
			process::exit(1);
		}
		Ok(_) => {
			process::exit(0);
		}
	}
}
