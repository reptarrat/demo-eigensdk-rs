//! Chain I/O

// use bls12_381::{G1Affine, G2Affine};
use eigenlayer_middleware_bindings::shared_types::{self, G1Point, G2Point};

mod error;

pub mod avs_registry;

pub use error::{Error, Result};

/// Maximum number of quorums
pub const MAX_NUMBER_OF_QUORUMS: u8 = 192;

/// Operator Id
pub type OperatorId = [u8; 32];
/// Quorum number
pub type QuorumNum = u8;

/// Operator public key
pub struct OperatorPubkeys {
    // G1 signatures are used to verify signatures onchain (since G1 is cheaper to verify onchain via precompiles)
    pub pubkey_g1: shared_types::G1Point,

    // G2 is used to verify signatures offchain (signatures are on G1)
    // pub g2_pub G2Affine,
    pub pubkey_g2: shared_types::G2Point,
    /// Operator address
    pub address: ethers::core::types::Address,
}
