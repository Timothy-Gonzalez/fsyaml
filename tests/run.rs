use serde_yaml::{self, Value};
use std::fs;

fn test(path: &str) {
	let expected_output = fs::read_to_string(format!("{}/expected.yaml", path))
		.expect("Failed to read expected.yaml")
		.replace("\r", "");

	let real_output =
		fsyaml::get_yaml_for_dir(format!("{}/root", path)).expect("An error occurred");

	let deserialized_expected: Value =
		serde_yaml::from_str(expected_output.as_str()).expect("Expected cannot be deserialized");
	let deserialized_real: Value =
		serde_yaml::from_str(real_output.as_str()).expect("Real cannot be deserialized");

	assert_eq!(deserialized_expected, deserialized_real)
}

#[test]
pub fn basic() {
	test("tests/basic")
}
