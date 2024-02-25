//! Chain I/O

mod error;

pub mod avs_registry;

pub use error::{Error, Result};

type OperatorId = [u8; 32];
