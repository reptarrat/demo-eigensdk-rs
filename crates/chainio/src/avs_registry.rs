//! Client for AVS Registry interactions

use crate::QuorumNum;
use crate::MAX_NUMBER_OF_QUORUMS;
use crate::{OperatorId, Result};
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use eigenlayer_middleware_bindings::{
    operator_state_retriever::{Operator, OperatorStateRetriever},
    registry_coordinator::RegistryCoordinator,
    stake_registry::StakeRegistry,
};
use ethers::core::types::U256;
use ethers::core::types::{Address, Bytes};
use ethers::middleware::Middleware;
use std::collections::HashMap;
use std::sync::Arc;

// TODO: tests for all methods on AvsRegistry
// TODO: tests for bitmap utils
// TODO: add eth client for getting current block number

// Implemented methods for parity with go-sdk
// - [ ] GetQuorumCount
// - [x] GetOperatorsStakeInQuorumsAtCurrentBlock
// - [ ] GetOperatorsStakeInQuorumsAtBlock
// - [] GetOperatorAddrsInQuorumsAtCurrentBlock
// 		- [x] instead did GetOperatorAddrsInQuorumsAtBlock
// - [x] GetOperator**s**StakeInQuorumsOfOperatorAtBlock
// - [ ] GetOperator**s**StakeInQuorumsOfOperatorAtCurrentBlock
// - [x] GetOperatorStakeInQuorumsOfOperatorAtCurrentBlock
// - [x] GetOperatorStakeInQuorumsOfOperatorAtCurrentBlock
// - [ ] GetCheckSignaturesIndices
// - [ ] GetOperatorId
// - [ ] GetOperatorFromId
// - [ ] IsOperatorRegistered
// - [ ] QueryExistingRegisteredOperatorPubKeys

/// Read and write to the AVS Registry contracts.
#[derive(Debug)]
pub struct AvsRegistry<M> {
    /// OperatorStateRetriever contract client
    operator_stake_retriever: OperatorStateRetriever<M>,
    /// Registry coordinator contract address
    // TODO: can we remove this and just find method to fetch address from
    // contract struct?
    registry_coordinator_addr: Address,
    /// Registry coordinator contract
    registry_coordinator: RegistryCoordinator<M>,
    stake_registry: StakeRegistry<M>,
}

impl<M: Middleware + 'static> AvsRegistry<M> {
    /// Create a new instance of [Self].
    pub fn new(
        middleware: Arc<M>,
        operator_state_retriever_addr: Address,
        registry_coordinator_addr: Address,
        stake_registry_addr: Address,
    ) -> Self {
        Self {
            operator_stake_retriever: OperatorStateRetriever::new(
                operator_state_retriever_addr,
                middleware.clone(),
            ),
            registry_coordinator_addr,
            registry_coordinator: RegistryCoordinator::new(
                registry_coordinator_addr.clone(),
                middleware.clone(),
            ),
            stake_registry: StakeRegistry::new(stake_registry_addr, middleware.clone()),
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
                self.registry_coordinator_addr.clone(),
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
            .map(|quorum| {
                quorum
                    .into_iter()
                    .map(|o| o.operator)
                    .collect::<Vec<Address>>()
            })
            .collect())
    }

    /// See operator_stake_retriever getOperatorState in contract source for more details.
    pub async fn get_operators_stake_in_quorums_of_operator_at_block(
        &self,
        operator_id: OperatorId,
        block_number: u32,
    ) -> Result<(Vec<u8>, Vec<Vec<Operator>>), M> {
        let (quorum_bitmap, operator_stakes) = self
            .operator_stake_retriever
            .get_operator_state_with_registry_coordinator_and_operator_id(
                self.registry_coordinator_addr,
                operator_id,
                block_number,
            )
            .await?;

        let quorums = bitmap_to_quorum_ids(quorum_bitmap);

        Ok((quorums, operator_stakes))
    }

    // Danger: This makes multiple calls that are supposed to get "current" info,
    // but that could change between calls as state changes
    /// Get the stake of each quorum the given operator belongs to
    pub async fn get_operator_stake_in_quorums_of_operator_at_current_block(
        &self,
        operator_id: OperatorId,
    ) -> Result<HashMap<QuorumNum, u128>, M> {
        let quorum_bitmap = self
            .registry_coordinator
            .get_current_quorum_bitmap(operator_id)
            .await?;
        let quorums = bitmap_to_quorum_ids(quorum_bitmap);

        let mut quorum_stakes = HashMap::new();
        for quorum in quorums.into_iter() {
            let stake = self
                .stake_registry
                .get_current_stake(operator_id, quorum)
                .await?;
            quorum_stakes.insert(quorum, stake);
        }

        Ok(quorum_stakes)
    }
}

/// See BitmapUtils.bitmapToBytesArray
pub fn bitmap_to_byte_vec(bitmap: U256) -> Vec<u8> {
    let bitvec = BitVec::<_, Lsb0>::from_slice(&bitmap.0);
    debug_assert!(bitvec.len() == 256);

    let mut bytes_vec = vec![0; bitvec.count_ones()];
    let mut bytes_vec_idx = 0;
    let mut i = 0u8;
    while bytes_vec_idx < bytes_vec.len() {
        // check if the i-th bit is flipped in the bitmap
        let mask = U256::from(1 << i);
        if bitmap & mask != U256::one() {
            // if the i-th bit is flipped, then add a byte encoding the value 'i' to the `bytes_vec`
            bytes_vec[bytes_vec_idx] = i;
            bytes_vec_idx += 1
        }

        i += 1
    }

    bytes_vec
}

/// Convert a bitmap to quorum ids
pub fn bitmap_to_quorum_ids(bitmap: U256) -> Vec<u8> {
    let bitvec = BitVec::<_, Lsb0>::from_slice(&bitmap.0);

    (0u8..MAX_NUMBER_OF_QUORUMS)
        .filter_map(|i| {
            bitvec
                .get(i as usize)
                .and_then(|b| if *b { Some(i) } else { None })
        })
        .collect()
}
