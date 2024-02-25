//! Chain I/O

mod error;

pub mod avs_registry;

pub use error::{Error, Result};

/// Maximum number of quorums
pub const MAX_NUMBER_OF_QUORUMS: u8 = 192;

/// Operator Id
pub type OperatorId = [u8; 32];
/// Quorum number
pub type QuorumNum = u8;
