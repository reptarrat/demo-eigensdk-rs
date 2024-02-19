use std::env;
use std::fs;
use std::path::Path;
use ethers_solc::{Project, ProjectPathsConfig};

fn main() {
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("hello.rs");

	// let sol_files = vec![
	// 	"./external/eigenlayer-middleware/src/BLSApkRegistry.sol",
	// 	"./external/eigenlayer-middleware/src/BLSApkRegistryStorage.sol",
	// 	"./external/eigenlayer-middleware/src/BLSSignatureChecker.sol",
	// 	"./external/eigenlayer-middleware/src/IndexRegistry.sol",
	// 	"./external/eigenlayer-middleware/src/IndexRegistryStorage.sol",
	// 	"./external/eigenlayer-middleware/src/IndexRegistryRetriever.sol",
	// 	"./external/eigenlayer-middleware/src/RegistryCoordinator.sol",
	// 	"./external/eigenlayer-middleware/src/RegistryCoordinatorStorage.sol",
	// 	"./external/eigenlayer-middleware/src/ServiceManagerBase.sol",
	// 	"./external/eigenlayer-middleware/src/StakeRegistry.sol",
	// 	"./external/eigenlayer-middleware/src/StakeRegistryStorage.sol",
	// ];

	// Solc::default().async_compile_source("./external/eigenlayer-middleware/src/")

	let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat("./external/eigenlayer-middleware/src/").unwrap())
        .build()
        .unwrap();
    let output = project.compile().unwrap();

    // Tell Cargo that if a source file changes, to rerun this build script.
    project.rerun_if_sources_changed();

    // fs::write(
    //     &dest_path,
    //     "pub fn message() -> &'static str {
    //         \"Hello, World!\"
    //     }
    //     "
    // ).unwrap();

    // println!("cargo:rerun-if-changed=external/*");
}