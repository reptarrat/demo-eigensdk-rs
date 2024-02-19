pub use i_eigen_pod::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_eigen_pod {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("activateRestaking"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("activateRestaking"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IEigenPodManager",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRestaked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRestaked"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mostRecentWithdrawalTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "mostRecentWithdrawalTimestamp",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonBeaconChainETHBalanceWei"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "nonBeaconChainETHBalanceWei",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("podOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("podOwner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("provenWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("provenWithdrawal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorPubkeyHash",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recoverTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recoverTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenList"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountsToWithdraw"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("depositDataRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validatorPubkeyHashToInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validatorPubkeyHashToInfo",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorPubkeyHash",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEigenPod.ValidatorInfo",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validatorPubkeyToInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validatorPubkeyToInfo",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorPubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEigenPod.ValidatorInfo",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validatorStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validatorStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorPubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IEigenPod.VALIDATOR_STATUS",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validatorStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IEigenPod.VALIDATOR_STATUS",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyAndProcessWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifyAndProcessWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracleTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stateRootProof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BeaconChainProofs.StateRootProof",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalProofs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BeaconChainProofs.WithdrawalProof[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorFieldsProofs",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorFields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalFields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[][]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyBalanceUpdates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifyBalanceUpdates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracleTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorIndices"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stateRootProof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BeaconChainProofs.StateRootProof",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorFieldsProofs",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorFields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[][]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyWithdrawalCredentials"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifyWithdrawalCredentials",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracleTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stateRootProof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BeaconChainProofs.StateRootProof",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorIndices"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalCredentialProofs",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorFields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[][]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawBeforeRestaking"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawBeforeRestaking",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "withdrawNonBeaconChainETHBalanceWei",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawNonBeaconChainETHBalanceWei",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToWithdraw"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawRestakedBeaconChainETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawRestakedBeaconChainETH",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "withdrawableRestakedExecutionLayerGwei",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawableRestakedExecutionLayerGwei",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EigenPodStaked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EigenPodStaked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FullWithdrawalRedeemed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FullWithdrawalRedeemed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalAmountGwei",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonBeaconChainETHReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NonBeaconChainETHReceived",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountReceived"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonBeaconChainETHWithdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NonBeaconChainETHWithdrawn",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountWithdrawn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PartialWithdrawalRedeemed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PartialWithdrawalRedeemed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "partialWithdrawalAmountGwei",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RestakedBeaconChainETHWithdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RestakedBeaconChainETHWithdrawn",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RestakingActivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RestakingActivated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorBalanceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValidatorBalanceUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("balanceTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newValidatorBalanceGwei",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorRestaked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ValidatorRestaked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IEIGENPOD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IEigenPod<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IEigenPod<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IEigenPod<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IEigenPod<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IEigenPod<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IEigenPod)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IEigenPod<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IEIGENPOD_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR` (0x1d905d5c) function
        pub fn max_restaked_balance_gwei_per_validator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([29, 144, 93, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activateRestaking` (0x0cd4649e) function
        pub fn activate_restaking(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 212, 100, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eigenPodManager` (0x4665bcda) function
        pub fn eigen_pod_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 101, 188, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRestaked` (0x3106ab53) function
        pub fn has_restaked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([49, 6, 171, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mostRecentWithdrawalTimestamp` (0x87e0d289) function
        pub fn most_recent_withdrawal_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([135, 224, 210, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonBeaconChainETHBalanceWei` (0xfe80b087) function
        pub fn non_beacon_chain_eth_balance_wei(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 128, 176, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `podOwner` (0x0b18ff66) function
        pub fn pod_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([11, 24, 255, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `provenWithdrawal` (0x34bea20a) function
        pub fn proven_withdrawal(
            &self,
            validator_pubkey_hash: [u8; 32],
            slot: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([52, 190, 162, 10], (validator_pubkey_hash, slot))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recoverTokens` (0xdda3346c) function
        pub fn recover_tokens(
            &self,
            token_list: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts_to_withdraw: ::std::vec::Vec<::ethers::core::types::U256>,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [221, 163, 52, 108],
                    (token_list, amounts_to_withdraw, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0x9b4e4634) function
        pub fn stake(
            &self,
            pubkey: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
            deposit_data_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 78, 70, 52], (pubkey, signature, deposit_data_root))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorPubkeyHashToInfo` (0x6fcd0e53) function
        pub fn validator_pubkey_hash_to_info(
            &self,
            validator_pubkey_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ValidatorInfo> {
            self.0
                .method_hash([111, 205, 14, 83], validator_pubkey_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorPubkeyToInfo` (0xb522538a) function
        pub fn validator_pubkey_to_info(
            &self,
            validator_pubkey: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ValidatorInfo> {
            self.0
                .method_hash([181, 34, 83, 138], validator_pubkey)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorStatus` (0x58eaee79) function
        pub fn validator_status(
            &self,
            validator_pubkey: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([88, 234, 238, 121], validator_pubkey)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorStatus` (0x7439841f) function
        pub fn validator_status_with_pubkey_hash(
            &self,
            pubkey_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([116, 57, 132, 31], pubkey_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyAndProcessWithdrawals` (0xe251ef52) function
        pub fn verify_and_process_withdrawals(
            &self,
            oracle_timestamp: u64,
            state_root_proof: StateRootProof,
            withdrawal_proofs: ::std::vec::Vec<WithdrawalProof>,
            validator_fields_proofs: ::std::vec::Vec<::ethers::core::types::Bytes>,
            validator_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
            withdrawal_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [226, 81, 239, 82],
                    (
                        oracle_timestamp,
                        state_root_proof,
                        withdrawal_proofs,
                        validator_fields_proofs,
                        validator_fields,
                        withdrawal_fields,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyBalanceUpdates` (0xa50600f4) function
        pub fn verify_balance_updates(
            &self,
            oracle_timestamp: u64,
            validator_indices: ::std::vec::Vec<u64>,
            state_root_proof: StateRootProof,
            validator_fields_proofs: ::std::vec::Vec<::ethers::core::types::Bytes>,
            validator_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [165, 6, 0, 244],
                    (
                        oracle_timestamp,
                        validator_indices,
                        state_root_proof,
                        validator_fields_proofs,
                        validator_fields,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyWithdrawalCredentials` (0x3f65cf19) function
        pub fn verify_withdrawal_credentials(
            &self,
            oracle_timestamp: u64,
            state_root_proof: StateRootProof,
            validator_indices: ::std::vec::Vec<u64>,
            withdrawal_credential_proofs: ::std::vec::Vec<::ethers::core::types::Bytes>,
            validator_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [63, 101, 207, 25],
                    (
                        oracle_timestamp,
                        state_root_proof,
                        validator_indices,
                        withdrawal_credential_proofs,
                        validator_fields,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawBeforeRestaking` (0xbaa7145a) function
        pub fn withdraw_before_restaking(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 167, 20, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawNonBeaconChainETHBalanceWei` (0xe2c83445) function
        pub fn withdraw_non_beacon_chain_eth_balance_wei(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_to_withdraw: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 200, 52, 69], (recipient, amount_to_withdraw))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawRestakedBeaconChainETH` (0xc4907442) function
        pub fn withdraw_restaked_beacon_chain_eth(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 144, 116, 66], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawableRestakedExecutionLayerGwei` (0x3474aa16) function
        pub fn withdrawable_restaked_execution_layer_gwei(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([52, 116, 170, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EigenPodStaked` event
        pub fn eigen_pod_staked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenPodStakedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FullWithdrawalRedeemed` event
        pub fn full_withdrawal_redeemed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FullWithdrawalRedeemedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NonBeaconChainETHReceived` event
        pub fn non_beacon_chain_eth_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NonBeaconChainETHReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NonBeaconChainETHWithdrawn` event
        pub fn non_beacon_chain_eth_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NonBeaconChainETHWithdrawnFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PartialWithdrawalRedeemed` event
        pub fn partial_withdrawal_redeemed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PartialWithdrawalRedeemedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RestakedBeaconChainETHWithdrawn` event
        pub fn restaked_beacon_chain_eth_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RestakedBeaconChainETHWithdrawnFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RestakingActivated` event
        pub fn restaking_activated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RestakingActivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorBalanceUpdated` event
        pub fn validator_balance_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorBalanceUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorRestaked` event
        pub fn validator_restaked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorRestakedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IEigenPodEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IEigenPod<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EigenPodStaked", abi = "EigenPodStaked(bytes)")]
    pub struct EigenPodStakedFilter {
        pub pubkey: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "FullWithdrawalRedeemed",
        abi = "FullWithdrawalRedeemed(uint40,uint64,address,uint64)"
    )]
    pub struct FullWithdrawalRedeemedFilter {
        pub validator_index: u64,
        pub withdrawal_timestamp: u64,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub withdrawal_amount_gwei: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "NonBeaconChainETHReceived",
        abi = "NonBeaconChainETHReceived(uint256)"
    )]
    pub struct NonBeaconChainETHReceivedFilter {
        pub amount_received: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "NonBeaconChainETHWithdrawn",
        abi = "NonBeaconChainETHWithdrawn(address,uint256)"
    )]
    pub struct NonBeaconChainETHWithdrawnFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_withdrawn: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PartialWithdrawalRedeemed",
        abi = "PartialWithdrawalRedeemed(uint40,uint64,address,uint64)"
    )]
    pub struct PartialWithdrawalRedeemedFilter {
        pub validator_index: u64,
        pub withdrawal_timestamp: u64,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub partial_withdrawal_amount_gwei: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RestakedBeaconChainETHWithdrawn",
        abi = "RestakedBeaconChainETHWithdrawn(address,uint256)"
    )]
    pub struct RestakedBeaconChainETHWithdrawnFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RestakingActivated", abi = "RestakingActivated(address)")]
    pub struct RestakingActivatedFilter {
        #[ethevent(indexed)]
        pub pod_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ValidatorBalanceUpdated",
        abi = "ValidatorBalanceUpdated(uint40,uint64,uint64)"
    )]
    pub struct ValidatorBalanceUpdatedFilter {
        pub validator_index: u64,
        pub balance_timestamp: u64,
        pub new_validator_balance_gwei: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ValidatorRestaked", abi = "ValidatorRestaked(uint40)")]
    pub struct ValidatorRestakedFilter {
        pub validator_index: u64,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEigenPodEvents {
        EigenPodStakedFilter(EigenPodStakedFilter),
        FullWithdrawalRedeemedFilter(FullWithdrawalRedeemedFilter),
        NonBeaconChainETHReceivedFilter(NonBeaconChainETHReceivedFilter),
        NonBeaconChainETHWithdrawnFilter(NonBeaconChainETHWithdrawnFilter),
        PartialWithdrawalRedeemedFilter(PartialWithdrawalRedeemedFilter),
        RestakedBeaconChainETHWithdrawnFilter(RestakedBeaconChainETHWithdrawnFilter),
        RestakingActivatedFilter(RestakingActivatedFilter),
        ValidatorBalanceUpdatedFilter(ValidatorBalanceUpdatedFilter),
        ValidatorRestakedFilter(ValidatorRestakedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IEigenPodEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EigenPodStakedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::EigenPodStakedFilter(decoded));
            }
            if let Ok(decoded) = FullWithdrawalRedeemedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::FullWithdrawalRedeemedFilter(decoded));
            }
            if let Ok(decoded) = NonBeaconChainETHReceivedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::NonBeaconChainETHReceivedFilter(decoded));
            }
            if let Ok(decoded) = NonBeaconChainETHWithdrawnFilter::decode_log(log) {
                return Ok(IEigenPodEvents::NonBeaconChainETHWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = PartialWithdrawalRedeemedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::PartialWithdrawalRedeemedFilter(decoded));
            }
            if let Ok(decoded) = RestakedBeaconChainETHWithdrawnFilter::decode_log(log) {
                return Ok(
                    IEigenPodEvents::RestakedBeaconChainETHWithdrawnFilter(decoded),
                );
            }
            if let Ok(decoded) = RestakingActivatedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::RestakingActivatedFilter(decoded));
            }
            if let Ok(decoded) = ValidatorBalanceUpdatedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::ValidatorBalanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ValidatorRestakedFilter::decode_log(log) {
                return Ok(IEigenPodEvents::ValidatorRestakedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IEigenPodEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EigenPodStakedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FullWithdrawalRedeemedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonBeaconChainETHReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonBeaconChainETHWithdrawnFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PartialWithdrawalRedeemedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RestakedBeaconChainETHWithdrawnFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RestakingActivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorBalanceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorRestakedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EigenPodStakedFilter> for IEigenPodEvents {
        fn from(value: EigenPodStakedFilter) -> Self {
            Self::EigenPodStakedFilter(value)
        }
    }
    impl ::core::convert::From<FullWithdrawalRedeemedFilter> for IEigenPodEvents {
        fn from(value: FullWithdrawalRedeemedFilter) -> Self {
            Self::FullWithdrawalRedeemedFilter(value)
        }
    }
    impl ::core::convert::From<NonBeaconChainETHReceivedFilter> for IEigenPodEvents {
        fn from(value: NonBeaconChainETHReceivedFilter) -> Self {
            Self::NonBeaconChainETHReceivedFilter(value)
        }
    }
    impl ::core::convert::From<NonBeaconChainETHWithdrawnFilter> for IEigenPodEvents {
        fn from(value: NonBeaconChainETHWithdrawnFilter) -> Self {
            Self::NonBeaconChainETHWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<PartialWithdrawalRedeemedFilter> for IEigenPodEvents {
        fn from(value: PartialWithdrawalRedeemedFilter) -> Self {
            Self::PartialWithdrawalRedeemedFilter(value)
        }
    }
    impl ::core::convert::From<RestakedBeaconChainETHWithdrawnFilter>
    for IEigenPodEvents {
        fn from(value: RestakedBeaconChainETHWithdrawnFilter) -> Self {
            Self::RestakedBeaconChainETHWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<RestakingActivatedFilter> for IEigenPodEvents {
        fn from(value: RestakingActivatedFilter) -> Self {
            Self::RestakingActivatedFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorBalanceUpdatedFilter> for IEigenPodEvents {
        fn from(value: ValidatorBalanceUpdatedFilter) -> Self {
            Self::ValidatorBalanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorRestakedFilter> for IEigenPodEvents {
        fn from(value: ValidatorRestakedFilter) -> Self {
            Self::ValidatorRestakedFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR` function with signature `MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR()` and selector `0x1d905d5c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR",
        abi = "MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR()"
    )]
    pub struct MaxRestakedBalanceGweiPerValidatorCall;
    ///Container type for all input parameters for the `activateRestaking` function with signature `activateRestaking()` and selector `0x0cd4649e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "activateRestaking", abi = "activateRestaking()")]
    pub struct ActivateRestakingCall;
    ///Container type for all input parameters for the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eigenPodManager", abi = "eigenPodManager()")]
    pub struct EigenPodManagerCall;
    ///Container type for all input parameters for the `hasRestaked` function with signature `hasRestaked()` and selector `0x3106ab53`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasRestaked", abi = "hasRestaked()")]
    pub struct HasRestakedCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mostRecentWithdrawalTimestamp` function with signature `mostRecentWithdrawalTimestamp()` and selector `0x87e0d289`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "mostRecentWithdrawalTimestamp",
        abi = "mostRecentWithdrawalTimestamp()"
    )]
    pub struct MostRecentWithdrawalTimestampCall;
    ///Container type for all input parameters for the `nonBeaconChainETHBalanceWei` function with signature `nonBeaconChainETHBalanceWei()` and selector `0xfe80b087`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "nonBeaconChainETHBalanceWei",
        abi = "nonBeaconChainETHBalanceWei()"
    )]
    pub struct NonBeaconChainETHBalanceWeiCall;
    ///Container type for all input parameters for the `podOwner` function with signature `podOwner()` and selector `0x0b18ff66`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "podOwner", abi = "podOwner()")]
    pub struct PodOwnerCall;
    ///Container type for all input parameters for the `provenWithdrawal` function with signature `provenWithdrawal(bytes32,uint64)` and selector `0x34bea20a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "provenWithdrawal", abi = "provenWithdrawal(bytes32,uint64)")]
    pub struct ProvenWithdrawalCall {
        pub validator_pubkey_hash: [u8; 32],
        pub slot: u64,
    }
    ///Container type for all input parameters for the `recoverTokens` function with signature `recoverTokens(address[],uint256[],address)` and selector `0xdda3346c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "recoverTokens",
        abi = "recoverTokens(address[],uint256[],address)"
    )]
    pub struct RecoverTokensCall {
        pub token_list: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts_to_withdraw: ::std::vec::Vec<::ethers::core::types::U256>,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(bytes,bytes,bytes32)` and selector `0x9b4e4634`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stake", abi = "stake(bytes,bytes,bytes32)")]
    pub struct StakeCall {
        pub pubkey: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub deposit_data_root: [u8; 32],
    }
    ///Container type for all input parameters for the `validatorPubkeyHashToInfo` function with signature `validatorPubkeyHashToInfo(bytes32)` and selector `0x6fcd0e53`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validatorPubkeyHashToInfo",
        abi = "validatorPubkeyHashToInfo(bytes32)"
    )]
    pub struct ValidatorPubkeyHashToInfoCall {
        pub validator_pubkey_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `validatorPubkeyToInfo` function with signature `validatorPubkeyToInfo(bytes)` and selector `0xb522538a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "validatorPubkeyToInfo", abi = "validatorPubkeyToInfo(bytes)")]
    pub struct ValidatorPubkeyToInfoCall {
        pub validator_pubkey: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validatorStatus` function with signature `validatorStatus(bytes)` and selector `0x58eaee79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "validatorStatus", abi = "validatorStatus(bytes)")]
    pub struct ValidatorStatusCall {
        pub validator_pubkey: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validatorStatus` function with signature `validatorStatus(bytes32)` and selector `0x7439841f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "validatorStatus", abi = "validatorStatus(bytes32)")]
    pub struct ValidatorStatusWithPubkeyHashCall {
        pub pubkey_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `verifyAndProcessWithdrawals` function with signature `verifyAndProcessWithdrawals(uint64,(bytes32,bytes),(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32)[],bytes[],bytes32[][],bytes32[][])` and selector `0xe251ef52`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifyAndProcessWithdrawals",
        abi = "verifyAndProcessWithdrawals(uint64,(bytes32,bytes),(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32)[],bytes[],bytes32[][],bytes32[][])"
    )]
    pub struct VerifyAndProcessWithdrawalsCall {
        pub oracle_timestamp: u64,
        pub state_root_proof: StateRootProof,
        pub withdrawal_proofs: ::std::vec::Vec<WithdrawalProof>,
        pub validator_fields_proofs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub validator_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
        pub withdrawal_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
    }
    ///Container type for all input parameters for the `verifyBalanceUpdates` function with signature `verifyBalanceUpdates(uint64,uint40[],(bytes32,bytes),bytes[],bytes32[][])` and selector `0xa50600f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifyBalanceUpdates",
        abi = "verifyBalanceUpdates(uint64,uint40[],(bytes32,bytes),bytes[],bytes32[][])"
    )]
    pub struct VerifyBalanceUpdatesCall {
        pub oracle_timestamp: u64,
        pub validator_indices: ::std::vec::Vec<u64>,
        pub state_root_proof: StateRootProof,
        pub validator_fields_proofs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub validator_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
    }
    ///Container type for all input parameters for the `verifyWithdrawalCredentials` function with signature `verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])` and selector `0x3f65cf19`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifyWithdrawalCredentials",
        abi = "verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])"
    )]
    pub struct VerifyWithdrawalCredentialsCall {
        pub oracle_timestamp: u64,
        pub state_root_proof: StateRootProof,
        pub validator_indices: ::std::vec::Vec<u64>,
        pub withdrawal_credential_proofs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub validator_fields: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
    }
    ///Container type for all input parameters for the `withdrawBeforeRestaking` function with signature `withdrawBeforeRestaking()` and selector `0xbaa7145a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawBeforeRestaking", abi = "withdrawBeforeRestaking()")]
    pub struct WithdrawBeforeRestakingCall;
    ///Container type for all input parameters for the `withdrawNonBeaconChainETHBalanceWei` function with signature `withdrawNonBeaconChainETHBalanceWei(address,uint256)` and selector `0xe2c83445`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawNonBeaconChainETHBalanceWei",
        abi = "withdrawNonBeaconChainETHBalanceWei(address,uint256)"
    )]
    pub struct WithdrawNonBeaconChainETHBalanceWeiCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_to_withdraw: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawRestakedBeaconChainETH` function with signature `withdrawRestakedBeaconChainETH(address,uint256)` and selector `0xc4907442`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawRestakedBeaconChainETH",
        abi = "withdrawRestakedBeaconChainETH(address,uint256)"
    )]
    pub struct WithdrawRestakedBeaconChainETHCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawableRestakedExecutionLayerGwei` function with signature `withdrawableRestakedExecutionLayerGwei()` and selector `0x3474aa16`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawableRestakedExecutionLayerGwei",
        abi = "withdrawableRestakedExecutionLayerGwei()"
    )]
    pub struct WithdrawableRestakedExecutionLayerGweiCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEigenPodCalls {
        MaxRestakedBalanceGweiPerValidator(MaxRestakedBalanceGweiPerValidatorCall),
        ActivateRestaking(ActivateRestakingCall),
        EigenPodManager(EigenPodManagerCall),
        HasRestaked(HasRestakedCall),
        Initialize(InitializeCall),
        MostRecentWithdrawalTimestamp(MostRecentWithdrawalTimestampCall),
        NonBeaconChainETHBalanceWei(NonBeaconChainETHBalanceWeiCall),
        PodOwner(PodOwnerCall),
        ProvenWithdrawal(ProvenWithdrawalCall),
        RecoverTokens(RecoverTokensCall),
        Stake(StakeCall),
        ValidatorPubkeyHashToInfo(ValidatorPubkeyHashToInfoCall),
        ValidatorPubkeyToInfo(ValidatorPubkeyToInfoCall),
        ValidatorStatus(ValidatorStatusCall),
        ValidatorStatusWithPubkeyHash(ValidatorStatusWithPubkeyHashCall),
        VerifyAndProcessWithdrawals(VerifyAndProcessWithdrawalsCall),
        VerifyBalanceUpdates(VerifyBalanceUpdatesCall),
        VerifyWithdrawalCredentials(VerifyWithdrawalCredentialsCall),
        WithdrawBeforeRestaking(WithdrawBeforeRestakingCall),
        WithdrawNonBeaconChainETHBalanceWei(WithdrawNonBeaconChainETHBalanceWeiCall),
        WithdrawRestakedBeaconChainETH(WithdrawRestakedBeaconChainETHCall),
        WithdrawableRestakedExecutionLayerGwei(
            WithdrawableRestakedExecutionLayerGweiCall,
        ),
    }
    impl ::ethers::core::abi::AbiDecode for IEigenPodCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxRestakedBalanceGweiPerValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxRestakedBalanceGweiPerValidator(decoded));
            }
            if let Ok(decoded) = <ActivateRestakingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActivateRestaking(decoded));
            }
            if let Ok(decoded) = <EigenPodManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EigenPodManager(decoded));
            }
            if let Ok(decoded) = <HasRestakedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRestaked(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MostRecentWithdrawalTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MostRecentWithdrawalTimestamp(decoded));
            }
            if let Ok(decoded) = <NonBeaconChainETHBalanceWeiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NonBeaconChainETHBalanceWei(decoded));
            }
            if let Ok(decoded) = <PodOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PodOwner(decoded));
            }
            if let Ok(decoded) = <ProvenWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProvenWithdrawal(decoded));
            }
            if let Ok(decoded) = <RecoverTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecoverTokens(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <ValidatorPubkeyHashToInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorPubkeyHashToInfo(decoded));
            }
            if let Ok(decoded) = <ValidatorPubkeyToInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorPubkeyToInfo(decoded));
            }
            if let Ok(decoded) = <ValidatorStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorStatus(decoded));
            }
            if let Ok(decoded) = <ValidatorStatusWithPubkeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorStatusWithPubkeyHash(decoded));
            }
            if let Ok(decoded) = <VerifyAndProcessWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyAndProcessWithdrawals(decoded));
            }
            if let Ok(decoded) = <VerifyBalanceUpdatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyBalanceUpdates(decoded));
            }
            if let Ok(decoded) = <VerifyWithdrawalCredentialsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyWithdrawalCredentials(decoded));
            }
            if let Ok(decoded) = <WithdrawBeforeRestakingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawBeforeRestaking(decoded));
            }
            if let Ok(decoded) = <WithdrawNonBeaconChainETHBalanceWeiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawNonBeaconChainETHBalanceWei(decoded));
            }
            if let Ok(decoded) = <WithdrawRestakedBeaconChainETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawRestakedBeaconChainETH(decoded));
            }
            if let Ok(decoded) = <WithdrawableRestakedExecutionLayerGweiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawableRestakedExecutionLayerGwei(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IEigenPodCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxRestakedBalanceGweiPerValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ActivateRestaking(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EigenPodManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRestaked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MostRecentWithdrawalTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonBeaconChainETHBalanceWei(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PodOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProvenWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecoverTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidatorPubkeyHashToInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorPubkeyToInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorStatusWithPubkeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyAndProcessWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyBalanceUpdates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyWithdrawalCredentials(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawBeforeRestaking(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawNonBeaconChainETHBalanceWei(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawRestakedBeaconChainETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawableRestakedExecutionLayerGwei(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IEigenPodCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxRestakedBalanceGweiPerValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ActivateRestaking(element) => ::core::fmt::Display::fmt(element, f),
                Self::EigenPodManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRestaked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MostRecentWithdrawalTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonBeaconChainETHBalanceWei(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PodOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProvenWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecoverTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatorPubkeyHashToInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorPubkeyToInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatorStatusWithPubkeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyAndProcessWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyBalanceUpdates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyWithdrawalCredentials(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawBeforeRestaking(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawNonBeaconChainETHBalanceWei(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawRestakedBeaconChainETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawableRestakedExecutionLayerGwei(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MaxRestakedBalanceGweiPerValidatorCall>
    for IEigenPodCalls {
        fn from(value: MaxRestakedBalanceGweiPerValidatorCall) -> Self {
            Self::MaxRestakedBalanceGweiPerValidator(value)
        }
    }
    impl ::core::convert::From<ActivateRestakingCall> for IEigenPodCalls {
        fn from(value: ActivateRestakingCall) -> Self {
            Self::ActivateRestaking(value)
        }
    }
    impl ::core::convert::From<EigenPodManagerCall> for IEigenPodCalls {
        fn from(value: EigenPodManagerCall) -> Self {
            Self::EigenPodManager(value)
        }
    }
    impl ::core::convert::From<HasRestakedCall> for IEigenPodCalls {
        fn from(value: HasRestakedCall) -> Self {
            Self::HasRestaked(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IEigenPodCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MostRecentWithdrawalTimestampCall> for IEigenPodCalls {
        fn from(value: MostRecentWithdrawalTimestampCall) -> Self {
            Self::MostRecentWithdrawalTimestamp(value)
        }
    }
    impl ::core::convert::From<NonBeaconChainETHBalanceWeiCall> for IEigenPodCalls {
        fn from(value: NonBeaconChainETHBalanceWeiCall) -> Self {
            Self::NonBeaconChainETHBalanceWei(value)
        }
    }
    impl ::core::convert::From<PodOwnerCall> for IEigenPodCalls {
        fn from(value: PodOwnerCall) -> Self {
            Self::PodOwner(value)
        }
    }
    impl ::core::convert::From<ProvenWithdrawalCall> for IEigenPodCalls {
        fn from(value: ProvenWithdrawalCall) -> Self {
            Self::ProvenWithdrawal(value)
        }
    }
    impl ::core::convert::From<RecoverTokensCall> for IEigenPodCalls {
        fn from(value: RecoverTokensCall) -> Self {
            Self::RecoverTokens(value)
        }
    }
    impl ::core::convert::From<StakeCall> for IEigenPodCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<ValidatorPubkeyHashToInfoCall> for IEigenPodCalls {
        fn from(value: ValidatorPubkeyHashToInfoCall) -> Self {
            Self::ValidatorPubkeyHashToInfo(value)
        }
    }
    impl ::core::convert::From<ValidatorPubkeyToInfoCall> for IEigenPodCalls {
        fn from(value: ValidatorPubkeyToInfoCall) -> Self {
            Self::ValidatorPubkeyToInfo(value)
        }
    }
    impl ::core::convert::From<ValidatorStatusCall> for IEigenPodCalls {
        fn from(value: ValidatorStatusCall) -> Self {
            Self::ValidatorStatus(value)
        }
    }
    impl ::core::convert::From<ValidatorStatusWithPubkeyHashCall> for IEigenPodCalls {
        fn from(value: ValidatorStatusWithPubkeyHashCall) -> Self {
            Self::ValidatorStatusWithPubkeyHash(value)
        }
    }
    impl ::core::convert::From<VerifyAndProcessWithdrawalsCall> for IEigenPodCalls {
        fn from(value: VerifyAndProcessWithdrawalsCall) -> Self {
            Self::VerifyAndProcessWithdrawals(value)
        }
    }
    impl ::core::convert::From<VerifyBalanceUpdatesCall> for IEigenPodCalls {
        fn from(value: VerifyBalanceUpdatesCall) -> Self {
            Self::VerifyBalanceUpdates(value)
        }
    }
    impl ::core::convert::From<VerifyWithdrawalCredentialsCall> for IEigenPodCalls {
        fn from(value: VerifyWithdrawalCredentialsCall) -> Self {
            Self::VerifyWithdrawalCredentials(value)
        }
    }
    impl ::core::convert::From<WithdrawBeforeRestakingCall> for IEigenPodCalls {
        fn from(value: WithdrawBeforeRestakingCall) -> Self {
            Self::WithdrawBeforeRestaking(value)
        }
    }
    impl ::core::convert::From<WithdrawNonBeaconChainETHBalanceWeiCall>
    for IEigenPodCalls {
        fn from(value: WithdrawNonBeaconChainETHBalanceWeiCall) -> Self {
            Self::WithdrawNonBeaconChainETHBalanceWei(value)
        }
    }
    impl ::core::convert::From<WithdrawRestakedBeaconChainETHCall> for IEigenPodCalls {
        fn from(value: WithdrawRestakedBeaconChainETHCall) -> Self {
            Self::WithdrawRestakedBeaconChainETH(value)
        }
    }
    impl ::core::convert::From<WithdrawableRestakedExecutionLayerGweiCall>
    for IEigenPodCalls {
        fn from(value: WithdrawableRestakedExecutionLayerGweiCall) -> Self {
            Self::WithdrawableRestakedExecutionLayerGwei(value)
        }
    }
    ///Container type for all return fields from the `MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR` function with signature `MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR()` and selector `0x1d905d5c`
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
    pub struct MaxRestakedBalanceGweiPerValidatorReturn(pub u64);
    ///Container type for all return fields from the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    pub struct EigenPodManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasRestaked` function with signature `hasRestaked()` and selector `0x3106ab53`
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
    pub struct HasRestakedReturn(pub bool);
    ///Container type for all return fields from the `mostRecentWithdrawalTimestamp` function with signature `mostRecentWithdrawalTimestamp()` and selector `0x87e0d289`
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
    pub struct MostRecentWithdrawalTimestampReturn(pub u64);
    ///Container type for all return fields from the `nonBeaconChainETHBalanceWei` function with signature `nonBeaconChainETHBalanceWei()` and selector `0xfe80b087`
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
    pub struct NonBeaconChainETHBalanceWeiReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `podOwner` function with signature `podOwner()` and selector `0x0b18ff66`
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
    pub struct PodOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `provenWithdrawal` function with signature `provenWithdrawal(bytes32,uint64)` and selector `0x34bea20a`
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
    pub struct ProvenWithdrawalReturn(pub bool);
    ///Container type for all return fields from the `validatorPubkeyHashToInfo` function with signature `validatorPubkeyHashToInfo(bytes32)` and selector `0x6fcd0e53`
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
    pub struct ValidatorPubkeyHashToInfoReturn(pub ValidatorInfo);
    ///Container type for all return fields from the `validatorPubkeyToInfo` function with signature `validatorPubkeyToInfo(bytes)` and selector `0xb522538a`
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
    pub struct ValidatorPubkeyToInfoReturn(pub ValidatorInfo);
    ///Container type for all return fields from the `validatorStatus` function with signature `validatorStatus(bytes)` and selector `0x58eaee79`
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
    pub struct ValidatorStatusReturn(pub u8);
    ///Container type for all return fields from the `validatorStatus` function with signature `validatorStatus(bytes32)` and selector `0x7439841f`
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
    pub struct ValidatorStatusWithPubkeyHashReturn(pub u8);
    ///Container type for all return fields from the `withdrawableRestakedExecutionLayerGwei` function with signature `withdrawableRestakedExecutionLayerGwei()` and selector `0x3474aa16`
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
    pub struct WithdrawableRestakedExecutionLayerGweiReturn(pub u64);
}
