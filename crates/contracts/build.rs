use std::env;
use std::fs;
use std::path::Path;
use ethers_solc::{Project, ProjectPathsConfig};
use ethers_solc::Solc;

fn main() {
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("hello.rs");

	let sol_files = vec![
		"./external/eigenlayer-middleware/src/BLSApkRegistry.sol",
		"./external/eigenlayer-middleware/src/BLSApkRegistryStorage.sol",
		"./external/eigenlayer-middleware/src/BLSSignatureChecker.sol",
		"./external/eigenlayer-middleware/src/IndexRegistry.sol",
		"./external/eigenlayer-middleware/src/IndexRegistryStorage.sol",
		"./external/eigenlayer-middleware/src/IndexRegistryRetriever.sol",
		"./external/eigenlayer-middleware/src/RegistryCoordinator.sol",
		"./external/eigenlayer-middleware/src/RegistryCoordinatorStorage.sol",
		"./external/eigenlayer-middleware/src/ServiceManagerBase.sol",
		"./external/eigenlayer-middleware/src/StakeRegistry.sol",
		"./external/eigenlayer-middleware/src/StakeRegistryStorage.sol",
	];

	let compiler_output =
		Solc::default().compile_source("./external/eigenlayer-middleware/src/").unwrap();

	// for (name, contract) in compiler_output.contracts_into_iter() {
	// 	let abi = contract.abi.unwrap();
	// 	let json = serde_json::to_string(&abi).unwrap();

	// 	fs::write(format!("src/gen/{name}.abi.json"), &json).unwrap();
	// }


}