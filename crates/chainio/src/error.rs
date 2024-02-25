//! Error types for this crate.

use ethers::middleware::contract::ContractError;
use ethers::middleware::Middleware;

/// Result convenience type for this crate
pub type Result<T, M> = core::result::Result<T, Error<M>>;

/// Error type for this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error<M: Middleware> {
    /// ethers contract error
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
}
