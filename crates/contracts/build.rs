// use forge to generate bindings
//

fn main() {
	println!("cargo:rerun-if-changed=lib/");

	std::process::Command("forge")
		.arg("bind")
		.arg("--bindings-path")
		.arg("lib/eigenlayer-middleware")
		.arg("--crate-name")
		.arg("eigenlayer-middleware")
}
