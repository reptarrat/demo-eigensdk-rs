//! Client for AVS Registry interactions

use crate::{OperatorId, OperatorPubkeys, QuorumNum, Result, MAX_NUMBER_OF_QUORUMS};
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use eigenlayer_middleware_bindings::{
    bls_apk_registry::BLSApkRegistry,
    operator_state_retriever::{CheckSignaturesIndices, Operator, OperatorStateRetriever},
    registry_coordinator::RegistryCoordinator,
    shared_types,
    stake_registry::StakeRegistry,
};
use ethers::{
    core::types::{Address, Bytes, Filter, H160, H256, U256},
    providers::{Http, Middleware, Provider},
};
use std::collections::HashMap;
use std::sync::Arc;

// TODO: tests for all methods on AvsRegistry
// TODO: tests for bitmap utils
// TODO: add eth client for getting current block number

// Implemented methods for parity with go-sdk
// - [ ] GetQuorumCount
// - [] GetOperatorsStakeInQuorumsAtCurrentBlock
// - [x] GetOperatorsStakeInQuorumsAtBlock
// - [] GetOperatorAddrsInQuorumsAtCurrentBlock
// 		- [x] instead did GetOperatorAddrsInQuorumsAtBlock
// - [x] GetOperatorsStakeInQuorumsOfOperatorAtBlock
// - [ ] GetOperatorsStakeInQuorumsOfOperatorAtCurrentBlock
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
    bls_apk_registry: BLSApkRegistry<M>,
    middleware: Arc<M>,
}

impl<M: Middleware + 'static> AvsRegistry<M> {
    /// Create a new instance of [Self].
    pub fn new(
        middleware: Arc<M>,
        operator_state_retriever_addr: Address,
        registry_coordinator_addr: Address,
        stake_registry_addr: Address,
        bls_apk_registry_addr: Address,
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
            bls_apk_registry: BLSApkRegistry::new(bls_apk_registry_addr, middleware.clone()),
            middleware,
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

    /// See OperatorStateRetriever.getCheckSignaturesIndices
    pub async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: ethers::types::Bytes,
        non_signer_operator_ids: Vec<OperatorId>,
    ) -> Result<CheckSignaturesIndices, M> {
        let indices = self
            .operator_stake_retriever
            .get_check_signatures_indices(
                self.registry_coordinator_addr,
                reference_block_number,
                quorum_numbers,
                non_signer_operator_ids,
            )
            .await?;

        Ok(indices)
    }

    /// See RegistryCoordinator.getOperatorId
    pub async fn get_operator_id(&self, operator_addr: Address) -> Result<OperatorId, M> {
        self.registry_coordinator
            .get_operator_id(operator_addr)
            .await
            .map_err(Into::into)
    }

    /// See RegistryCoordinator.getOperatorFromId
    pub async fn get_operator_from_id(&self, operator_id: OperatorId) -> Result<Address, M> {
        self.registry_coordinator
            .get_operator_from_id(operator_id)
            .await
            .map_err(Into::into)
    }

    pub async fn is_operator_registered(&self, operator_addr: Address) -> Result<bool, M> {
        let operatorStatus = self
            .registry_coordinator
            .get_operator_status(operator_addr)
            .await?;

        // 0 = NEVER_REGISTERED, 1 = REGISTERED, 2 = DEREGISTERED
        Ok(operatorStatus == 1)
    }

    pub async fn query_existing_registered_operator_pub_keys(
        &self,
        from_block: u32,
        to_block: u32,
    ) -> Result<Vec<OperatorPubkeys>, M> {
        let logs_with_meta = self
            .bls_apk_registry
            .new_pubkey_registration_filter()
            .from_block(from_block)
            .to_block(to_block)
            // TODO: we could also do stream (polling) or subscribe for pub/sub clients
            .query_with_meta()
            .await?;

        // TODO: clean up
        let mut out = vec![];
        for (log, meta) in logs_with_meta {
            out.push(OperatorPubkeys {
                pubkey_g1: log.pubkey_g1,
                pubkey_g2: log.pubkey_g2,
                address: log.operator,
            });
        }

        Ok(out)
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
