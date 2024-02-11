//! Signer


// A Wallet for keys persisted in memory and on fs. This is a type alias for `Wallet<SigningKey>`.
// See `impl Wallet<SigningKey>` in alloy-signer.
pub use alloy_signer::{LocalWallet, Signer};

#[cfg(test)]
mod tests {
	use super::*;
	use alloy_consensus::TxLegacy;
    use alloy_primitives::{address, b256, ChainId, Signature, U256, Address};

	struct Test {
		path: String,
		password: String,
		success: bool,
		address: Address,
	}

	#[tokio::test]
	async fn decrypts_mock_key_from_go_sdk() {
		let test_cases = vec![
			Test {
				path:     "./mock/dummy.key.json".to_string(),
				password: "test12345".to_string(),
				success:  true,
				address: address!("d4ee935d77b590aa64aed7b02dd66f1383006684"),
			},
			Test {
				path:     "./mockdata/dummy.key.json".to_string(),
				password: "wrong_password".to_string(),
				success:  false,
				address: address!("d4ee935d77b590aa64aed7b02dd66f1383006684"),
			},
		];

		for case in test_cases {
			let result = LocalWallet::decrypt_keystore(case.path, case.password);

			if let Ok(wallet) = result {
				assert_eq!(
					wallet.address(),
					case.address
				)
			} else {
				assert!(!case.success)
			}
		}
	}
}