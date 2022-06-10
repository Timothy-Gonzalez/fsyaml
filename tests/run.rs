use fsyaml::Command;
use std::fs;

fn test(path: &str) {
	let expected_output = fs::read_to_string(format!("{}/expected.yaml", path))
		.expect("Failed to read expected.yaml")
		.replace("\r", "");

	let real_output = Command {
		root: format!("{}/root", path),
	}
	.run()
	.expect("An error occurred");

	assert_eq!(expected_output, real_output)
}

#[test]
pub fn basic() {
	test("tests/basic")
}
