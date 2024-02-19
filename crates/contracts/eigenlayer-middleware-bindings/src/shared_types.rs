///`G1Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct G1Point {
    pub x: ::ethers::core::types::U256,
    pub y: ::ethers::core::types::U256,
}
///`G2Point(uint256[2],uint256[2])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct G2Point {
    pub x: [::ethers::core::types::U256; 2],
    pub y: [::ethers::core::types::U256; 2],
}
///`StateRootProof(bytes32,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct StateRootProof {
    pub beacon_state_root: [u8; 32],
    pub proof: ::ethers::core::types::Bytes,
}
///`WithdrawalProof(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct WithdrawalProof {
    pub withdrawal_proof: ::ethers::core::types::Bytes,
    pub slot_proof: ::ethers::core::types::Bytes,
    pub execution_payload_proof: ::ethers::core::types::Bytes,
    pub timestamp_proof: ::ethers::core::types::Bytes,
    pub historical_summary_block_root_proof: ::ethers::core::types::Bytes,
    pub block_root_index: u64,
    pub historical_summary_index: u64,
    pub withdrawal_index: u64,
    pub block_root: [u8; 32],
    pub slot_root: [u8; 32],
    pub timestamp_root: [u8; 32],
    pub execution_payload_root: [u8; 32],
}
///`ApkUpdate(bytes24,uint32,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ApkUpdate {
    pub apk_hash: [u8; 24],
    pub update_block_number: u32,
    pub next_update_block_number: u32,
}
///`PubkeyRegistrationParams((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2]))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PubkeyRegistrationParams {
    pub pubkey_registration_signature: G1Point,
    pub pubkey_g1: G1Point,
    pub pubkey_g2: G2Point,
}
///`NonSignerStakesAndSignature(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct NonSignerStakesAndSignature {
    pub non_signer_quorum_bitmap_indices: ::std::vec::Vec<u32>,
    pub non_signer_pubkeys: ::std::vec::Vec<G1Point>,
    pub quorum_apks: ::std::vec::Vec<G1Point>,
    pub apk_g2: G2Point,
    pub sigma: G1Point,
    pub quorum_apk_indices: ::std::vec::Vec<u32>,
    pub total_stake_indices: ::std::vec::Vec<u32>,
    pub non_signer_stake_indices: ::std::vec::Vec<::std::vec::Vec<u32>>,
}
///`QuorumStakeTotals(uint96[],uint96[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct QuorumStakeTotals {
    pub signed_stake_for_quorum: ::std::vec::Vec<u128>,
    pub total_stake_for_quorum: ::std::vec::Vec<u128>,
}
///`DelayedWithdrawal(uint224,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DelayedWithdrawal {
    pub amount: ::ethers::core::types::U256,
    pub block_created: u32,
}
///`UserDelayedWithdrawals(uint256,(uint224,uint32)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct UserDelayedWithdrawals {
    pub delayed_withdrawals_completed: ::ethers::core::types::U256,
    pub delayed_withdrawals: ::std::vec::Vec<DelayedWithdrawal>,
}
///`OperatorDetails(address,address,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OperatorDetails {
    pub earnings_receiver: ::ethers::core::types::Address,
    pub delegation_approver: ::ethers::core::types::Address,
    pub staker_opt_out_window_blocks: u32,
}
///`QueuedWithdrawalParams(address[],uint256[],address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct QueuedWithdrawalParams {
    pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    pub withdrawer: ::ethers::core::types::Address,
}
///`Withdrawal(address,address,address,uint256,uint32,address[],uint256[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Withdrawal {
    pub staker: ::ethers::core::types::Address,
    pub delegated_to: ::ethers::core::types::Address,
    pub withdrawer: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub start_block: u32,
    pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
}
///`ValidatorInfo(uint64,uint64,uint64,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ValidatorInfo {
    pub validator_index: u64,
    pub restaked_balance_gwei: u64,
    pub most_recent_balance_update_timestamp: u64,
    pub status: u8,
}
///`OperatorUpdate(uint32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OperatorUpdate {
    pub from_block_number: u32,
    pub operator_id: [u8; 32],
}
///`QuorumUpdate(uint32,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct QuorumUpdate {
    pub from_block_number: u32,
    pub num_operators: u32,
}
///`OperatorInfo(bytes32,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OperatorInfo {
    pub operator_id: [u8; 32],
    pub status: u8,
}
///`OperatorKickParam(uint8,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OperatorKickParam {
    pub quorum_number: u8,
    pub operator: ::ethers::core::types::Address,
}
///`OperatorSetParam(uint32,uint16,uint16)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OperatorSetParam {
    pub max_operator_count: u32,
    pub kick_bi_ps_of_operator_stake: u16,
    pub kick_bi_ps_of_total_stake: u16,
}
///`QuorumBitmapUpdate(uint32,uint32,uint192)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct QuorumBitmapUpdate {
    pub update_block_number: u32,
    pub next_update_block_number: u32,
    pub quorum_bitmap: ::ethers::core::types::U256,
}
///`SignatureWithExpiry(bytes,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SignatureWithExpiry {
    pub signature: ::ethers::core::types::Bytes,
    pub expiry: ::ethers::core::types::U256,
}
///`SignatureWithSaltAndExpiry(bytes,bytes32,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SignatureWithSaltAndExpiry {
    pub signature: ::ethers::core::types::Bytes,
    pub salt: [u8; 32],
    pub expiry: ::ethers::core::types::U256,
}
///`MiddlewareTimes(uint32,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MiddlewareTimes {
    pub stalest_update_block: u32,
    pub latest_serve_until_block: u32,
}
///`StakeUpdate(uint32,uint32,uint96)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct StakeUpdate {
    pub update_block_number: u32,
    pub next_update_block_number: u32,
    pub stake: u128,
}
///`StrategyParams(address,uint96)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct StrategyParams {
    pub strategy: ::ethers::core::types::Address,
    pub multiplier: u128,
}
///`DeprecatedStructQueuedWithdrawal(address[],uint256[],address,(address,uint96),uint32,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DeprecatedStructQueuedWithdrawal {
    pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    pub staker: ::ethers::core::types::Address,
    pub withdrawer_and_nonce: DeprecatedStructWithdrawerAndNonce,
    pub withdrawal_start_block: u32,
    pub delegated_address: ::ethers::core::types::Address,
}
///`DeprecatedStructWithdrawerAndNonce(address,uint96)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DeprecatedStructWithdrawerAndNonce {
    pub withdrawer: ::ethers::core::types::Address,
    pub nonce: u128,
}
///`FuzzInterface(address,string[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzInterface {
    pub addr: ::ethers::core::types::Address,
    pub artifacts: ::std::vec::Vec<::std::string::String>,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
