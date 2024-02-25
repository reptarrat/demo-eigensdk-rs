//! Chain I/O

// use alloy_rpc_client::RpcClient;
use eigenlayer_middleware_bindings::operator_state_retriever::{
	OperatorStateRetriever, CheckSignaturesIndices, Operator
};
use ethers::core::types::{Address, Bytes};
use ethers::middleware::contract::ContractError;
use ethers::middleware::Middleware;
use std::sync::Arc;

pub type Result<T, M> = core::result::Result<T, Error<M>>;

pub enum Error<M: Middleware> {
    ContractError { e: ContractError<M> },
}

impl<M: Middleware> From<ContractError<M>> for Error<M> {
	fn from(e: ContractError<M>) -> Self {
		Error::ContractError { e}
	}
}

/// Read and write to the AVS Registry contracts.
pub struct AvsRegistry<M> {
    operator_stake_retriever: OperatorStateRetriever<M>,
    registry_coordinator: Address,
}

impl<M: Middleware> AvsRegistry<M> {
    /// Create a new instance of [Self].
    pub fn new(
        middleware: Arc<M>,
        operator_state_retriever: Address,
        registry_coordinator: Address,
    ) -> Self {
        Self {
            operator_stake_retriever: OperatorStateRetriever::new(
                operator_state_retriever,
                middleware,
            ),
            registry_coordinator: registry_coordinator,
        }
    }

    /// Returns the ordered list of operators (id and stake) for each quorum.
	/// See contract source for parameter details.
    async fn get_operators_stake_in_quorums_at_block(
        &self,
        quorum_numbers: Bytes,
        block_number: u32,
    ) -> Result<Vec<Vec<Operator>>, M> {
        self.operator_stake_retriever
            .get_operator_state(
                self.registry_coordinator.clone(),
                quorum_numbers,
                block_number,
            )
            .call()
			.await
			.map_err(Into::into)
    }
}
