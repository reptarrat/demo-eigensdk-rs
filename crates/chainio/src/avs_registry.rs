//! Client for AVS Registry interactions

use eigenlayer_middleware_bindings::operator_state_retriever::{
	OperatorStateRetriever, Operator
};
use ethers::core::types::{Address, Bytes};
use ethers::middleware::Middleware;
use std::sync::Arc;
use crate::{Result, OperatorId};

/// Read and write to the AVS Registry contracts.
#[derive(Debug)]
pub struct AvsRegistry<M> {
	/// OperatorStateRetriever contract client
    operator_stake_retriever: OperatorStateRetriever<M>,
	/// Registry coordinator address
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
    pub async fn get_operators_stake_in_quorums_at_block(
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

	/// Returns ordered list of operator addresses for each quroum. See
	/// See operator_stake_retriever getOperatorState in contract source for more details.
	pub async fn get_operator_address_in_quorums_at_block(
		&self,
        quorum_numbers: Bytes,
        block_number: u32,
	) -> Result<Vec<Vec<Address>>, M> {
		Ok(self
			.get_operators_stake_in_quorums_at_block(quorum_numbers, block_number)
			.await?
			.into_iter()
			.map(|quorum|
				quorum.into_iter().map(|o| o.operator).collect::<Vec<Address>>()
			)
			.collect())
	}

	/// See operator_stake_retriever getOperatorState in contract source for more details.
	pub async fn get_operators_stake_in_quorums_of_operator_at_block(
		&self,
        operator_id: OperatorId,
        block_number: u32,
	) -> Result<Ve<u8>, M> {
		let (quorum_bitmap, operator_stakes) = self
			.operator_stake_retriever
			.get_operator_state_with_registry_coordinator_and_operator_id(
				self.registry_coordinator,
				operator_id,
				block_number,
			)
			.await?;

		// TODO: bitmap to quorum IDs
		unimplemented!()
	}
}
