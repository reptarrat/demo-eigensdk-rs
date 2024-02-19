// use forge to generate bindings
//

fn main() {
	println!("cargo:rerun-if-changed=lib/");

	std::process::Command::new("forge")
		.arg("bind")
		.arg("--bindings-path")
		.arg("../../lib/eigenlayer-middleware")
		.arg("--crate-name")
		.arg("eigenlayer-middleware")
		.output()
		.expect("failed to run forge build");
}
