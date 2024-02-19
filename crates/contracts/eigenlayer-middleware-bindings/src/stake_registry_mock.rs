pub use stake_registry_mock::*;
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
pub mod stake_registry_mock {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WEIGHTING_DIVISOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WEIGHTING_DIVISOR"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addStrategies"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StrategyParams[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("delegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IDelegationManager",
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
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("getCurrentStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCurrentStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentTotalStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentTotalStake",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLatestStakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestStakeUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StakeUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getMockOperatorId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMockOperatorId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumberAndIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeAtBlockNumberAndIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeHistory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStakeHistory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StakeUpdate[]",
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
                    ::std::borrow::ToOwned::to_owned("getStakeUpdateAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeUpdateAtIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StakeUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getStakeUpdateIndexAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeUpdateIndexAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getTotalStakeAtBlockNumberFromIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeAtBlockNumberFromIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeHistoryLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeHistoryLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned(
                        "getTotalStakeIndicesAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeIndicesAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeUpdateAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeUpdateAtIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StakeUpdate",
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
                    ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StrategyParams[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("minimumStakeForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minimumStakeForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("modifyStrategyParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "modifyStrategyParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyIndices"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newMultipliers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96[]"),
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
                    ::std::borrow::ToOwned::to_owned("registerOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registryCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registryCoordinator",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("removeStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeStrategies"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("indicesToRemove"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                        "set_updateOperatorStakeReturnBitmap",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "set_updateOperatorStakeReturnBitmap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
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
                    ::std::borrow::ToOwned::to_owned("strategyParamsByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyParamsByIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StrategyParams",
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
                    ::std::borrow::ToOwned::to_owned("strategyParamsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyParamsLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("updateOperatorStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateOperatorStake",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightOfOperatorForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "weightOfOperatorForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
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
                    ::std::borrow::ToOwned::to_owned("MinimumStakeForQuorumUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinimumStakeForQuorumUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorStakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorStakeUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("stake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QuorumCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("QuorumCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyAddedToQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyAddedToQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyMultiplierUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyMultiplierUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("multiplier"),
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
                    ::std::borrow::ToOwned::to_owned("StrategyRemovedFromQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyRemovedFromQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static STAKEREGISTRYMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\r\xAD\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80c\xACk\xFB\x03\x11a\0\xF9W\x80c\xD5\xEC\xCC\x05\x11a\0\x97W\x80c\xF2\xBE\x94\xAE\x11a\0qW\x80c\xF2\xBE\x94\xAE\x14a\x04\x80W\x80c\xF8Q\xE1\x98\x14a\x04\x98W\x80c\xFA(\xC6'\x14a\x04\xA6W\x80c\xFFiJw\x14a\x04\xB4W`\0\x80\xFD[\x80c\xD5\xEC\xCC\x05\x14a\x04*W\x80c\xDD\x98F\xB9\x14a\x04]W\x80c\xDF\\\xF7#\x14a\x03rW`\0\x80\xFD[\x80c\xBD)\xB8\xCD\x11a\0\xD3W\x80c\xBD)\xB8\xCD\x14a\x04\x18W\x80c\xC4gx\xA5\x14a\x04*W\x80c\xC6\x01R}\x14a\x048W\x80c\xC8)LV\x14a\x04FW`\0\x80\xFD[\x80c\xACk\xFB\x03\x14a\x03\xA5W\x80c\xAD\xC8\x04\xDA\x14a\x03\xC5W\x80c\xB6\x90Kx\x14a\x04\x05W`\0\x80\xFD[\x80cLQ\xBF\x91\x11a\x01fW\x80c_\x1F-w\x11a\x01@W\x80c_\x1F-w\x14a\x03%W\x80cf\xAC\xFE\xFE\x14a\x038W\x80cm\x14\xA9\x87\x14a\x03rW\x80c\x81\xC0u\x02\x14a\x03\x81W`\0\x80\xFD[\x80cLQ\xBF\x91\x14a\x02\xE0W\x80cT\x01\xED'\x14a\x03\x10W\x80c^Zgu\x14a\x03\x1EW`\0\x80\xFD[\x80c\"J<9\x11a\x01\xA2W\x80c\"J<9\x14a\x025W\x80c%PGw\x14a\x02\x96W\x80c,\xD9Y@\x14a\x02\xBDW\x80c<\xA5\xA5\xF5\x14a\x01\xC9W`\0\x80\xFD[\x80c\x04\x91\xB4\x1C\x14a\x01\xC9W\x80c\x1F\x9Bt\xE0\x14a\x01\xF0W\x80c \xB6b\x98\x14a\x02\x1EW[`\0\x80\xFD[a\x01\xDDa\x01\xD76`\x04a\x05:V[P`\0\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x06a\x01\xFE6`\x04a\x05mV[`\0\x92\x91PPV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE7V[a\x023a\x02,6`\x04a\x05\xEFV[PPPPPV[\0[a\x01\xDDa\x02C6`\x04a\x06oV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01Ri\x1B\xDC\x19\\\x98]\x1B\xDC\x92Y`\xB2\x1B`4\x82\x01R`\0\x90`>\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x91PPV[a\x02\xAFa\x02\xA46`\x04a\x07iV[``\x80\x93P\x93\x91PPV[`@Qa\x01\xE7\x92\x91\x90a\x08\x05V[a\x02\xD3a\x02\xCB6`\x04a\x083V[``\x92\x91PPV[`@Qa\x01\xE7\x91\x90a\x08_V[a\x023a\x02\xEE6`\x04a\x08\xD7V[`\0\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\x01`\xC0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\x06a\x01\xFE6`\x04a\x083V[`\0a\x01\xDDV[a\x023a\x0336`\x04a\t\0V[PPPV[a\x03Za\x03F6`\x04a\t\x93V[`\0T`\x01`\x01`\xC0\x1B\x03\x16\x94\x93PPPPV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE7V[`@Q`\0\x81R` \x01a\x01\xE7V[a\x03\x98a\x03\x8F6`\x04a\n\x02V[``\x93\x92PPPV[`@Qa\x01\xE7\x91\x90a\nGV[a\x03\xB8a\x03\xB36`\x04a\n\x85V[a\x04\xC2V[`@Qa\x01\xE7\x91\x90a\n\xB8V[a\x03\xD8a\x03\xD36`\x04a\n\xEDV[a\x04\xE7V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xE7V[a\x03\xB8a\x04\x136`\x04a\n\xEDV[a\x05\x02V[a\x023a\x04&6`\x04a\x0B\x17V[PPV[a\x02\x06a\x01\xD76`\x04a\x05:V[a\x023a\x04&6`\x04a\x0C'V[a\x02\x06a\x04T6`\x04a\x0CjV[`\0\x93\x92PPPV[a\x04ka\x04T6`\x04a\x0C\xA6V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE7V[a\x02\x06a\x04\x8E6`\x04a\x0C\xE2V[`\0\x94\x93PPPPV[a\x03\xB8a\x04\x136`\x04a\x083V[a\x02\x06a\x04T6`\x04a\x0C\xA6V[a\x023a\x0336`\x04a\r$V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x04\xFCV[\x805`\xFF\x81\x16\x81\x14a\x055W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05LW`\0\x80\xFD[a\x04\xE0\x82a\x05$V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05jW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\x80W`\0\x80\xFD[a\x05\x89\x83a\x05$V[\x91P` \x83\x015a\x05\x99\x81a\x05UV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x05\xB6W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xCDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x05\xE8W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x06\x07W`\0\x80\xFD[a\x06\x10\x86a\x05$V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x06,W`\0\x80\xFD[a\x068\x89\x83\x8A\x01a\x05\xA4V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x06QW`\0\x80\xFD[Pa\x06^\x88\x82\x89\x01a\x05\xA4V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x81W`\0\x80\xFD[\x815a\x04\xE0\x81a\x05UV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xC4Wa\x06\xC4a\x06\x8CV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xF2Wa\x06\xF2a\x06\x8CV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x07\x0BW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07$Wa\x07$a\x06\x8CV[a\x077`\x1F\x82\x01`\x1F\x19\x16` \x01a\x06\xCAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07LW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07~W`\0\x80\xFD[\x835a\x07\x89\x81a\x05UV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xABW`\0\x80\xFD[a\x07\xB7\x86\x82\x87\x01a\x06\xFAV[\x91PP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x07\xFAW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\xD5V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x08\x18`@\x83\x01\x85a\x07\xC1V[\x82\x81\x03` \x84\x01Ra\x08*\x81\x85a\x07\xC1V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08FW`\0\x80\xFD[\x825\x91Pa\x08V` \x84\x01a\x05$V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x08\xCBWa\x08\xB8\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a\x08{V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\xE9W`\0\x80\xFD[\x815`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x04\xE0W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\t\x15W`\0\x80\xFD[a\t\x1E\x84a\x05$V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t9W`\0\x80\xFD[a\tE\x86\x82\x87\x01a\x05\xA4V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\tdW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\t{W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x05\xE8W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\t\xA9W`\0\x80\xFD[\x845a\t\xB4\x81a\x05UV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xD6W`\0\x80\xFD[a\t\xE2\x87\x82\x88\x01a\tRV[\x95\x98\x94\x97P\x95PPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x055W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\n\x17W`\0\x80\xFD[a\n \x84a\t\xEEV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n;W`\0\x80\xFD[a\tE\x86\x82\x87\x01a\tRV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x08\xCBW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\ncV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\x9AW`\0\x80\xFD[a\n\xA3\x84a\x05$V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x04\xFCV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\0W`\0\x80\xFD[a\x0B\t\x83a\x05$V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B*W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BGW`\0\x80\xFD[a\x0BS\x85\x82\x86\x01a\x06\xFAV[\x91PP\x92P\x92\x90PV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x055W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x0B\x85W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\x0B\xA0Wa\x0B\xA0a\x06\x8CV[a\x0B\xAE\x81\x83`\x05\x1B\x01a\x06\xCAV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0B\xCDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\x1CW`@\x81\x89\x03\x12\x15a\x0B\xEAW`\0\x80\x81\xFD[a\x0B\xF2a\x06\xA2V[\x815a\x0B\xFD\x81a\x05UV[\x81Ra\x0C\n\x82\x86\x01a\x0B]V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a\x0B\xD1V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C:W`\0\x80\xFD[a\x0CC\x83a\x05$V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C^W`\0\x80\xFD[a\x0BS\x85\x82\x86\x01a\x0BtV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x7FW`\0\x80\xFD[a\x0C\x88\x84a\x05$V[\x92Pa\x0C\x96` \x85\x01a\t\xEEV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xBBW`\0\x80\xFD[\x835\x92Pa\x0C\xCB` \x85\x01a\x05$V[\x91Pa\x0C\xD9`@\x85\x01a\t\xEEV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0C\xF8W`\0\x80\xFD[a\r\x01\x85a\x05$V[\x93Pa\r\x0F` \x86\x01a\t\xEEV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\r9W`\0\x80\xFD[a\rB\x84a\x05$V[\x92Pa\rP` \x85\x01a\x0B]V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\rkW`\0\x80\xFD[a\x07\xB7\x86\x82\x87\x01a\x0BtV\xFE\xA2dipfsX\"\x12 %k\xB3#\x04h;\xA7\x88\xA2\x83^\x8E\xB2\xBFA\xE6\xF7\xE24D\xC85\x1E\xAC\x04\n\x8C\xE0pVpdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STAKEREGISTRYMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80c\xACk\xFB\x03\x11a\0\xF9W\x80c\xD5\xEC\xCC\x05\x11a\0\x97W\x80c\xF2\xBE\x94\xAE\x11a\0qW\x80c\xF2\xBE\x94\xAE\x14a\x04\x80W\x80c\xF8Q\xE1\x98\x14a\x04\x98W\x80c\xFA(\xC6'\x14a\x04\xA6W\x80c\xFFiJw\x14a\x04\xB4W`\0\x80\xFD[\x80c\xD5\xEC\xCC\x05\x14a\x04*W\x80c\xDD\x98F\xB9\x14a\x04]W\x80c\xDF\\\xF7#\x14a\x03rW`\0\x80\xFD[\x80c\xBD)\xB8\xCD\x11a\0\xD3W\x80c\xBD)\xB8\xCD\x14a\x04\x18W\x80c\xC4gx\xA5\x14a\x04*W\x80c\xC6\x01R}\x14a\x048W\x80c\xC8)LV\x14a\x04FW`\0\x80\xFD[\x80c\xACk\xFB\x03\x14a\x03\xA5W\x80c\xAD\xC8\x04\xDA\x14a\x03\xC5W\x80c\xB6\x90Kx\x14a\x04\x05W`\0\x80\xFD[\x80cLQ\xBF\x91\x11a\x01fW\x80c_\x1F-w\x11a\x01@W\x80c_\x1F-w\x14a\x03%W\x80cf\xAC\xFE\xFE\x14a\x038W\x80cm\x14\xA9\x87\x14a\x03rW\x80c\x81\xC0u\x02\x14a\x03\x81W`\0\x80\xFD[\x80cLQ\xBF\x91\x14a\x02\xE0W\x80cT\x01\xED'\x14a\x03\x10W\x80c^Zgu\x14a\x03\x1EW`\0\x80\xFD[\x80c\"J<9\x11a\x01\xA2W\x80c\"J<9\x14a\x025W\x80c%PGw\x14a\x02\x96W\x80c,\xD9Y@\x14a\x02\xBDW\x80c<\xA5\xA5\xF5\x14a\x01\xC9W`\0\x80\xFD[\x80c\x04\x91\xB4\x1C\x14a\x01\xC9W\x80c\x1F\x9Bt\xE0\x14a\x01\xF0W\x80c \xB6b\x98\x14a\x02\x1EW[`\0\x80\xFD[a\x01\xDDa\x01\xD76`\x04a\x05:V[P`\0\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x06a\x01\xFE6`\x04a\x05mV[`\0\x92\x91PPV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE7V[a\x023a\x02,6`\x04a\x05\xEFV[PPPPPV[\0[a\x01\xDDa\x02C6`\x04a\x06oV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01Ri\x1B\xDC\x19\\\x98]\x1B\xDC\x92Y`\xB2\x1B`4\x82\x01R`\0\x90`>\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x91PPV[a\x02\xAFa\x02\xA46`\x04a\x07iV[``\x80\x93P\x93\x91PPV[`@Qa\x01\xE7\x92\x91\x90a\x08\x05V[a\x02\xD3a\x02\xCB6`\x04a\x083V[``\x92\x91PPV[`@Qa\x01\xE7\x91\x90a\x08_V[a\x023a\x02\xEE6`\x04a\x08\xD7V[`\0\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\x01`\xC0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\x06a\x01\xFE6`\x04a\x083V[`\0a\x01\xDDV[a\x023a\x0336`\x04a\t\0V[PPPV[a\x03Za\x03F6`\x04a\t\x93V[`\0T`\x01`\x01`\xC0\x1B\x03\x16\x94\x93PPPPV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE7V[`@Q`\0\x81R` \x01a\x01\xE7V[a\x03\x98a\x03\x8F6`\x04a\n\x02V[``\x93\x92PPPV[`@Qa\x01\xE7\x91\x90a\nGV[a\x03\xB8a\x03\xB36`\x04a\n\x85V[a\x04\xC2V[`@Qa\x01\xE7\x91\x90a\n\xB8V[a\x03\xD8a\x03\xD36`\x04a\n\xEDV[a\x04\xE7V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xE7V[a\x03\xB8a\x04\x136`\x04a\n\xEDV[a\x05\x02V[a\x023a\x04&6`\x04a\x0B\x17V[PPV[a\x02\x06a\x01\xD76`\x04a\x05:V[a\x023a\x04&6`\x04a\x0C'V[a\x02\x06a\x04T6`\x04a\x0CjV[`\0\x93\x92PPPV[a\x04ka\x04T6`\x04a\x0C\xA6V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE7V[a\x02\x06a\x04\x8E6`\x04a\x0C\xE2V[`\0\x94\x93PPPPV[a\x03\xB8a\x04\x136`\x04a\x083V[a\x02\x06a\x04T6`\x04a\x0C\xA6V[a\x023a\x0336`\x04a\r$V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x04\xFCV[\x805`\xFF\x81\x16\x81\x14a\x055W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05LW`\0\x80\xFD[a\x04\xE0\x82a\x05$V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05jW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\x80W`\0\x80\xFD[a\x05\x89\x83a\x05$V[\x91P` \x83\x015a\x05\x99\x81a\x05UV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x05\xB6W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xCDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x05\xE8W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x06\x07W`\0\x80\xFD[a\x06\x10\x86a\x05$V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x06,W`\0\x80\xFD[a\x068\x89\x83\x8A\x01a\x05\xA4V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x06QW`\0\x80\xFD[Pa\x06^\x88\x82\x89\x01a\x05\xA4V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x81W`\0\x80\xFD[\x815a\x04\xE0\x81a\x05UV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xC4Wa\x06\xC4a\x06\x8CV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xF2Wa\x06\xF2a\x06\x8CV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x07\x0BW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07$Wa\x07$a\x06\x8CV[a\x077`\x1F\x82\x01`\x1F\x19\x16` \x01a\x06\xCAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07LW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07~W`\0\x80\xFD[\x835a\x07\x89\x81a\x05UV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xABW`\0\x80\xFD[a\x07\xB7\x86\x82\x87\x01a\x06\xFAV[\x91PP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x07\xFAW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\xD5V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x08\x18`@\x83\x01\x85a\x07\xC1V[\x82\x81\x03` \x84\x01Ra\x08*\x81\x85a\x07\xC1V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08FW`\0\x80\xFD[\x825\x91Pa\x08V` \x84\x01a\x05$V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x08\xCBWa\x08\xB8\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a\x08{V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\xE9W`\0\x80\xFD[\x815`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x04\xE0W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\t\x15W`\0\x80\xFD[a\t\x1E\x84a\x05$V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t9W`\0\x80\xFD[a\tE\x86\x82\x87\x01a\x05\xA4V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\tdW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\t{W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x05\xE8W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\t\xA9W`\0\x80\xFD[\x845a\t\xB4\x81a\x05UV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xD6W`\0\x80\xFD[a\t\xE2\x87\x82\x88\x01a\tRV[\x95\x98\x94\x97P\x95PPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x055W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\n\x17W`\0\x80\xFD[a\n \x84a\t\xEEV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n;W`\0\x80\xFD[a\tE\x86\x82\x87\x01a\tRV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x08\xCBW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\ncV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\x9AW`\0\x80\xFD[a\n\xA3\x84a\x05$V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x04\xFCV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\0W`\0\x80\xFD[a\x0B\t\x83a\x05$V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B*W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BGW`\0\x80\xFD[a\x0BS\x85\x82\x86\x01a\x06\xFAV[\x91PP\x92P\x92\x90PV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x055W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x0B\x85W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\x0B\xA0Wa\x0B\xA0a\x06\x8CV[a\x0B\xAE\x81\x83`\x05\x1B\x01a\x06\xCAV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0B\xCDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\x1CW`@\x81\x89\x03\x12\x15a\x0B\xEAW`\0\x80\x81\xFD[a\x0B\xF2a\x06\xA2V[\x815a\x0B\xFD\x81a\x05UV[\x81Ra\x0C\n\x82\x86\x01a\x0B]V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a\x0B\xD1V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C:W`\0\x80\xFD[a\x0CC\x83a\x05$V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C^W`\0\x80\xFD[a\x0BS\x85\x82\x86\x01a\x0BtV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x7FW`\0\x80\xFD[a\x0C\x88\x84a\x05$V[\x92Pa\x0C\x96` \x85\x01a\t\xEEV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xBBW`\0\x80\xFD[\x835\x92Pa\x0C\xCB` \x85\x01a\x05$V[\x91Pa\x0C\xD9`@\x85\x01a\t\xEEV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0C\xF8W`\0\x80\xFD[a\r\x01\x85a\x05$V[\x93Pa\r\x0F` \x86\x01a\t\xEEV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\r9W`\0\x80\xFD[a\rB\x84a\x05$V[\x92Pa\rP` \x85\x01a\x0B]V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\rkW`\0\x80\xFD[a\x07\xB7\x86\x82\x87\x01a\x0BtV\xFE\xA2dipfsX\"\x12 %k\xB3#\x04h;\xA7\x88\xA2\x83^\x8E\xB2\xBFA\xE6\xF7\xE24D\xC85\x1E\xAC\x04\n\x8C\xE0pVpdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STAKEREGISTRYMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StakeRegistryMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeRegistryMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeRegistryMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeRegistryMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeRegistryMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeRegistryMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeRegistryMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STAKEREGISTRYMOCK_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                STAKEREGISTRYMOCK_ABI.clone(),
                STAKEREGISTRYMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WEIGHTING_DIVISOR` (0x5e5a6775) function
        pub fn weighting_divisor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 90, 103, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStrategies` (0xc601527d) function
        pub fn add_strategies(
            &self,
            quorum_number: u8,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 1, 82, 125], (quorum_number, strategy_params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperator` (0xbd29b8cd) function
        pub fn deregister_operator(
            &self,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 41, 184, 205], (operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentStake` (0x5401ed27) function
        pub fn get_current_stake(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([84, 1, 237, 39], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentTotalStake` (0xd5eccc05) function
        pub fn get_current_total_stake(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([213, 236, 204, 5], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestStakeUpdate` (0xf851e198) function
        pub fn get_latest_stake_update(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([248, 81, 225, 152], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMockOperatorId` (0x224a3c39) function
        pub fn get_mock_operator_id(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([34, 74, 60, 57], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAtBlockNumber` (0xfa28c627) function
        pub fn get_stake_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [250, 40, 198, 39],
                    (operator_id, quorum_number, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAtBlockNumberAndIndex` (0xf2be94ae) function
        pub fn get_stake_at_block_number_and_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [242, 190, 148, 174],
                    (quorum_number, block_number, operator_id, index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeHistory` (0x2cd95940) function
        pub fn get_stake_history(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<StakeUpdate>,
        > {
            self.0
                .method_hash([44, 217, 89, 64], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateAtIndex` (0xac6bfb03) function
        pub fn get_stake_update_at_index(
            &self,
            quorum_number: u8,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([172, 107, 251, 3], (quorum_number, operator_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateIndexAtBlockNumber` (0xdd9846b9) function
        pub fn get_stake_update_index_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash(
                    [221, 152, 70, 185],
                    (operator_id, quorum_number, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeAtBlockNumberFromIndex` (0xc8294c56) function
        pub fn get_total_stake_at_block_number_from_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([200, 41, 76, 86], (quorum_number, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeHistoryLength` (0x0491b41c) function
        pub fn get_total_stake_history_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 145, 180, 28], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeIndicesAtBlockNumber` (0x81c07502) function
        pub fn get_total_stake_indices_at_block_number(
            &self,
            block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([129, 192, 117, 2], (block_number, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeUpdateAtIndex` (0xb6904b78) function
        pub fn get_total_stake_update_at_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([182, 144, 75, 120], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeQuorum` (0xff694a77) function
        pub fn initialize_quorum(
            &self,
            quorum_number: u8,
            minimum_stake: u128,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [255, 105, 74, 119],
                    (quorum_number, minimum_stake, strategy_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumStakeForQuorum` (0xc46778a5) function
        pub fn minimum_stake_for_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([196, 103, 120, 165], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyStrategyParams` (0x20b66298) function
        pub fn modify_strategy_params(
            &self,
            quorum_number: u8,
            strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
            new_multipliers: ::std::vec::Vec<u128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [32, 182, 98, 152],
                    (quorum_number, strategy_indices, new_multipliers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x25504777) function
        pub fn register_operator(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<u128>, ::std::vec::Vec<u128>),
        > {
            self.0
                .method_hash([37, 80, 71, 119], (operator, operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([109, 20, 169, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStrategies` (0x5f1f2d77) function
        pub fn remove_strategies(
            &self,
            quorum_number: u8,
            indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 31, 45, 119], (quorum_number, indices_to_remove))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_updateOperatorStakeReturnBitmap` (0x4c51bf91) function
        pub fn set_update_operator_stake_return_bitmap(
            &self,
            new_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 81, 191, 145], new_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParamsByIndex` (0xadc804da) function
        pub fn strategy_params_by_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StrategyParams> {
            self.0
                .method_hash([173, 200, 4, 218], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParamsLength` (0x3ca5a5f5) function
        pub fn strategy_params_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 165, 165, 245], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorStake` (0x66acfefe) function
        pub fn update_operator_stake(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 172, 254, 254], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weightOfOperatorForQuorum` (0x1f9b74e0) function
        pub fn weight_of_operator_for_quorum(
            &self,
            quorum_number: u8,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([31, 155, 116, 224], (quorum_number, operator))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MinimumStakeForQuorumUpdated` event
        pub fn minimum_stake_for_quorum_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinimumStakeForQuorumUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorStakeUpdate` event
        pub fn operator_stake_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorStakeUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuorumCreated` event
        pub fn quorum_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuorumCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyAddedToQuorum` event
        pub fn strategy_added_to_quorum_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyAddedToQuorumFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyMultiplierUpdated` event
        pub fn strategy_multiplier_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyMultiplierUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyRemovedFromQuorum` event
        pub fn strategy_removed_from_quorum_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyRemovedFromQuorumFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeRegistryMockEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StakeRegistryMock<M> {
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
    #[ethevent(
        name = "MinimumStakeForQuorumUpdated",
        abi = "MinimumStakeForQuorumUpdated(uint8,uint96)"
    )]
    pub struct MinimumStakeForQuorumUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub minimum_stake: u128,
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
        name = "OperatorStakeUpdate",
        abi = "OperatorStakeUpdate(bytes32,uint8,uint96)"
    )]
    pub struct OperatorStakeUpdateFilter {
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub stake: u128,
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
    #[ethevent(name = "QuorumCreated", abi = "QuorumCreated(uint8)")]
    pub struct QuorumCreatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
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
        name = "StrategyAddedToQuorum",
        abi = "StrategyAddedToQuorum(uint8,address)"
    )]
    pub struct StrategyAddedToQuorumFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
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
        name = "StrategyMultiplierUpdated",
        abi = "StrategyMultiplierUpdated(uint8,address,uint256)"
    )]
    pub struct StrategyMultiplierUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: ::ethers::core::types::U256,
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
        name = "StrategyRemovedFromQuorum",
        abi = "StrategyRemovedFromQuorum(uint8,address)"
    )]
    pub struct StrategyRemovedFromQuorumFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StakeRegistryMockEvents {
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        OperatorStakeUpdateFilter(OperatorStakeUpdateFilter),
        QuorumCreatedFilter(QuorumCreatedFilter),
        StrategyAddedToQuorumFilter(StrategyAddedToQuorumFilter),
        StrategyMultiplierUpdatedFilter(StrategyMultiplierUpdatedFilter),
        StrategyRemovedFromQuorumFilter(StrategyRemovedFromQuorumFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeRegistryMockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(
                    StakeRegistryMockEvents::MinimumStakeForQuorumUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorStakeUpdateFilter::decode_log(log) {
                return Ok(StakeRegistryMockEvents::OperatorStakeUpdateFilter(decoded));
            }
            if let Ok(decoded) = QuorumCreatedFilter::decode_log(log) {
                return Ok(StakeRegistryMockEvents::QuorumCreatedFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToQuorumFilter::decode_log(log) {
                return Ok(StakeRegistryMockEvents::StrategyAddedToQuorumFilter(decoded));
            }
            if let Ok(decoded) = StrategyMultiplierUpdatedFilter::decode_log(log) {
                return Ok(
                    StakeRegistryMockEvents::StrategyMultiplierUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyRemovedFromQuorumFilter::decode_log(log) {
                return Ok(
                    StakeRegistryMockEvents::StrategyRemovedFromQuorumFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeRegistryMockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinimumStakeForQuorumUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorStakeUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyAddedToQuorumFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyMultiplierUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyRemovedFromQuorumFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumUpdatedFilter>
    for StakeRegistryMockEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorStakeUpdateFilter> for StakeRegistryMockEvents {
        fn from(value: OperatorStakeUpdateFilter) -> Self {
            Self::OperatorStakeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<QuorumCreatedFilter> for StakeRegistryMockEvents {
        fn from(value: QuorumCreatedFilter) -> Self {
            Self::QuorumCreatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToQuorumFilter> for StakeRegistryMockEvents {
        fn from(value: StrategyAddedToQuorumFilter) -> Self {
            Self::StrategyAddedToQuorumFilter(value)
        }
    }
    impl ::core::convert::From<StrategyMultiplierUpdatedFilter>
    for StakeRegistryMockEvents {
        fn from(value: StrategyMultiplierUpdatedFilter) -> Self {
            Self::StrategyMultiplierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromQuorumFilter>
    for StakeRegistryMockEvents {
        fn from(value: StrategyRemovedFromQuorumFilter) -> Self {
            Self::StrategyRemovedFromQuorumFilter(value)
        }
    }
    ///Container type for all input parameters for the `WEIGHTING_DIVISOR` function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`
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
    #[ethcall(name = "WEIGHTING_DIVISOR", abi = "WEIGHTING_DIVISOR()")]
    pub struct WeightingDivisorCall;
    ///Container type for all input parameters for the `addStrategies` function with signature `addStrategies(uint8,(address,uint96)[])` and selector `0xc601527d`
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
    #[ethcall(name = "addStrategies", abi = "addStrategies(uint8,(address,uint96)[])")]
    pub struct AddStrategiesCall {
        pub quorum_number: u8,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
    }
    ///Container type for all input parameters for the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
    ///Container type for all input parameters for the `deregisterOperator` function with signature `deregisterOperator(bytes32,bytes)` and selector `0xbd29b8cd`
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
    #[ethcall(name = "deregisterOperator", abi = "deregisterOperator(bytes32,bytes)")]
    pub struct DeregisterOperatorCall {
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getCurrentStake` function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`
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
    #[ethcall(name = "getCurrentStake", abi = "getCurrentStake(bytes32,uint8)")]
    pub struct GetCurrentStakeCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getCurrentTotalStake` function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`
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
    #[ethcall(name = "getCurrentTotalStake", abi = "getCurrentTotalStake(uint8)")]
    pub struct GetCurrentTotalStakeCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getLatestStakeUpdate` function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`
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
        name = "getLatestStakeUpdate",
        abi = "getLatestStakeUpdate(bytes32,uint8)"
    )]
    pub struct GetLatestStakeUpdateCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getMockOperatorId` function with signature `getMockOperatorId(address)` and selector `0x224a3c39`
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
    #[ethcall(name = "getMockOperatorId", abi = "getMockOperatorId(address)")]
    pub struct GetMockOperatorIdCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getStakeAtBlockNumber` function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`
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
        name = "getStakeAtBlockNumber",
        abi = "getStakeAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getStakeAtBlockNumberAndIndex` function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`
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
        name = "getStakeAtBlockNumberAndIndex",
        abi = "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)"
    )]
    pub struct GetStakeAtBlockNumberAndIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeHistory` function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`
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
    #[ethcall(name = "getStakeHistory", abi = "getStakeHistory(bytes32,uint8)")]
    pub struct GetStakeHistoryCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeUpdateAtIndex` function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`
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
        name = "getStakeUpdateAtIndex",
        abi = "getStakeUpdateAtIndex(uint8,bytes32,uint256)"
    )]
    pub struct GetStakeUpdateAtIndexCall {
        pub quorum_number: u8,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeUpdateIndexAtBlockNumber` function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`
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
        name = "getStakeUpdateIndexAtBlockNumber",
        abi = "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeUpdateIndexAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getTotalStakeAtBlockNumberFromIndex` function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`
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
        name = "getTotalStakeAtBlockNumberFromIndex",
        abi = "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)"
    )]
    pub struct GetTotalStakeAtBlockNumberFromIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalStakeHistoryLength` function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`
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
        name = "getTotalStakeHistoryLength",
        abi = "getTotalStakeHistoryLength(uint8)"
    )]
    pub struct GetTotalStakeHistoryLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getTotalStakeIndicesAtBlockNumber` function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`
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
        name = "getTotalStakeIndicesAtBlockNumber",
        abi = "getTotalStakeIndicesAtBlockNumber(uint32,bytes)"
    )]
    pub struct GetTotalStakeIndicesAtBlockNumberCall {
        pub block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getTotalStakeUpdateAtIndex` function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`
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
        name = "getTotalStakeUpdateAtIndex",
        abi = "getTotalStakeUpdateAtIndex(uint8,uint256)"
    )]
    pub struct GetTotalStakeUpdateAtIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initializeQuorum` function with signature `initializeQuorum(uint8,uint96,(address,uint96)[])` and selector `0xff694a77`
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
        name = "initializeQuorum",
        abi = "initializeQuorum(uint8,uint96,(address,uint96)[])"
    )]
    pub struct InitializeQuorumCall {
        pub quorum_number: u8,
        pub minimum_stake: u128,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
    }
    ///Container type for all input parameters for the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`
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
    #[ethcall(name = "minimumStakeForQuorum", abi = "minimumStakeForQuorum(uint8)")]
    pub struct MinimumStakeForQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `modifyStrategyParams` function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`
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
        name = "modifyStrategyParams",
        abi = "modifyStrategyParams(uint8,uint256[],uint96[])"
    )]
    pub struct ModifyStrategyParamsCall {
        pub quorum_number: u8,
        pub strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub new_multipliers: ::std::vec::Vec<u128>,
    }
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`
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
        name = "registerOperator",
        abi = "registerOperator(address,bytes32,bytes)"
    )]
    pub struct RegisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `registryCoordinator` function with signature `registryCoordinator()` and selector `0x6d14a987`
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
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
    ///Container type for all input parameters for the `removeStrategies` function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`
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
    #[ethcall(name = "removeStrategies", abi = "removeStrategies(uint8,uint256[])")]
    pub struct RemoveStrategiesCall {
        pub quorum_number: u8,
        pub indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `set_updateOperatorStakeReturnBitmap` function with signature `set_updateOperatorStakeReturnBitmap(uint192)` and selector `0x4c51bf91`
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
        name = "set_updateOperatorStakeReturnBitmap",
        abi = "set_updateOperatorStakeReturnBitmap(uint192)"
    )]
    pub struct SetUpdateOperatorStakeReturnBitmapCall {
        pub new_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategyParamsByIndex` function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`
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
        name = "strategyParamsByIndex",
        abi = "strategyParamsByIndex(uint8,uint256)"
    )]
    pub struct StrategyParamsByIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategyParamsLength` function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`
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
    #[ethcall(name = "strategyParamsLength", abi = "strategyParamsLength(uint8)")]
    pub struct StrategyParamsLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`
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
        name = "updateOperatorStake",
        abi = "updateOperatorStake(address,bytes32,bytes)"
    )]
    pub struct UpdateOperatorStakeCall(
        pub ::ethers::core::types::Address,
        pub [u8; 32],
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `weightOfOperatorForQuorum` function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`
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
        name = "weightOfOperatorForQuorum",
        abi = "weightOfOperatorForQuorum(uint8,address)"
    )]
    pub struct WeightOfOperatorForQuorumCall {
        pub quorum_number: u8,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StakeRegistryMockCalls {
        WeightingDivisor(WeightingDivisorCall),
        AddStrategies(AddStrategiesCall),
        Delegation(DelegationCall),
        DeregisterOperator(DeregisterOperatorCall),
        GetCurrentStake(GetCurrentStakeCall),
        GetCurrentTotalStake(GetCurrentTotalStakeCall),
        GetLatestStakeUpdate(GetLatestStakeUpdateCall),
        GetMockOperatorId(GetMockOperatorIdCall),
        GetStakeAtBlockNumber(GetStakeAtBlockNumberCall),
        GetStakeAtBlockNumberAndIndex(GetStakeAtBlockNumberAndIndexCall),
        GetStakeHistory(GetStakeHistoryCall),
        GetStakeUpdateAtIndex(GetStakeUpdateAtIndexCall),
        GetStakeUpdateIndexAtBlockNumber(GetStakeUpdateIndexAtBlockNumberCall),
        GetTotalStakeAtBlockNumberFromIndex(GetTotalStakeAtBlockNumberFromIndexCall),
        GetTotalStakeHistoryLength(GetTotalStakeHistoryLengthCall),
        GetTotalStakeIndicesAtBlockNumber(GetTotalStakeIndicesAtBlockNumberCall),
        GetTotalStakeUpdateAtIndex(GetTotalStakeUpdateAtIndexCall),
        InitializeQuorum(InitializeQuorumCall),
        MinimumStakeForQuorum(MinimumStakeForQuorumCall),
        ModifyStrategyParams(ModifyStrategyParamsCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RemoveStrategies(RemoveStrategiesCall),
        SetUpdateOperatorStakeReturnBitmap(SetUpdateOperatorStakeReturnBitmapCall),
        StrategyParamsByIndex(StrategyParamsByIndexCall),
        StrategyParamsLength(StrategyParamsLengthCall),
        UpdateOperatorStake(UpdateOperatorStakeCall),
        WeightOfOperatorForQuorum(WeightOfOperatorForQuorumCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeRegistryMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <WeightingDivisorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WeightingDivisor(decoded));
            }
            if let Ok(decoded) = <AddStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddStrategies(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <GetCurrentStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentStake(decoded));
            }
            if let Ok(decoded) = <GetCurrentTotalStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentTotalStake(decoded));
            }
            if let Ok(decoded) = <GetLatestStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestStakeUpdate(decoded));
            }
            if let Ok(decoded) = <GetMockOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMockOperatorId(decoded));
            }
            if let Ok(decoded) = <GetStakeAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetStakeAtBlockNumberAndIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeAtBlockNumberAndIndex(decoded));
            }
            if let Ok(decoded) = <GetStakeHistoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeHistory(decoded));
            }
            if let Ok(decoded) = <GetStakeUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeUpdateAtIndex(decoded));
            }
            if let Ok(decoded) = <GetStakeUpdateIndexAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeUpdateIndexAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeAtBlockNumberFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeAtBlockNumberFromIndex(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeHistoryLength(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeIndicesAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeIndicesAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeUpdateAtIndex(decoded));
            }
            if let Ok(decoded) = <InitializeQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeQuorum(decoded));
            }
            if let Ok(decoded) = <MinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <ModifyStrategyParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyStrategyParams(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) = <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) = <RemoveStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveStrategies(decoded));
            }
            if let Ok(decoded) = <SetUpdateOperatorStakeReturnBitmapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUpdateOperatorStakeReturnBitmap(decoded));
            }
            if let Ok(decoded) = <StrategyParamsByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyParamsByIndex(decoded));
            }
            if let Ok(decoded) = <StrategyParamsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyParamsLength(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorStake(decoded));
            }
            if let Ok(decoded) = <WeightOfOperatorForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WeightOfOperatorForQuorum(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeRegistryMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::WeightingDivisor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentTotalStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMockOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAtBlockNumberAndIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeHistory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateIndexAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeIndicesAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyStrategyParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUpdateOperatorStakeReturnBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParamsByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParamsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightOfOperatorForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakeRegistryMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::WeightingDivisor(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentTotalStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestStakeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMockOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeAtBlockNumberAndIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeUpdateAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeUpdateIndexAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeHistoryLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeIndicesAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeUpdateAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModifyStrategyParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUpdateOperatorStakeReturnBitmap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyParamsByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyParamsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateOperatorStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WeightOfOperatorForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<WeightingDivisorCall> for StakeRegistryMockCalls {
        fn from(value: WeightingDivisorCall) -> Self {
            Self::WeightingDivisor(value)
        }
    }
    impl ::core::convert::From<AddStrategiesCall> for StakeRegistryMockCalls {
        fn from(value: AddStrategiesCall) -> Self {
            Self::AddStrategies(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for StakeRegistryMockCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for StakeRegistryMockCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetCurrentStakeCall> for StakeRegistryMockCalls {
        fn from(value: GetCurrentStakeCall) -> Self {
            Self::GetCurrentStake(value)
        }
    }
    impl ::core::convert::From<GetCurrentTotalStakeCall> for StakeRegistryMockCalls {
        fn from(value: GetCurrentTotalStakeCall) -> Self {
            Self::GetCurrentTotalStake(value)
        }
    }
    impl ::core::convert::From<GetLatestStakeUpdateCall> for StakeRegistryMockCalls {
        fn from(value: GetLatestStakeUpdateCall) -> Self {
            Self::GetLatestStakeUpdate(value)
        }
    }
    impl ::core::convert::From<GetMockOperatorIdCall> for StakeRegistryMockCalls {
        fn from(value: GetMockOperatorIdCall) -> Self {
            Self::GetMockOperatorId(value)
        }
    }
    impl ::core::convert::From<GetStakeAtBlockNumberCall> for StakeRegistryMockCalls {
        fn from(value: GetStakeAtBlockNumberCall) -> Self {
            Self::GetStakeAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetStakeAtBlockNumberAndIndexCall>
    for StakeRegistryMockCalls {
        fn from(value: GetStakeAtBlockNumberAndIndexCall) -> Self {
            Self::GetStakeAtBlockNumberAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeHistoryCall> for StakeRegistryMockCalls {
        fn from(value: GetStakeHistoryCall) -> Self {
            Self::GetStakeHistory(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateAtIndexCall> for StakeRegistryMockCalls {
        fn from(value: GetStakeUpdateAtIndexCall) -> Self {
            Self::GetStakeUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateIndexAtBlockNumberCall>
    for StakeRegistryMockCalls {
        fn from(value: GetStakeUpdateIndexAtBlockNumberCall) -> Self {
            Self::GetStakeUpdateIndexAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeAtBlockNumberFromIndexCall>
    for StakeRegistryMockCalls {
        fn from(value: GetTotalStakeAtBlockNumberFromIndexCall) -> Self {
            Self::GetTotalStakeAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeHistoryLengthCall>
    for StakeRegistryMockCalls {
        fn from(value: GetTotalStakeHistoryLengthCall) -> Self {
            Self::GetTotalStakeHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeIndicesAtBlockNumberCall>
    for StakeRegistryMockCalls {
        fn from(value: GetTotalStakeIndicesAtBlockNumberCall) -> Self {
            Self::GetTotalStakeIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeUpdateAtIndexCall>
    for StakeRegistryMockCalls {
        fn from(value: GetTotalStakeUpdateAtIndexCall) -> Self {
            Self::GetTotalStakeUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<InitializeQuorumCall> for StakeRegistryMockCalls {
        fn from(value: InitializeQuorumCall) -> Self {
            Self::InitializeQuorum(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumCall> for StakeRegistryMockCalls {
        fn from(value: MinimumStakeForQuorumCall) -> Self {
            Self::MinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<ModifyStrategyParamsCall> for StakeRegistryMockCalls {
        fn from(value: ModifyStrategyParamsCall) -> Self {
            Self::ModifyStrategyParams(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for StakeRegistryMockCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for StakeRegistryMockCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesCall> for StakeRegistryMockCalls {
        fn from(value: RemoveStrategiesCall) -> Self {
            Self::RemoveStrategies(value)
        }
    }
    impl ::core::convert::From<SetUpdateOperatorStakeReturnBitmapCall>
    for StakeRegistryMockCalls {
        fn from(value: SetUpdateOperatorStakeReturnBitmapCall) -> Self {
            Self::SetUpdateOperatorStakeReturnBitmap(value)
        }
    }
    impl ::core::convert::From<StrategyParamsByIndexCall> for StakeRegistryMockCalls {
        fn from(value: StrategyParamsByIndexCall) -> Self {
            Self::StrategyParamsByIndex(value)
        }
    }
    impl ::core::convert::From<StrategyParamsLengthCall> for StakeRegistryMockCalls {
        fn from(value: StrategyParamsLengthCall) -> Self {
            Self::StrategyParamsLength(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorStakeCall> for StakeRegistryMockCalls {
        fn from(value: UpdateOperatorStakeCall) -> Self {
            Self::UpdateOperatorStake(value)
        }
    }
    impl ::core::convert::From<WeightOfOperatorForQuorumCall>
    for StakeRegistryMockCalls {
        fn from(value: WeightOfOperatorForQuorumCall) -> Self {
            Self::WeightOfOperatorForQuorum(value)
        }
    }
    ///Container type for all return fields from the `WEIGHTING_DIVISOR` function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`
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
    pub struct WeightingDivisorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getCurrentStake` function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`
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
    pub struct GetCurrentStakeReturn(pub u128);
    ///Container type for all return fields from the `getCurrentTotalStake` function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`
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
    pub struct GetCurrentTotalStakeReturn(pub u128);
    ///Container type for all return fields from the `getLatestStakeUpdate` function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`
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
    pub struct GetLatestStakeUpdateReturn(pub StakeUpdate);
    ///Container type for all return fields from the `getMockOperatorId` function with signature `getMockOperatorId(address)` and selector `0x224a3c39`
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
    pub struct GetMockOperatorIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getStakeAtBlockNumber` function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`
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
    pub struct GetStakeAtBlockNumberReturn(pub u128);
    ///Container type for all return fields from the `getStakeAtBlockNumberAndIndex` function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`
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
    pub struct GetStakeAtBlockNumberAndIndexReturn(pub u128);
    ///Container type for all return fields from the `getStakeHistory` function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`
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
    pub struct GetStakeHistoryReturn(pub ::std::vec::Vec<StakeUpdate>);
    ///Container type for all return fields from the `getStakeUpdateAtIndex` function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`
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
    pub struct GetStakeUpdateAtIndexReturn(pub StakeUpdate);
    ///Container type for all return fields from the `getStakeUpdateIndexAtBlockNumber` function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`
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
    pub struct GetStakeUpdateIndexAtBlockNumberReturn(pub u32);
    ///Container type for all return fields from the `getTotalStakeAtBlockNumberFromIndex` function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`
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
    pub struct GetTotalStakeAtBlockNumberFromIndexReturn(pub u128);
    ///Container type for all return fields from the `getTotalStakeHistoryLength` function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`
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
    pub struct GetTotalStakeHistoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalStakeIndicesAtBlockNumber` function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`
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
    pub struct GetTotalStakeIndicesAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getTotalStakeUpdateAtIndex` function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`
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
    pub struct GetTotalStakeUpdateAtIndexReturn(pub StakeUpdate);
    ///Container type for all return fields from the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`
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
    pub struct MinimumStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `registerOperator` function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`
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
    pub struct RegisterOperatorReturn(
        pub ::std::vec::Vec<u128>,
        pub ::std::vec::Vec<u128>,
    );
    ///Container type for all return fields from the `registryCoordinator` function with signature `registryCoordinator()` and selector `0x6d14a987`
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
    pub struct RegistryCoordinatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `strategyParamsByIndex` function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`
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
    pub struct StrategyParamsByIndexReturn(pub StrategyParams);
    ///Container type for all return fields from the `strategyParamsLength` function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`
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
    pub struct StrategyParamsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`
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
    pub struct UpdateOperatorStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weightOfOperatorForQuorum` function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`
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
    pub struct WeightOfOperatorForQuorumReturn(pub u128);
}
