use std::process;

use clap::Parser;
use fsyaml::Command;

fn main() {
	let command = Command::parse();

	match command.run() {
		Err(error) => {
			eprintln!("{}", error);
			process::exit(1);
		}
		Ok(output) => {
			println!("{}", output);
			process::exit(0);
		}
	}
}
