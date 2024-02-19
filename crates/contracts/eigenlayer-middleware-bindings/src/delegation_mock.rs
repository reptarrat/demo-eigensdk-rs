pub use delegation_mock::*;
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
pub mod delegation_mock {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DELEGATION_APPROVAL_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DELEGATION_APPROVAL_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("STAKER_DELEGATION_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STAKER_DELEGATION_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStrategyManager",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("calculateApproverDigestHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateApproverDigestHash",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "approverDigestHash",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateCurrentStakerDelegationDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateCurrentStakerDelegationDigestHash",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "calculateDelegationApprovalDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDelegationApprovalDigestHash",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                        "calculateOperatorAVSRegistrationDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateOperatorAVSRegistrationDigestHash",
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
                                    name: ::std::borrow::ToOwned::to_owned("digestHash"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateStakerDelegationDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateStakerDelegationDigestHash",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateStakerDigestHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateStakerDigestHash",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerDigestHash"),
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
                    ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateWithdrawalRoot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.Withdrawal",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeQueuedWithdrawal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.Withdrawal",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "middlewareTimesIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAsTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeQueuedWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.Withdrawal[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "middlewareTimesIndexes",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("receiveAsTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
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
                    ::std::borrow::ToOwned::to_owned("cumulativeWithdrawalsQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cumulativeWithdrawalsQueued",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseDelegatedShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "decreaseDelegatedShares",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegateTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegateTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithExpiry",
                                        ),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegateToBySignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "delegateToBySignature",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithExpiry",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithExpiry",
                                        ),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegatedTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegatedTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("delegationApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegationApprover"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegationApproverSaltIsSpent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "delegationApproverSaltIsSpent",
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("earningsReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("earningsReceiver"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategies"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWithdrawalDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getWithdrawalDelay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
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
                    ::std::borrow::ToOwned::to_owned("increaseDelegatedShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "increaseDelegatedShares",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isDelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("isNotDelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isNotDelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "migrateQueuedWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalsToQueue",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal[]",
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
                    ::std::borrow::ToOwned::to_owned("minWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minWithdrawalDelayBlocks",
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
                    ::std::borrow::ToOwned::to_owned("modifyOperatorDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "modifyOperatorDetails",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.OperatorDetails",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorDetails"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.OperatorDetails",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorSaltIsSpent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorSaltIsSpent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("avs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
                    ::std::borrow::ToOwned::to_owned("operatorShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorShares"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                    ::std::borrow::ToOwned::to_owned("queueWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queueWithdrawals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "queuedWithdrawalParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.QueuedWithdrawalParams[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerAsOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerAsOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.OperatorDetails",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStrategyManager",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("setIsOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setIsOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_isOperatorReturnValue",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setOperatorShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOperatorShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("stakerNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakerNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerOptOutWindowBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "stakerOptOutWindowBlocks",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyWithdrawalDelayBlocks",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                    ::std::borrow::ToOwned::to_owned("undelegate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("undelegate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateAVSMetadataURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateAVSMetadataURI",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateOperatorMetadataURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateOperatorMetadataURI",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawSharesAsTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawSharesAsTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStrategyManager",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MinWithdrawalDelayBlocksSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinWithdrawalDelayBlocksSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
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
                    ::std::borrow::ToOwned::to_owned("OperatorDetailsModified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorDetailsModified",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newOperatorDetails",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorMetadataURIUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorMetadataURIUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("metadataURI"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSharesDecreased"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorSharesDecreased",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("OperatorSharesIncreased"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorSharesIncreased",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("StakerDelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakerDelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerForceUndelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StakerForceUndelegated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerUndelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakerUndelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyWithdrawalDelayBlocksSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyWithdrawalDelayBlocksSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrawalCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawalCompleted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalMigrated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrawalMigrated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldWithdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newWithdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrawalQueued"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
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
    pub static DELEGATIONMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x15\x84\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x8AW`\x005`\xE0\x1C\x80cmp\xF7\xAE\x11a\x01\\W\x80c\xBBE\xFE\xF2\x11a\0\xCEW\x80c\xCB\xB5\xD4\xDB\x11a\0\x87W\x80c\xCB\xB5\xD4\xDB\x14a\x05\xB1W\x80c\xDA\x8B\xE8d\x14a\x05\xEAW\x80c\xDB\xE3[\xD8\x14a\x06\"W\x80c\xEE\xA9\x06K\x14a\x06\\W\x80c\xF1ar\xB0\x14a\x06\x9BW\x80c\xF6\x98\xDA%\x14a\x02\xB8W`\0\x80\xFD[\x80c\xBBE\xFE\xF2\x14a\x03\x7FW\x80c\xBCV\xFFf\x14a\x05\0W\x80c\xC4H\xFE\xB8\x14a\x05\x13W\x80c\xC4\x887Z\x14a\x03\x1EW\x80c\xC5\xE4\x80\xDB\x14a\x05\x1BW\x80c\xC9KQ\x11\x14a\x05\xA3W`\0\x80\xFD[\x80c\x90\x04\x13G\x11a\x01 W\x80c\x90\x04\x13G\x14a\x04\xC3W\x80c\x96V\x82q\x14a\x032W\x80c\x99\xBE\x81\xC8\x14a\x04\xD6W\x80c\xA1\x06\x0C\x88\x14a\x04\xE8W\x80c\xA1x\x84\x84\x14a\x03\x1EW\x80c\xA9\x8F\xB3U\x14a\x04\xD6W`\0\x80\xFD[\x80cmp\xF7\xAE\x14a\x04TW\x80ct\xD8\x98\xD7\x14a\x032W\x80cw\x8EU\xF3\x14a\x04wW\x80c\x7FT\x80q\x14a\x04\xA2W\x80c\x8D\x90\xCDV\x14a\x04\xB0W`\0\x80\xFD[\x80c)\xC7}O\x11a\x02\0W\x80cY{6\xDA\x11a\x01\xB9W\x80cY{6\xDA\x14a\x03\xE4W\x80c\\\xFE\x8D,\x14a\x03\xF2W\x80c_\x96o\x14\x14a\x03\x8DW\x80c`\xD7\xFA\xED\x14a\x04\x03W\x80ce\xDA\x12d\x14a\x04\x18W\x80cg\xF2\x92\xC7\x14a\x04AW`\0\x80\xFD[\x80c)\xC7}O\x14a\x03\x1EW\x80c3@C\x96\x14a\x03gW\x80c7H#\xB5\x14a\x03\x7FW\x80c<\xDE\xB5\xE0\x14a\x03\x8DW\x80c>(9\x1D\x14a\x03\xB6W\x80cC7s\x82\x14a\x02\xB8W`\0\x80\xFD[\x80c\x13-Ig\x11a\x02RW\x80c\x13-Ig\x14a\x03\x10W\x80c\x16\x92\x83e\x14a\x03\x1EW\x80c\x1B\xBC\xE0\x91\x14a\x032W\x80c\x1D6\x96\xB7\x14a\x03IW\x80c `kp\x14a\x02\xB8W\x80c(\xA5s\xAE\x14a\x03\x10W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x02\x8FW\x80c\x04\xA4\xF9y\x14a\x02\xB8W\x80c\x0B\x9FHz\x14a\x02\xBFW\x80c\r\xD8\xDD\x02\x14a\x02\xD8W\x80c\x0FX\x9EY\x14a\x02\xFBW[`\0\x80\xFD[a\x02\xA5a\x02\x9D6`\x04a\x08\xFBV[`\0\x92\x91PPV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0a\x02\xA5V[a\x02\xA5a\x02\xCD6`\x04a\taV[`\0\x95\x94PPPPPV[a\x02\xEEa\x02\xE66`\x04a\x08\xFBV[``\x92\x91PPV[`@Qa\x02\xAF\x91\x90a\t\xBCV[a\x03\x0Ea\x03\t6`\x04a\nYV[PPPV[\0[a\x03\x0Ea\x03\t6`\x04a\n\xACV[a\x02\xA5a\x03,6`\x04a\n\xEDV[P`\0\x90V[a\x02\xA5a\x03@6`\x04a\n\xACV[`\0\x93\x92PPPV[a\x03Wa\x03,6`\x04a\n\xEDV[`@Q\x90\x15\x15\x81R` \x01a\x02\xAFV[a\x03\x0Ea\x03u6`\x04a\x0B\x11V[PPPPPPPPV[a\x03Wa\x02\x9D6`\x04a\x0B\xD4V[a\x03\x9Ea\x03\x9B6`\x04a\n\xEDV[\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xAFV[a\x03Wa\x03\xC46`\x04a\n\xEDV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15\x15\x90V[a\x02\xA5a\x03,6`\x04a\r\xB8V[a\x03\x0Ea\x04\x006`\x04a\x0E\xE9V[PV[a\x03\x0Ea\x04\x116`\x04a\x10KV[PPPPPV[a\x03\x9Ea\x04&6`\x04a\n\xEDV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x0Ea\x04O6`\x04a\x10\xD6V[a\x06\xA9V[a\x03Wa\x04b6`\x04a\n\xEDV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA5a\x04\x856`\x04a\x11>V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\x0Ea\x04\x116`\x04a\x12\x1FV[a\x03\x0Ea\x04\xBE6`\x04a\x12\xAFV[a\x07#V[a\x02\xEEa\x04\xD16`\x04a\x13\0V[a\x07\x89V[a\x03\x0Ea\x04\xE46`\x04a\x13OV[PPV[a\x02\xA5a\x04\xF66`\x04a\x13\x84V[`\0\x94\x93PPPPV[a\x03\x0Ea\x05\x0E6`\x04a\x13\xCAV[a\x08kV[a\xC4\xE0a\x02\xA5V[a\x05ma\x05)6`\x04a\n\xEDV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x80\x83R` \x83\x01R`\0\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x02\xAFV[a\x02\xA5a\x04\xF66`\x04a\x14.V[a\x03\x0Ea\x05\xBF6`\x04a\x14fV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[a\x02\xEEa\x05\xF86`\x04a\n\xEDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U``\x90V[a\x03\x0Ea\x0606`\x04a\n\xACV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x90\x95\x16\x82R\x92\x90\x92R\x91\x90 UV[a\x03\x0Ea\x06j6`\x04a\x14\x9BV[PP3`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03\x0Ea\x04\x006`\x04a\x14\xF3V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x86\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@Qc\x8C\x80\xD4\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x85\x16\x90c\x8C\x80\xD4\xE5\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07uW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03uW=`\0\x80>=`\0\xFD[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xA6Wa\x07\xA6a\x0C\0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x08cW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x08\rWa\x08\ra\x15\x0FV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x08HWa\x08Ha\x15\x0FV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x08\\\x81a\x15%V[\x90Pa\x07\xD5V[P\x93\x92PPPV[`@Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90R\x86\x16\x90c\xC4b>\xA1\x90`\x84\x01a\x06\xEAV[`\0\x80\x83`\x1F\x84\x01\x12a\x08\xC2W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xD9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x08\xF4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\t\x0EW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\t$W`\0\x80\xFD[a\t0\x85\x82\x86\x01a\x08\xB0V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\0W`\0\x80\xFD[\x805a\t\\\x81a\t<V[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\tyW`\0\x80\xFD[\x855a\t\x84\x81a\t<V[\x94P` \x86\x015a\t\x94\x81a\t<V[\x93P`@\x86\x015a\t\xA4\x81a\t<V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\t\xF4W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\t\xD8V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15a\n\x12W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\n*W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\nAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08\xF4W`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a\nnW`\0\x80\xFD[a\nx\x85\x85a\n\0V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x93W`\0\x80\xFD[a\n\x9F\x86\x82\x87\x01a\n\x18V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xC1W`\0\x80\xFD[\x835a\n\xCC\x81a\t<V[\x92P` \x84\x015a\n\xDC\x81a\t<V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\xFFW`\0\x80\xFD[\x815a\x0B\n\x81a\t<V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15a\x0B-W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0BDW`\0\x80\xFD[a\x0BP\x8C\x83\x8D\x01a\x08\xB0V[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15a\x0BiW`\0\x80\xFD[a\x0Bu\x8C\x83\x8D\x01a\x08\xB0V[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15a\x0B\x8EW`\0\x80\xFD[a\x0B\x9A\x8C\x83\x8D\x01a\x08\xB0V[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15a\x0B\xB3W`\0\x80\xFD[Pa\x0B\xC0\x8B\x82\x8C\x01a\x08\xB0V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xE7W`\0\x80\xFD[\x825a\x0B\xF2\x81a\t<V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C8Wa\x0C8a\x0C\0V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C8Wa\x0C8a\x0C\0V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C8Wa\x0C8a\x0C\0V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\xAAWa\x0C\xAAa\x0C\0V[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\\W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\xDFWa\x0C\xDFa\x0C\0V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0C\xFAW`\0\x80\xFD[\x815` a\r\x0Fa\r\n\x83a\x0C\xC6V[a\x0C\x82V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\r.W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\rRW\x805a\rE\x81a\t<V[\x83R\x91\x83\x01\x91\x83\x01a\r2V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\rnW`\0\x80\xFD[\x815` a\r~a\r\n\x83a\x0C\xC6V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\r\x9DW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\rRW\x805\x83R\x91\x83\x01\x91\x83\x01a\r\xA1V[`\0` \x82\x84\x03\x12\x15a\r\xCAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\xE1W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\r\xF5W`\0\x80\xFD[a\r\xFDa\x0C\x16V[a\x0E\x06\x83a\tQV[\x81Ra\x0E\x14` \x84\x01a\tQV[` \x82\x01Ra\x0E%`@\x84\x01a\tQV[`@\x82\x01R``\x83\x015``\x82\x01Ra\x0E@`\x80\x84\x01a\x0C\xB2V[`\x80\x82\x01R`\xA0\x83\x015\x82\x81\x11\x15a\x0EWW`\0\x80\xFD[a\x0Ec\x87\x82\x86\x01a\x0C\xE9V[`\xA0\x83\x01RP`\xC0\x83\x015\x82\x81\x11\x15a\x0E{W`\0\x80\xFD[a\x0E\x87\x87\x82\x86\x01a\r]V[`\xC0\x83\x01RP\x95\x94PPPPPV[`\0`@\x82\x84\x03\x12\x15a\x0E\xA8W`\0\x80\xFD[a\x0E\xB0a\x0C>V[\x90P\x815a\x0E\xBD\x81a\t<V[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\xDEW`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xFCW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0F\x13W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0F'W`\0\x80\xFD[\x815a\x0F5a\r\n\x82a\x0C\xC6V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x0FTW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x10.W\x805\x85\x81\x11\x15a\x0FpW`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x0F\x88W`\0\x80\x81\xFD[a\x0F\x90a\x0C`V[\x89\x83\x015\x88\x81\x11\x15a\x0F\xA2W`\0\x80\x81\xFD[a\x0F\xB0\x8E\x8C\x83\x87\x01\x01a\x0C\xE9V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15a\x0F\xC7W`\0\x80\x81\xFD[a\x0F\xD5\x8F\x8D\x83\x88\x01\x01a\r]V[\x8C\x84\x01RP``a\x0F\xE7\x81\x86\x01a\tQV[\x82\x84\x01R`\x80\x91Pa\x0F\xFB\x8F\x83\x87\x01a\x0E\x96V[\x90\x83\x01Ra\x10\x0B`\xC0\x85\x01a\x0C\xB2V[\x90\x82\x01Ra\x10\x1A\x83\x83\x01a\tQV[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01a\x0FXV[P\x98\x97PPPPPPPPV[\x805\x80\x15\x15\x81\x14a\t\\W`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x10cW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10zW`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x10\x8EW`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15a\x10\xA4W`\0\x80\xFD[Pa\x10\xB1\x88\x82\x89\x01a\x08\xB0V[\x90\x95P\x93PP`@\x86\x015\x91Pa\x10\xCA``\x87\x01a\x10;V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x10\xEEW`\0\x80\xFD[\x855a\x10\xF9\x81a\t<V[\x94P` \x86\x015a\x11\t\x81a\t<V[\x93P`@\x86\x015a\x11\x19\x81a\t<V[\x92P``\x86\x015\x91P`\x80\x86\x015a\x110\x81a\t<V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11QW`\0\x80\xFD[\x825a\x11\\\x81a\t<V[\x91P` \x83\x015a\x11l\x81a\t<V[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x11\x89W`\0\x80\xFD[a\x11\x91a\x0C>V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\xAAW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x11\xBEW`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x11\xD2Wa\x11\xD2a\x0C\0V[a\x11\xE4`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x0C\x82V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15a\x11\xFAW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x127W`\0\x80\xFD[\x855a\x12B\x81a\t<V[\x94P` \x86\x015a\x12R\x81a\t<V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12nW`\0\x80\xFD[a\x12z\x89\x83\x8A\x01a\x11wV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x12\x90W`\0\x80\xFD[Pa\x12\x9D\x88\x82\x89\x01a\x11wV[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xC5W`\0\x80\xFD[\x845a\x12\xD0\x81a\t<V[\x93P` \x85\x015a\x12\xE0\x81a\t<V[\x92P`@\x85\x015a\x12\xF0\x81a\t<V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x13W`\0\x80\xFD[\x825a\x13\x1E\x81a\t<V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x139W`\0\x80\xFD[a\x13E\x85\x82\x86\x01a\x0C\xE9V[\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x13bW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13xW`\0\x80\xFD[a\t0\x85\x82\x86\x01a\n\x18V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13\x9AW`\0\x80\xFD[\x845a\x13\xA5\x81a\t<V[\x93P` \x85\x015a\x13\xB5\x81a\t<V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x13\xE2W`\0\x80\xFD[\x855a\x13\xED\x81a\t<V[\x94P` \x86\x015a\x13\xFD\x81a\t<V[\x93P`@\x86\x015a\x14\r\x81a\t<V[\x92P``\x86\x015a\x14\x1D\x81a\t<V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14DW`\0\x80\xFD[\x845a\x14O\x81a\t<V[\x93P` \x85\x015\x92P`@\x85\x015a\x12\xF0\x81a\t<V[`\0\x80`@\x83\x85\x03\x12\x15a\x14yW`\0\x80\xFD[\x825a\x14\x84\x81a\t<V[\x91Pa\x14\x92` \x84\x01a\x10;V[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xB0W`\0\x80\xFD[\x835a\x14\xBB\x81a\t<V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD6W`\0\x80\xFD[a\x14\xE2\x86\x82\x87\x01a\x11wV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a\x15\x05W`\0\x80\xFD[a\x0B\n\x83\x83a\n\0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x15GWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 v\xEB\xC1\x0F&\x1DB\xF9%\xB8Y*\x88z\x9E\x14\x86C\xCBL\xF3\x9A\xBE\xCB\x1A\xEB\xD0\xB8N\xC8\0\x1EdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static DELEGATIONMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x8AW`\x005`\xE0\x1C\x80cmp\xF7\xAE\x11a\x01\\W\x80c\xBBE\xFE\xF2\x11a\0\xCEW\x80c\xCB\xB5\xD4\xDB\x11a\0\x87W\x80c\xCB\xB5\xD4\xDB\x14a\x05\xB1W\x80c\xDA\x8B\xE8d\x14a\x05\xEAW\x80c\xDB\xE3[\xD8\x14a\x06\"W\x80c\xEE\xA9\x06K\x14a\x06\\W\x80c\xF1ar\xB0\x14a\x06\x9BW\x80c\xF6\x98\xDA%\x14a\x02\xB8W`\0\x80\xFD[\x80c\xBBE\xFE\xF2\x14a\x03\x7FW\x80c\xBCV\xFFf\x14a\x05\0W\x80c\xC4H\xFE\xB8\x14a\x05\x13W\x80c\xC4\x887Z\x14a\x03\x1EW\x80c\xC5\xE4\x80\xDB\x14a\x05\x1BW\x80c\xC9KQ\x11\x14a\x05\xA3W`\0\x80\xFD[\x80c\x90\x04\x13G\x11a\x01 W\x80c\x90\x04\x13G\x14a\x04\xC3W\x80c\x96V\x82q\x14a\x032W\x80c\x99\xBE\x81\xC8\x14a\x04\xD6W\x80c\xA1\x06\x0C\x88\x14a\x04\xE8W\x80c\xA1x\x84\x84\x14a\x03\x1EW\x80c\xA9\x8F\xB3U\x14a\x04\xD6W`\0\x80\xFD[\x80cmp\xF7\xAE\x14a\x04TW\x80ct\xD8\x98\xD7\x14a\x032W\x80cw\x8EU\xF3\x14a\x04wW\x80c\x7FT\x80q\x14a\x04\xA2W\x80c\x8D\x90\xCDV\x14a\x04\xB0W`\0\x80\xFD[\x80c)\xC7}O\x11a\x02\0W\x80cY{6\xDA\x11a\x01\xB9W\x80cY{6\xDA\x14a\x03\xE4W\x80c\\\xFE\x8D,\x14a\x03\xF2W\x80c_\x96o\x14\x14a\x03\x8DW\x80c`\xD7\xFA\xED\x14a\x04\x03W\x80ce\xDA\x12d\x14a\x04\x18W\x80cg\xF2\x92\xC7\x14a\x04AW`\0\x80\xFD[\x80c)\xC7}O\x14a\x03\x1EW\x80c3@C\x96\x14a\x03gW\x80c7H#\xB5\x14a\x03\x7FW\x80c<\xDE\xB5\xE0\x14a\x03\x8DW\x80c>(9\x1D\x14a\x03\xB6W\x80cC7s\x82\x14a\x02\xB8W`\0\x80\xFD[\x80c\x13-Ig\x11a\x02RW\x80c\x13-Ig\x14a\x03\x10W\x80c\x16\x92\x83e\x14a\x03\x1EW\x80c\x1B\xBC\xE0\x91\x14a\x032W\x80c\x1D6\x96\xB7\x14a\x03IW\x80c `kp\x14a\x02\xB8W\x80c(\xA5s\xAE\x14a\x03\x10W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x02\x8FW\x80c\x04\xA4\xF9y\x14a\x02\xB8W\x80c\x0B\x9FHz\x14a\x02\xBFW\x80c\r\xD8\xDD\x02\x14a\x02\xD8W\x80c\x0FX\x9EY\x14a\x02\xFBW[`\0\x80\xFD[a\x02\xA5a\x02\x9D6`\x04a\x08\xFBV[`\0\x92\x91PPV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0a\x02\xA5V[a\x02\xA5a\x02\xCD6`\x04a\taV[`\0\x95\x94PPPPPV[a\x02\xEEa\x02\xE66`\x04a\x08\xFBV[``\x92\x91PPV[`@Qa\x02\xAF\x91\x90a\t\xBCV[a\x03\x0Ea\x03\t6`\x04a\nYV[PPPV[\0[a\x03\x0Ea\x03\t6`\x04a\n\xACV[a\x02\xA5a\x03,6`\x04a\n\xEDV[P`\0\x90V[a\x02\xA5a\x03@6`\x04a\n\xACV[`\0\x93\x92PPPV[a\x03Wa\x03,6`\x04a\n\xEDV[`@Q\x90\x15\x15\x81R` \x01a\x02\xAFV[a\x03\x0Ea\x03u6`\x04a\x0B\x11V[PPPPPPPPV[a\x03Wa\x02\x9D6`\x04a\x0B\xD4V[a\x03\x9Ea\x03\x9B6`\x04a\n\xEDV[\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xAFV[a\x03Wa\x03\xC46`\x04a\n\xEDV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15\x15\x90V[a\x02\xA5a\x03,6`\x04a\r\xB8V[a\x03\x0Ea\x04\x006`\x04a\x0E\xE9V[PV[a\x03\x0Ea\x04\x116`\x04a\x10KV[PPPPPV[a\x03\x9Ea\x04&6`\x04a\n\xEDV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x0Ea\x04O6`\x04a\x10\xD6V[a\x06\xA9V[a\x03Wa\x04b6`\x04a\n\xEDV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA5a\x04\x856`\x04a\x11>V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\x0Ea\x04\x116`\x04a\x12\x1FV[a\x03\x0Ea\x04\xBE6`\x04a\x12\xAFV[a\x07#V[a\x02\xEEa\x04\xD16`\x04a\x13\0V[a\x07\x89V[a\x03\x0Ea\x04\xE46`\x04a\x13OV[PPV[a\x02\xA5a\x04\xF66`\x04a\x13\x84V[`\0\x94\x93PPPPV[a\x03\x0Ea\x05\x0E6`\x04a\x13\xCAV[a\x08kV[a\xC4\xE0a\x02\xA5V[a\x05ma\x05)6`\x04a\n\xEDV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x80\x83R` \x83\x01R`\0\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x02\xAFV[a\x02\xA5a\x04\xF66`\x04a\x14.V[a\x03\x0Ea\x05\xBF6`\x04a\x14fV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[a\x02\xEEa\x05\xF86`\x04a\n\xEDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U``\x90V[a\x03\x0Ea\x0606`\x04a\n\xACV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x90\x95\x16\x82R\x92\x90\x92R\x91\x90 UV[a\x03\x0Ea\x06j6`\x04a\x14\x9BV[PP3`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03\x0Ea\x04\x006`\x04a\x14\xF3V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x86\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@Qc\x8C\x80\xD4\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x85\x16\x90c\x8C\x80\xD4\xE5\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07uW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03uW=`\0\x80>=`\0\xFD[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xA6Wa\x07\xA6a\x0C\0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x08cW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x08\rWa\x08\ra\x15\x0FV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x08HWa\x08Ha\x15\x0FV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x08\\\x81a\x15%V[\x90Pa\x07\xD5V[P\x93\x92PPPV[`@Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90R\x86\x16\x90c\xC4b>\xA1\x90`\x84\x01a\x06\xEAV[`\0\x80\x83`\x1F\x84\x01\x12a\x08\xC2W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xD9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x08\xF4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\t\x0EW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\t$W`\0\x80\xFD[a\t0\x85\x82\x86\x01a\x08\xB0V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\0W`\0\x80\xFD[\x805a\t\\\x81a\t<V[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\tyW`\0\x80\xFD[\x855a\t\x84\x81a\t<V[\x94P` \x86\x015a\t\x94\x81a\t<V[\x93P`@\x86\x015a\t\xA4\x81a\t<V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\t\xF4W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\t\xD8V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15a\n\x12W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\n*W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\nAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08\xF4W`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a\nnW`\0\x80\xFD[a\nx\x85\x85a\n\0V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x93W`\0\x80\xFD[a\n\x9F\x86\x82\x87\x01a\n\x18V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xC1W`\0\x80\xFD[\x835a\n\xCC\x81a\t<V[\x92P` \x84\x015a\n\xDC\x81a\t<V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\xFFW`\0\x80\xFD[\x815a\x0B\n\x81a\t<V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15a\x0B-W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0BDW`\0\x80\xFD[a\x0BP\x8C\x83\x8D\x01a\x08\xB0V[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15a\x0BiW`\0\x80\xFD[a\x0Bu\x8C\x83\x8D\x01a\x08\xB0V[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15a\x0B\x8EW`\0\x80\xFD[a\x0B\x9A\x8C\x83\x8D\x01a\x08\xB0V[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15a\x0B\xB3W`\0\x80\xFD[Pa\x0B\xC0\x8B\x82\x8C\x01a\x08\xB0V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xE7W`\0\x80\xFD[\x825a\x0B\xF2\x81a\t<V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C8Wa\x0C8a\x0C\0V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C8Wa\x0C8a\x0C\0V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C8Wa\x0C8a\x0C\0V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\xAAWa\x0C\xAAa\x0C\0V[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\\W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\xDFWa\x0C\xDFa\x0C\0V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0C\xFAW`\0\x80\xFD[\x815` a\r\x0Fa\r\n\x83a\x0C\xC6V[a\x0C\x82V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\r.W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\rRW\x805a\rE\x81a\t<V[\x83R\x91\x83\x01\x91\x83\x01a\r2V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\rnW`\0\x80\xFD[\x815` a\r~a\r\n\x83a\x0C\xC6V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\r\x9DW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\rRW\x805\x83R\x91\x83\x01\x91\x83\x01a\r\xA1V[`\0` \x82\x84\x03\x12\x15a\r\xCAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\xE1W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\r\xF5W`\0\x80\xFD[a\r\xFDa\x0C\x16V[a\x0E\x06\x83a\tQV[\x81Ra\x0E\x14` \x84\x01a\tQV[` \x82\x01Ra\x0E%`@\x84\x01a\tQV[`@\x82\x01R``\x83\x015``\x82\x01Ra\x0E@`\x80\x84\x01a\x0C\xB2V[`\x80\x82\x01R`\xA0\x83\x015\x82\x81\x11\x15a\x0EWW`\0\x80\xFD[a\x0Ec\x87\x82\x86\x01a\x0C\xE9V[`\xA0\x83\x01RP`\xC0\x83\x015\x82\x81\x11\x15a\x0E{W`\0\x80\xFD[a\x0E\x87\x87\x82\x86\x01a\r]V[`\xC0\x83\x01RP\x95\x94PPPPPV[`\0`@\x82\x84\x03\x12\x15a\x0E\xA8W`\0\x80\xFD[a\x0E\xB0a\x0C>V[\x90P\x815a\x0E\xBD\x81a\t<V[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\xDEW`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xFCW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0F\x13W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0F'W`\0\x80\xFD[\x815a\x0F5a\r\n\x82a\x0C\xC6V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x0FTW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x10.W\x805\x85\x81\x11\x15a\x0FpW`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x0F\x88W`\0\x80\x81\xFD[a\x0F\x90a\x0C`V[\x89\x83\x015\x88\x81\x11\x15a\x0F\xA2W`\0\x80\x81\xFD[a\x0F\xB0\x8E\x8C\x83\x87\x01\x01a\x0C\xE9V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15a\x0F\xC7W`\0\x80\x81\xFD[a\x0F\xD5\x8F\x8D\x83\x88\x01\x01a\r]V[\x8C\x84\x01RP``a\x0F\xE7\x81\x86\x01a\tQV[\x82\x84\x01R`\x80\x91Pa\x0F\xFB\x8F\x83\x87\x01a\x0E\x96V[\x90\x83\x01Ra\x10\x0B`\xC0\x85\x01a\x0C\xB2V[\x90\x82\x01Ra\x10\x1A\x83\x83\x01a\tQV[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01a\x0FXV[P\x98\x97PPPPPPPPV[\x805\x80\x15\x15\x81\x14a\t\\W`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x10cW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10zW`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x10\x8EW`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15a\x10\xA4W`\0\x80\xFD[Pa\x10\xB1\x88\x82\x89\x01a\x08\xB0V[\x90\x95P\x93PP`@\x86\x015\x91Pa\x10\xCA``\x87\x01a\x10;V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x10\xEEW`\0\x80\xFD[\x855a\x10\xF9\x81a\t<V[\x94P` \x86\x015a\x11\t\x81a\t<V[\x93P`@\x86\x015a\x11\x19\x81a\t<V[\x92P``\x86\x015\x91P`\x80\x86\x015a\x110\x81a\t<V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11QW`\0\x80\xFD[\x825a\x11\\\x81a\t<V[\x91P` \x83\x015a\x11l\x81a\t<V[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x11\x89W`\0\x80\xFD[a\x11\x91a\x0C>V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\xAAW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x11\xBEW`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x11\xD2Wa\x11\xD2a\x0C\0V[a\x11\xE4`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x0C\x82V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15a\x11\xFAW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x127W`\0\x80\xFD[\x855a\x12B\x81a\t<V[\x94P` \x86\x015a\x12R\x81a\t<V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12nW`\0\x80\xFD[a\x12z\x89\x83\x8A\x01a\x11wV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x12\x90W`\0\x80\xFD[Pa\x12\x9D\x88\x82\x89\x01a\x11wV[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xC5W`\0\x80\xFD[\x845a\x12\xD0\x81a\t<V[\x93P` \x85\x015a\x12\xE0\x81a\t<V[\x92P`@\x85\x015a\x12\xF0\x81a\t<V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x13W`\0\x80\xFD[\x825a\x13\x1E\x81a\t<V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x139W`\0\x80\xFD[a\x13E\x85\x82\x86\x01a\x0C\xE9V[\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x13bW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13xW`\0\x80\xFD[a\t0\x85\x82\x86\x01a\n\x18V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13\x9AW`\0\x80\xFD[\x845a\x13\xA5\x81a\t<V[\x93P` \x85\x015a\x13\xB5\x81a\t<V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x13\xE2W`\0\x80\xFD[\x855a\x13\xED\x81a\t<V[\x94P` \x86\x015a\x13\xFD\x81a\t<V[\x93P`@\x86\x015a\x14\r\x81a\t<V[\x92P``\x86\x015a\x14\x1D\x81a\t<V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14DW`\0\x80\xFD[\x845a\x14O\x81a\t<V[\x93P` \x85\x015\x92P`@\x85\x015a\x12\xF0\x81a\t<V[`\0\x80`@\x83\x85\x03\x12\x15a\x14yW`\0\x80\xFD[\x825a\x14\x84\x81a\t<V[\x91Pa\x14\x92` \x84\x01a\x10;V[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xB0W`\0\x80\xFD[\x835a\x14\xBB\x81a\t<V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD6W`\0\x80\xFD[a\x14\xE2\x86\x82\x87\x01a\x11wV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a\x15\x05W`\0\x80\xFD[a\x0B\n\x83\x83a\n\0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x15GWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 v\xEB\xC1\x0F&\x1DB\xF9%\xB8Y*\x88z\x9E\x14\x86C\xCBL\xF3\x9A\xBE\xCB\x1A\xEB\xD0\xB8N\xC8\0\x1EdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static DELEGATIONMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DelegationMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DelegationMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DelegationMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DelegationMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DelegationMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DelegationMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DelegationMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DELEGATIONMOCK_ABI.clone(),
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
                DELEGATIONMOCK_ABI.clone(),
                DELEGATIONMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DELEGATION_APPROVAL_TYPEHASH` (0x04a4f979) function
        pub fn delegation_approval_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([4, 164, 249, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function
        pub fn domain_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKER_DELEGATION_TYPEHASH` (0x43377382) function
        pub fn staker_delegation_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 55, 115, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addShares` (0xbc56ff66) function
        pub fn add_shares(
            &self,
            strategy_manager: ::ethers::core::types::Address,
            staker: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [188, 86, 255, 102],
                    (strategy_manager, staker, token, strategy, shares),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateApproverDigestHash` (0x74d898d7) function
        pub fn calculate_approver_digest_hash(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([116, 216, 152, 215], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateCurrentStakerDelegationDigestHash` (0x1bbce091) function
        pub fn calculate_current_staker_delegation_digest_hash(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 188, 224, 145], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDelegationApprovalDigestHash` (0x0b9f487a) function
        pub fn calculate_delegation_approval_digest_hash(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Address,
            p3: [u8; 32],
            p4: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([11, 159, 72, 122], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOperatorAVSRegistrationDigestHash` (0xa1060c88) function
        pub fn calculate_operator_avs_registration_digest_hash(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: [u8; 32],
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([161, 6, 12, 136], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateStakerDelegationDigestHash` (0xc94b5111) function
        pub fn calculate_staker_delegation_digest_hash(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::Address,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([201, 75, 81, 17], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateStakerDigestHash` (0x96568271) function
        pub fn calculate_staker_digest_hash(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([150, 86, 130, 113], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateWithdrawalRoot` (0x597b36da) function
        pub fn calculate_withdrawal_root(
            &self,
            withdrawal: Withdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([89, 123, 54, 218], (withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeQueuedWithdrawal` (0x60d7faed) function
        pub fn complete_queued_withdrawal(
            &self,
            withdrawal: Withdrawal,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            middleware_times_index: ::ethers::core::types::U256,
            receive_as_tokens: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [96, 215, 250, 237],
                    (withdrawal, tokens, middleware_times_index, receive_as_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeQueuedWithdrawals` (0x33404396) function
        pub fn complete_queued_withdrawals(
            &self,
            withdrawals: ::std::vec::Vec<Withdrawal>,
            tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
            middleware_times_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
            receive_as_tokens: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [51, 64, 67, 150],
                    (withdrawals, tokens, middleware_times_indexes, receive_as_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeWithdrawalsQueued` (0xa1788484) function
        pub fn cumulative_withdrawals_queued(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 120, 132, 132], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseDelegatedShares` (0x132d4967) function
        pub fn decrease_delegated_shares(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 45, 73, 103], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateTo` (0xeea9064b) function
        pub fn delegate_to(
            &self,
            operator: ::ethers::core::types::Address,
            p1: SignatureWithExpiry,
            p2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 169, 6, 75], (operator, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateToBySignature` (0x7f548071) function
        pub fn delegate_to_by_signature(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: SignatureWithExpiry,
            p3: SignatureWithExpiry,
            p4: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 84, 128, 113], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegatedTo` (0x65da1264) function
        pub fn delegated_to(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([101, 218, 18, 100], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationApprover` (0x3cdeb5e0) function
        pub fn delegation_approver(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([60, 222, 181, 224], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationApproverSaltIsSpent` (0xbb45fef2) function
        pub fn delegation_approver_salt_is_spent(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([187, 69, 254, 242], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earningsReceiver` (0x5f966f14) function
        pub fn earnings_receiver(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([95, 150, 111, 20], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorShares` (0x90041347) function
        pub fn get_operator_shares(
            &self,
            operator: ::ethers::core::types::Address,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([144, 4, 19, 71], (operator, strategies))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWithdrawalDelay` (0x0449ca39) function
        pub fn get_withdrawal_delay(
            &self,
            p0: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 73, 202, 57], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseDelegatedShares` (0x28a573ae) function
        pub fn increase_delegated_shares(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 165, 115, 174], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDelegated` (0x3e28391d) function
        pub fn is_delegated(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 40, 57, 29], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isNotDelegated` (0x1d3696b7) function
        pub fn is_not_delegated(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 54, 150, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperator` (0x6d70f7ae) function
        pub fn is_operator(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 112, 247, 174], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateQueuedWithdrawals` (0x5cfe8d2c) function
        pub fn migrate_queued_withdrawals(
            &self,
            withdrawals_to_queue: ::std::vec::Vec<DeprecatedStructQueuedWithdrawal>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 254, 141, 44], withdrawals_to_queue)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minWithdrawalDelayBlocks` (0xc448feb8) function
        pub fn min_withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 72, 254, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyOperatorDetails` (0xf16172b0) function
        pub fn modify_operator_details(
            &self,
            p0: OperatorDetails,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 97, 114, 176], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorDetails` (0xc5e480db) function
        pub fn operator_details(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorDetails> {
            self.0
                .method_hash([197, 228, 128, 219], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSaltIsSpent` (0x374823b5) function
        pub fn operator_salt_is_spent(
            &self,
            avs: ::ethers::core::types::Address,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 72, 35, 181], (avs, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorShares` (0x778e55f3) function
        pub fn operator_shares(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 142, 85, 243], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueWithdrawals` (0x0dd8dd02) function
        pub fn queue_withdrawals(
            &self,
            queued_withdrawal_params: ::std::vec::Vec<QueuedWithdrawalParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([13, 216, 221, 2], queued_withdrawal_params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerAsOperator` (0x0f589e59) function
        pub fn register_as_operator(
            &self,
            p0: OperatorDetails,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 88, 158, 89], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeShares` (0x8d90cd56) function
        pub fn remove_shares(
            &self,
            strategy_manager: ::ethers::core::types::Address,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [141, 144, 205, 86],
                    (strategy_manager, staker, strategy, shares),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setIsOperator` (0xcbb5d4db) function
        pub fn set_is_operator(
            &self,
            operator: ::ethers::core::types::Address,
            is_operator_return_value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 181, 212, 219], (operator, is_operator_return_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperatorShares` (0xdbe35bd8) function
        pub fn set_operator_shares(
            &self,
            operator: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 227, 91, 216], (operator, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerNonce` (0x29c77d4f) function
        pub fn staker_nonce(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([41, 199, 125, 79], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerOptOutWindowBlocks` (0x16928365) function
        pub fn staker_opt_out_window_blocks(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 146, 131, 101], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyWithdrawalDelayBlocks` (0xc488375a) function
        pub fn strategy_withdrawal_delay_blocks(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 136, 55, 90], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `undelegate` (0xda8be864) function
        pub fn undelegate(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([218, 139, 232, 100], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAVSMetadataURI` (0xa98fb355) function
        pub fn update_avs_metadata_uri(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 143, 179, 85], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorMetadataURI` (0x99be81c8) function
        pub fn update_operator_metadata_uri(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 190, 129, 200], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawSharesAsTokens` (0x67f292c7) function
        pub fn withdraw_shares_as_tokens(
            &self,
            strategy_manager: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [103, 242, 146, 199],
                    (strategy_manager, recipient, strategy, shares, token),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MinWithdrawalDelayBlocksSet` event
        pub fn min_withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinWithdrawalDelayBlocksSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorDetailsModified` event
        pub fn operator_details_modified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorDetailsModifiedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorMetadataURIUpdated` event
        pub fn operator_metadata_uri_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorMetadataURIUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorRegistered` event
        pub fn operator_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSharesDecreased` event
        pub fn operator_shares_decreased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSharesDecreasedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSharesIncreased` event
        pub fn operator_shares_increased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSharesIncreasedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakerDelegated` event
        pub fn staker_delegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakerDelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakerForceUndelegated` event
        pub fn staker_force_undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakerForceUndelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakerUndelegated` event
        pub fn staker_undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakerUndelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyWithdrawalDelayBlocksSet` event
        pub fn strategy_withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyWithdrawalDelayBlocksSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalCompleted` event
        pub fn withdrawal_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalCompletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalMigrated` event
        pub fn withdrawal_migrated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalMigratedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalQueued` event
        pub fn withdrawal_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalQueuedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegationMockEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DelegationMock<M> {
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
        name = "MinWithdrawalDelayBlocksSet",
        abi = "MinWithdrawalDelayBlocksSet(uint256,uint256)"
    )]
    pub struct MinWithdrawalDelayBlocksSetFilter {
        pub previous_value: ::ethers::core::types::U256,
        pub new_value: ::ethers::core::types::U256,
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
        name = "OperatorDetailsModified",
        abi = "OperatorDetailsModified(address,(address,address,uint32))"
    )]
    pub struct OperatorDetailsModifiedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub new_operator_details: OperatorDetails,
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
        name = "OperatorMetadataURIUpdated",
        abi = "OperatorMetadataURIUpdated(address,string)"
    )]
    pub struct OperatorMetadataURIUpdatedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub metadata_uri: ::std::string::String,
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
        name = "OperatorRegistered",
        abi = "OperatorRegistered(address,(address,address,uint32))"
    )]
    pub struct OperatorRegisteredFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub operator_details: OperatorDetails,
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
        name = "OperatorSharesDecreased",
        abi = "OperatorSharesDecreased(address,address,address,uint256)"
    )]
    pub struct OperatorSharesDecreasedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
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
        name = "OperatorSharesIncreased",
        abi = "OperatorSharesIncreased(address,address,address,uint256)"
    )]
    pub struct OperatorSharesIncreasedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
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
    #[ethevent(name = "StakerDelegated", abi = "StakerDelegated(address,address)")]
    pub struct StakerDelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StakerForceUndelegated",
        abi = "StakerForceUndelegated(address,address)"
    )]
    pub struct StakerForceUndelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "StakerUndelegated", abi = "StakerUndelegated(address,address)")]
    pub struct StakerUndelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StrategyWithdrawalDelayBlocksSet",
        abi = "StrategyWithdrawalDelayBlocksSet(address,uint256,uint256)"
    )]
    pub struct StrategyWithdrawalDelayBlocksSetFilter {
        pub strategy: ::ethers::core::types::Address,
        pub previous_value: ::ethers::core::types::U256,
        pub new_value: ::ethers::core::types::U256,
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
    #[ethevent(name = "WithdrawalCompleted", abi = "WithdrawalCompleted(bytes32)")]
    pub struct WithdrawalCompletedFilter {
        pub withdrawal_root: [u8; 32],
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
    #[ethevent(name = "WithdrawalMigrated", abi = "WithdrawalMigrated(bytes32,bytes32)")]
    pub struct WithdrawalMigratedFilter {
        pub old_withdrawal_root: [u8; 32],
        pub new_withdrawal_root: [u8; 32],
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
        name = "WithdrawalQueued",
        abi = "WithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))"
    )]
    pub struct WithdrawalQueuedFilter {
        pub withdrawal_root: [u8; 32],
        pub withdrawal: Withdrawal,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DelegationMockEvents {
        MinWithdrawalDelayBlocksSetFilter(MinWithdrawalDelayBlocksSetFilter),
        OperatorDetailsModifiedFilter(OperatorDetailsModifiedFilter),
        OperatorMetadataURIUpdatedFilter(OperatorMetadataURIUpdatedFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSharesDecreasedFilter(OperatorSharesDecreasedFilter),
        OperatorSharesIncreasedFilter(OperatorSharesIncreasedFilter),
        StakerDelegatedFilter(StakerDelegatedFilter),
        StakerForceUndelegatedFilter(StakerForceUndelegatedFilter),
        StakerUndelegatedFilter(StakerUndelegatedFilter),
        StrategyWithdrawalDelayBlocksSetFilter(StrategyWithdrawalDelayBlocksSetFilter),
        WithdrawalCompletedFilter(WithdrawalCompletedFilter),
        WithdrawalMigratedFilter(WithdrawalMigratedFilter),
        WithdrawalQueuedFilter(WithdrawalQueuedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DelegationMockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinWithdrawalDelayBlocksSetFilter::decode_log(log) {
                return Ok(
                    DelegationMockEvents::MinWithdrawalDelayBlocksSetFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorDetailsModifiedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::OperatorDetailsModifiedFilter(decoded));
            }
            if let Ok(decoded) = OperatorMetadataURIUpdatedFilter::decode_log(log) {
                return Ok(
                    DelegationMockEvents::OperatorMetadataURIUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(DelegationMockEvents::OperatorRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OperatorSharesDecreasedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::OperatorSharesDecreasedFilter(decoded));
            }
            if let Ok(decoded) = OperatorSharesIncreasedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::OperatorSharesIncreasedFilter(decoded));
            }
            if let Ok(decoded) = StakerDelegatedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::StakerDelegatedFilter(decoded));
            }
            if let Ok(decoded) = StakerForceUndelegatedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::StakerForceUndelegatedFilter(decoded));
            }
            if let Ok(decoded) = StakerUndelegatedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::StakerUndelegatedFilter(decoded));
            }
            if let Ok(decoded) = StrategyWithdrawalDelayBlocksSetFilter::decode_log(
                log,
            ) {
                return Ok(
                    DelegationMockEvents::StrategyWithdrawalDelayBlocksSetFilter(decoded),
                );
            }
            if let Ok(decoded) = WithdrawalCompletedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::WithdrawalCompletedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalMigratedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::WithdrawalMigratedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalQueuedFilter::decode_log(log) {
                return Ok(DelegationMockEvents::WithdrawalQueuedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DelegationMockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinWithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDetailsModifiedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorMetadataURIUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSharesDecreasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSharesIncreasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerDelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerForceUndelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerUndelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyWithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalCompletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalMigratedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MinWithdrawalDelayBlocksSetFilter>
    for DelegationMockEvents {
        fn from(value: MinWithdrawalDelayBlocksSetFilter) -> Self {
            Self::MinWithdrawalDelayBlocksSetFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDetailsModifiedFilter> for DelegationMockEvents {
        fn from(value: OperatorDetailsModifiedFilter) -> Self {
            Self::OperatorDetailsModifiedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorMetadataURIUpdatedFilter>
    for DelegationMockEvents {
        fn from(value: OperatorMetadataURIUpdatedFilter) -> Self {
            Self::OperatorMetadataURIUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter> for DelegationMockEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSharesDecreasedFilter> for DelegationMockEvents {
        fn from(value: OperatorSharesDecreasedFilter) -> Self {
            Self::OperatorSharesDecreasedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSharesIncreasedFilter> for DelegationMockEvents {
        fn from(value: OperatorSharesIncreasedFilter) -> Self {
            Self::OperatorSharesIncreasedFilter(value)
        }
    }
    impl ::core::convert::From<StakerDelegatedFilter> for DelegationMockEvents {
        fn from(value: StakerDelegatedFilter) -> Self {
            Self::StakerDelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StakerForceUndelegatedFilter> for DelegationMockEvents {
        fn from(value: StakerForceUndelegatedFilter) -> Self {
            Self::StakerForceUndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StakerUndelegatedFilter> for DelegationMockEvents {
        fn from(value: StakerUndelegatedFilter) -> Self {
            Self::StakerUndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyWithdrawalDelayBlocksSetFilter>
    for DelegationMockEvents {
        fn from(value: StrategyWithdrawalDelayBlocksSetFilter) -> Self {
            Self::StrategyWithdrawalDelayBlocksSetFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalCompletedFilter> for DelegationMockEvents {
        fn from(value: WithdrawalCompletedFilter) -> Self {
            Self::WithdrawalCompletedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalMigratedFilter> for DelegationMockEvents {
        fn from(value: WithdrawalMigratedFilter) -> Self {
            Self::WithdrawalMigratedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalQueuedFilter> for DelegationMockEvents {
        fn from(value: WithdrawalQueuedFilter) -> Self {
            Self::WithdrawalQueuedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DELEGATION_APPROVAL_TYPEHASH` function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`
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
        name = "DELEGATION_APPROVAL_TYPEHASH",
        abi = "DELEGATION_APPROVAL_TYPEHASH()"
    )]
    pub struct DelegationApprovalTypehashCall;
    ///Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    ///Container type for all input parameters for the `STAKER_DELEGATION_TYPEHASH` function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`
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
    #[ethcall(name = "STAKER_DELEGATION_TYPEHASH", abi = "STAKER_DELEGATION_TYPEHASH()")]
    pub struct StakerDelegationTypehashCall;
    ///Container type for all input parameters for the `addShares` function with signature `addShares(address,address,address,address,uint256)` and selector `0xbc56ff66`
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
        name = "addShares",
        abi = "addShares(address,address,address,address,uint256)"
    )]
    pub struct AddSharesCall {
        pub strategy_manager: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateApproverDigestHash` function with signature `calculateApproverDigestHash(address,address,uint256)` and selector `0x74d898d7`
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
        name = "calculateApproverDigestHash",
        abi = "calculateApproverDigestHash(address,address,uint256)"
    )]
    pub struct CalculateApproverDigestHashCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `calculateCurrentStakerDelegationDigestHash` function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`
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
        name = "calculateCurrentStakerDelegationDigestHash",
        abi = "calculateCurrentStakerDelegationDigestHash(address,address,uint256)"
    )]
    pub struct CalculateCurrentStakerDelegationDigestHashCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `calculateDelegationApprovalDigestHash` function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`
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
        name = "calculateDelegationApprovalDigestHash",
        abi = "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)"
    )]
    pub struct CalculateDelegationApprovalDigestHashCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub [u8; 32],
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `calculateOperatorAVSRegistrationDigestHash` function with signature `calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)` and selector `0xa1060c88`
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
        name = "calculateOperatorAVSRegistrationDigestHash",
        abi = "calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)"
    )]
    pub struct CalculateOperatorAVSRegistrationDigestHashCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub [u8; 32],
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `calculateStakerDelegationDigestHash` function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`
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
        name = "calculateStakerDelegationDigestHash",
        abi = "calculateStakerDelegationDigestHash(address,uint256,address,uint256)"
    )]
    pub struct CalculateStakerDelegationDigestHashCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `calculateStakerDigestHash` function with signature `calculateStakerDigestHash(address,address,uint256)` and selector `0x96568271`
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
        name = "calculateStakerDigestHash",
        abi = "calculateStakerDigestHash(address,address,uint256)"
    )]
    pub struct CalculateStakerDigestHashCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`
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
        name = "calculateWithdrawalRoot",
        abi = "calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))"
    )]
    pub struct CalculateWithdrawalRootCall {
        pub withdrawal: Withdrawal,
    }
    ///Container type for all input parameters for the `completeQueuedWithdrawal` function with signature `completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)` and selector `0x60d7faed`
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
        name = "completeQueuedWithdrawal",
        abi = "completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)"
    )]
    pub struct CompleteQueuedWithdrawalCall {
        pub withdrawal: Withdrawal,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub middleware_times_index: ::ethers::core::types::U256,
        pub receive_as_tokens: bool,
    }
    ///Container type for all input parameters for the `completeQueuedWithdrawals` function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])` and selector `0x33404396`
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
        name = "completeQueuedWithdrawals",
        abi = "completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])"
    )]
    pub struct CompleteQueuedWithdrawalsCall {
        pub withdrawals: ::std::vec::Vec<Withdrawal>,
        pub tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        pub middleware_times_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
        pub receive_as_tokens: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
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
        name = "cumulativeWithdrawalsQueued",
        abi = "cumulativeWithdrawalsQueued(address)"
    )]
    pub struct CumulativeWithdrawalsQueuedCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decreaseDelegatedShares` function with signature `decreaseDelegatedShares(address,address,uint256)` and selector `0x132d4967`
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
        name = "decreaseDelegatedShares",
        abi = "decreaseDelegatedShares(address,address,uint256)"
    )]
    pub struct DecreaseDelegatedSharesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `delegateTo` function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`
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
    #[ethcall(name = "delegateTo", abi = "delegateTo(address,(bytes,uint256),bytes32)")]
    pub struct DelegateToCall {
        pub operator: ::ethers::core::types::Address,
        pub p1: SignatureWithExpiry,
        pub p2: [u8; 32],
    }
    ///Container type for all input parameters for the `delegateToBySignature` function with signature `delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)` and selector `0x7f548071`
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
        name = "delegateToBySignature",
        abi = "delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)"
    )]
    pub struct DelegateToBySignatureCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub SignatureWithExpiry,
        pub SignatureWithExpiry,
        pub [u8; 32],
    );
    ///Container type for all input parameters for the `delegatedTo` function with signature `delegatedTo(address)` and selector `0x65da1264`
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
    #[ethcall(name = "delegatedTo", abi = "delegatedTo(address)")]
    pub struct DelegatedToCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `delegationApprover` function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`
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
    #[ethcall(name = "delegationApprover", abi = "delegationApprover(address)")]
    pub struct DelegationApproverCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `delegationApproverSaltIsSpent` function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`
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
        name = "delegationApproverSaltIsSpent",
        abi = "delegationApproverSaltIsSpent(address,bytes32)"
    )]
    pub struct DelegationApproverSaltIsSpentCall(
        pub ::ethers::core::types::Address,
        pub [u8; 32],
    );
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `earningsReceiver` function with signature `earningsReceiver(address)` and selector `0x5f966f14`
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
    #[ethcall(name = "earningsReceiver", abi = "earningsReceiver(address)")]
    pub struct EarningsReceiverCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOperatorShares` function with signature `getOperatorShares(address,address[])` and selector `0x90041347`
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
    #[ethcall(name = "getOperatorShares", abi = "getOperatorShares(address,address[])")]
    pub struct GetOperatorSharesCall {
        pub operator: ::ethers::core::types::Address,
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getWithdrawalDelay` function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`
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
    #[ethcall(name = "getWithdrawalDelay", abi = "getWithdrawalDelay(address[])")]
    pub struct GetWithdrawalDelayCall(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all input parameters for the `increaseDelegatedShares` function with signature `increaseDelegatedShares(address,address,uint256)` and selector `0x28a573ae`
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
        name = "increaseDelegatedShares",
        abi = "increaseDelegatedShares(address,address,uint256)"
    )]
    pub struct IncreaseDelegatedSharesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `isDelegated` function with signature `isDelegated(address)` and selector `0x3e28391d`
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
    #[ethcall(name = "isDelegated", abi = "isDelegated(address)")]
    pub struct IsDelegatedCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isNotDelegated` function with signature `isNotDelegated(address)` and selector `0x1d3696b7`
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
    #[ethcall(name = "isNotDelegated", abi = "isNotDelegated(address)")]
    pub struct IsNotDelegatedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isOperator` function with signature `isOperator(address)` and selector `0x6d70f7ae`
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
    #[ethcall(name = "isOperator", abi = "isOperator(address)")]
    pub struct IsOperatorCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `migrateQueuedWithdrawals` function with signature `migrateQueuedWithdrawals((address[],uint256[],address,(address,uint96),uint32,address)[])` and selector `0x5cfe8d2c`
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
        name = "migrateQueuedWithdrawals",
        abi = "migrateQueuedWithdrawals((address[],uint256[],address,(address,uint96),uint32,address)[])"
    )]
    pub struct MigrateQueuedWithdrawalsCall {
        pub withdrawals_to_queue: ::std::vec::Vec<DeprecatedStructQueuedWithdrawal>,
    }
    ///Container type for all input parameters for the `minWithdrawalDelayBlocks` function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`
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
    #[ethcall(name = "minWithdrawalDelayBlocks", abi = "minWithdrawalDelayBlocks()")]
    pub struct MinWithdrawalDelayBlocksCall;
    ///Container type for all input parameters for the `modifyOperatorDetails` function with signature `modifyOperatorDetails((address,address,uint32))` and selector `0xf16172b0`
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
        name = "modifyOperatorDetails",
        abi = "modifyOperatorDetails((address,address,uint32))"
    )]
    pub struct ModifyOperatorDetailsCall(pub OperatorDetails);
    ///Container type for all input parameters for the `operatorDetails` function with signature `operatorDetails(address)` and selector `0xc5e480db`
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
    #[ethcall(name = "operatorDetails", abi = "operatorDetails(address)")]
    pub struct OperatorDetailsCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorSaltIsSpent` function with signature `operatorSaltIsSpent(address,bytes32)` and selector `0x374823b5`
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
        name = "operatorSaltIsSpent",
        abi = "operatorSaltIsSpent(address,bytes32)"
    )]
    pub struct OperatorSaltIsSpentCall {
        pub avs: ::ethers::core::types::Address,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `operatorShares` function with signature `operatorShares(address,address)` and selector `0x778e55f3`
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
    #[ethcall(name = "operatorShares", abi = "operatorShares(address,address)")]
    pub struct OperatorSharesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `queueWithdrawals` function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`
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
        name = "queueWithdrawals",
        abi = "queueWithdrawals((address[],uint256[],address)[])"
    )]
    pub struct QueueWithdrawalsCall {
        pub queued_withdrawal_params: ::std::vec::Vec<QueuedWithdrawalParams>,
    }
    ///Container type for all input parameters for the `registerAsOperator` function with signature `registerAsOperator((address,address,uint32),string)` and selector `0x0f589e59`
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
        name = "registerAsOperator",
        abi = "registerAsOperator((address,address,uint32),string)"
    )]
    pub struct RegisterAsOperatorCall(pub OperatorDetails, pub ::std::string::String);
    ///Container type for all input parameters for the `removeShares` function with signature `removeShares(address,address,address,uint256)` and selector `0x8d90cd56`
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
        name = "removeShares",
        abi = "removeShares(address,address,address,uint256)"
    )]
    pub struct RemoveSharesCall {
        pub strategy_manager: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setIsOperator` function with signature `setIsOperator(address,bool)` and selector `0xcbb5d4db`
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
    #[ethcall(name = "setIsOperator", abi = "setIsOperator(address,bool)")]
    pub struct SetIsOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub is_operator_return_value: bool,
    }
    ///Container type for all input parameters for the `setOperatorShares` function with signature `setOperatorShares(address,address,uint256)` and selector `0xdbe35bd8`
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
        name = "setOperatorShares",
        abi = "setOperatorShares(address,address,uint256)"
    )]
    pub struct SetOperatorSharesCall {
        pub operator: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakerNonce` function with signature `stakerNonce(address)` and selector `0x29c77d4f`
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
    #[ethcall(name = "stakerNonce", abi = "stakerNonce(address)")]
    pub struct StakerNonceCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `stakerOptOutWindowBlocks` function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`
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
        name = "stakerOptOutWindowBlocks",
        abi = "stakerOptOutWindowBlocks(address)"
    )]
    pub struct StakerOptOutWindowBlocksCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `strategyWithdrawalDelayBlocks` function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`
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
        name = "strategyWithdrawalDelayBlocks",
        abi = "strategyWithdrawalDelayBlocks(address)"
    )]
    pub struct StrategyWithdrawalDelayBlocksCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `undelegate` function with signature `undelegate(address)` and selector `0xda8be864`
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
    #[ethcall(name = "undelegate", abi = "undelegate(address)")]
    pub struct UndelegateCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateAVSMetadataURI` function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`
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
    #[ethcall(name = "updateAVSMetadataURI", abi = "updateAVSMetadataURI(string)")]
    pub struct UpdateAVSMetadataURICall(pub ::std::string::String);
    ///Container type for all input parameters for the `updateOperatorMetadataURI` function with signature `updateOperatorMetadataURI(string)` and selector `0x99be81c8`
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
        name = "updateOperatorMetadataURI",
        abi = "updateOperatorMetadataURI(string)"
    )]
    pub struct UpdateOperatorMetadataURICall(pub ::std::string::String);
    ///Container type for all input parameters for the `withdrawSharesAsTokens` function with signature `withdrawSharesAsTokens(address,address,address,uint256,address)` and selector `0x67f292c7`
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
        name = "withdrawSharesAsTokens",
        abi = "withdrawSharesAsTokens(address,address,address,uint256,address)"
    )]
    pub struct WithdrawSharesAsTokensCall {
        pub strategy_manager: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DelegationMockCalls {
        DelegationApprovalTypehash(DelegationApprovalTypehashCall),
        DomainTypehash(DomainTypehashCall),
        StakerDelegationTypehash(StakerDelegationTypehashCall),
        AddShares(AddSharesCall),
        CalculateApproverDigestHash(CalculateApproverDigestHashCall),
        CalculateCurrentStakerDelegationDigestHash(
            CalculateCurrentStakerDelegationDigestHashCall,
        ),
        CalculateDelegationApprovalDigestHash(CalculateDelegationApprovalDigestHashCall),
        CalculateOperatorAVSRegistrationDigestHash(
            CalculateOperatorAVSRegistrationDigestHashCall,
        ),
        CalculateStakerDelegationDigestHash(CalculateStakerDelegationDigestHashCall),
        CalculateStakerDigestHash(CalculateStakerDigestHashCall),
        CalculateWithdrawalRoot(CalculateWithdrawalRootCall),
        CompleteQueuedWithdrawal(CompleteQueuedWithdrawalCall),
        CompleteQueuedWithdrawals(CompleteQueuedWithdrawalsCall),
        CumulativeWithdrawalsQueued(CumulativeWithdrawalsQueuedCall),
        DecreaseDelegatedShares(DecreaseDelegatedSharesCall),
        DelegateTo(DelegateToCall),
        DelegateToBySignature(DelegateToBySignatureCall),
        DelegatedTo(DelegatedToCall),
        DelegationApprover(DelegationApproverCall),
        DelegationApproverSaltIsSpent(DelegationApproverSaltIsSpentCall),
        DomainSeparator(DomainSeparatorCall),
        EarningsReceiver(EarningsReceiverCall),
        GetOperatorShares(GetOperatorSharesCall),
        GetWithdrawalDelay(GetWithdrawalDelayCall),
        IncreaseDelegatedShares(IncreaseDelegatedSharesCall),
        IsDelegated(IsDelegatedCall),
        IsNotDelegated(IsNotDelegatedCall),
        IsOperator(IsOperatorCall),
        MigrateQueuedWithdrawals(MigrateQueuedWithdrawalsCall),
        MinWithdrawalDelayBlocks(MinWithdrawalDelayBlocksCall),
        ModifyOperatorDetails(ModifyOperatorDetailsCall),
        OperatorDetails(OperatorDetailsCall),
        OperatorSaltIsSpent(OperatorSaltIsSpentCall),
        OperatorShares(OperatorSharesCall),
        QueueWithdrawals(QueueWithdrawalsCall),
        RegisterAsOperator(RegisterAsOperatorCall),
        RemoveShares(RemoveSharesCall),
        SetIsOperator(SetIsOperatorCall),
        SetOperatorShares(SetOperatorSharesCall),
        StakerNonce(StakerNonceCall),
        StakerOptOutWindowBlocks(StakerOptOutWindowBlocksCall),
        StrategyWithdrawalDelayBlocks(StrategyWithdrawalDelayBlocksCall),
        Undelegate(UndelegateCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
        UpdateOperatorMetadataURI(UpdateOperatorMetadataURICall),
        WithdrawSharesAsTokens(WithdrawSharesAsTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for DelegationMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DelegationApprovalTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationApprovalTypehash(decoded));
            }
            if let Ok(decoded) = <DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainTypehash(decoded));
            }
            if let Ok(decoded) = <StakerDelegationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerDelegationTypehash(decoded));
            }
            if let Ok(decoded) = <AddSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddShares(decoded));
            }
            if let Ok(decoded) = <CalculateApproverDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateApproverDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateCurrentStakerDelegationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateCurrentStakerDelegationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateDelegationApprovalDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDelegationApprovalDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateOperatorAVSRegistrationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOperatorAVSRegistrationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateStakerDelegationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateStakerDelegationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateStakerDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateStakerDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateWithdrawalRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateWithdrawalRoot(decoded));
            }
            if let Ok(decoded) = <CompleteQueuedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteQueuedWithdrawal(decoded));
            }
            if let Ok(decoded) = <CompleteQueuedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteQueuedWithdrawals(decoded));
            }
            if let Ok(decoded) = <CumulativeWithdrawalsQueuedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CumulativeWithdrawalsQueued(decoded));
            }
            if let Ok(decoded) = <DecreaseDelegatedSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseDelegatedShares(decoded));
            }
            if let Ok(decoded) = <DelegateToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegateTo(decoded));
            }
            if let Ok(decoded) = <DelegateToBySignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegateToBySignature(decoded));
            }
            if let Ok(decoded) = <DelegatedToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegatedTo(decoded));
            }
            if let Ok(decoded) = <DelegationApproverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationApprover(decoded));
            }
            if let Ok(decoded) = <DelegationApproverSaltIsSpentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationApproverSaltIsSpent(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <EarningsReceiverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EarningsReceiver(decoded));
            }
            if let Ok(decoded) = <GetOperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorShares(decoded));
            }
            if let Ok(decoded) = <GetWithdrawalDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWithdrawalDelay(decoded));
            }
            if let Ok(decoded) = <IncreaseDelegatedSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseDelegatedShares(decoded));
            }
            if let Ok(decoded) = <IsDelegatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDelegated(decoded));
            }
            if let Ok(decoded) = <IsNotDelegatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsNotDelegated(decoded));
            }
            if let Ok(decoded) = <IsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOperator(decoded));
            }
            if let Ok(decoded) = <MigrateQueuedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MigrateQueuedWithdrawals(decoded));
            }
            if let Ok(decoded) = <MinWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) = <ModifyOperatorDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyOperatorDetails(decoded));
            }
            if let Ok(decoded) = <OperatorDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorDetails(decoded));
            }
            if let Ok(decoded) = <OperatorSaltIsSpentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorSaltIsSpent(decoded));
            }
            if let Ok(decoded) = <OperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorShares(decoded));
            }
            if let Ok(decoded) = <QueueWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QueueWithdrawals(decoded));
            }
            if let Ok(decoded) = <RegisterAsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterAsOperator(decoded));
            }
            if let Ok(decoded) = <RemoveSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveShares(decoded));
            }
            if let Ok(decoded) = <SetIsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetIsOperator(decoded));
            }
            if let Ok(decoded) = <SetOperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOperatorShares(decoded));
            }
            if let Ok(decoded) = <StakerNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerNonce(decoded));
            }
            if let Ok(decoded) = <StakerOptOutWindowBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerOptOutWindowBlocks(decoded));
            }
            if let Ok(decoded) = <StrategyWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) = <UndelegateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Undelegate(decoded));
            }
            if let Ok(decoded) = <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorMetadataURI(decoded));
            }
            if let Ok(decoded) = <WithdrawSharesAsTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawSharesAsTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DelegationMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DelegationApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerDelegationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateApproverDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorAVSRegistrationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateStakerDelegationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateStakerDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteQueuedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteQueuedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CumulativeWithdrawalsQueued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseDelegatedShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateToBySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegatedTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationApproverSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EarningsReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWithdrawalDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseDelegatedShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDelegated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsNotDelegated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MigrateQueuedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueueWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterAsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetIsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerOptOutWindowBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Undelegate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAVSMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawSharesAsTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DelegationMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelegationApprovalTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerDelegationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateApproverDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateOperatorAVSRegistrationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateStakerDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteQueuedWithdrawal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteQueuedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CumulativeWithdrawalsQueued(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseDelegatedShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegateTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateToBySignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegatedTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegationApproverSaltIsSpent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarningsReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWithdrawalDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseDelegatedShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsDelegated(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsNotDelegated(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateQueuedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSaltIsSpent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueueWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterAsOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetIsOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerOptOutWindowBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Undelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAVSMetadataURI(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateOperatorMetadataURI(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawSharesAsTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DelegationApprovalTypehashCall> for DelegationMockCalls {
        fn from(value: DelegationApprovalTypehashCall) -> Self {
            Self::DelegationApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<DomainTypehashCall> for DelegationMockCalls {
        fn from(value: DomainTypehashCall) -> Self {
            Self::DomainTypehash(value)
        }
    }
    impl ::core::convert::From<StakerDelegationTypehashCall> for DelegationMockCalls {
        fn from(value: StakerDelegationTypehashCall) -> Self {
            Self::StakerDelegationTypehash(value)
        }
    }
    impl ::core::convert::From<AddSharesCall> for DelegationMockCalls {
        fn from(value: AddSharesCall) -> Self {
            Self::AddShares(value)
        }
    }
    impl ::core::convert::From<CalculateApproverDigestHashCall> for DelegationMockCalls {
        fn from(value: CalculateApproverDigestHashCall) -> Self {
            Self::CalculateApproverDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateCurrentStakerDelegationDigestHashCall>
    for DelegationMockCalls {
        fn from(value: CalculateCurrentStakerDelegationDigestHashCall) -> Self {
            Self::CalculateCurrentStakerDelegationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateDelegationApprovalDigestHashCall>
    for DelegationMockCalls {
        fn from(value: CalculateDelegationApprovalDigestHashCall) -> Self {
            Self::CalculateDelegationApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorAVSRegistrationDigestHashCall>
    for DelegationMockCalls {
        fn from(value: CalculateOperatorAVSRegistrationDigestHashCall) -> Self {
            Self::CalculateOperatorAVSRegistrationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateStakerDelegationDigestHashCall>
    for DelegationMockCalls {
        fn from(value: CalculateStakerDelegationDigestHashCall) -> Self {
            Self::CalculateStakerDelegationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateStakerDigestHashCall> for DelegationMockCalls {
        fn from(value: CalculateStakerDigestHashCall) -> Self {
            Self::CalculateStakerDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateWithdrawalRootCall> for DelegationMockCalls {
        fn from(value: CalculateWithdrawalRootCall) -> Self {
            Self::CalculateWithdrawalRoot(value)
        }
    }
    impl ::core::convert::From<CompleteQueuedWithdrawalCall> for DelegationMockCalls {
        fn from(value: CompleteQueuedWithdrawalCall) -> Self {
            Self::CompleteQueuedWithdrawal(value)
        }
    }
    impl ::core::convert::From<CompleteQueuedWithdrawalsCall> for DelegationMockCalls {
        fn from(value: CompleteQueuedWithdrawalsCall) -> Self {
            Self::CompleteQueuedWithdrawals(value)
        }
    }
    impl ::core::convert::From<CumulativeWithdrawalsQueuedCall> for DelegationMockCalls {
        fn from(value: CumulativeWithdrawalsQueuedCall) -> Self {
            Self::CumulativeWithdrawalsQueued(value)
        }
    }
    impl ::core::convert::From<DecreaseDelegatedSharesCall> for DelegationMockCalls {
        fn from(value: DecreaseDelegatedSharesCall) -> Self {
            Self::DecreaseDelegatedShares(value)
        }
    }
    impl ::core::convert::From<DelegateToCall> for DelegationMockCalls {
        fn from(value: DelegateToCall) -> Self {
            Self::DelegateTo(value)
        }
    }
    impl ::core::convert::From<DelegateToBySignatureCall> for DelegationMockCalls {
        fn from(value: DelegateToBySignatureCall) -> Self {
            Self::DelegateToBySignature(value)
        }
    }
    impl ::core::convert::From<DelegatedToCall> for DelegationMockCalls {
        fn from(value: DelegatedToCall) -> Self {
            Self::DelegatedTo(value)
        }
    }
    impl ::core::convert::From<DelegationApproverCall> for DelegationMockCalls {
        fn from(value: DelegationApproverCall) -> Self {
            Self::DelegationApprover(value)
        }
    }
    impl ::core::convert::From<DelegationApproverSaltIsSpentCall>
    for DelegationMockCalls {
        fn from(value: DelegationApproverSaltIsSpentCall) -> Self {
            Self::DelegationApproverSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for DelegationMockCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EarningsReceiverCall> for DelegationMockCalls {
        fn from(value: EarningsReceiverCall) -> Self {
            Self::EarningsReceiver(value)
        }
    }
    impl ::core::convert::From<GetOperatorSharesCall> for DelegationMockCalls {
        fn from(value: GetOperatorSharesCall) -> Self {
            Self::GetOperatorShares(value)
        }
    }
    impl ::core::convert::From<GetWithdrawalDelayCall> for DelegationMockCalls {
        fn from(value: GetWithdrawalDelayCall) -> Self {
            Self::GetWithdrawalDelay(value)
        }
    }
    impl ::core::convert::From<IncreaseDelegatedSharesCall> for DelegationMockCalls {
        fn from(value: IncreaseDelegatedSharesCall) -> Self {
            Self::IncreaseDelegatedShares(value)
        }
    }
    impl ::core::convert::From<IsDelegatedCall> for DelegationMockCalls {
        fn from(value: IsDelegatedCall) -> Self {
            Self::IsDelegated(value)
        }
    }
    impl ::core::convert::From<IsNotDelegatedCall> for DelegationMockCalls {
        fn from(value: IsNotDelegatedCall) -> Self {
            Self::IsNotDelegated(value)
        }
    }
    impl ::core::convert::From<IsOperatorCall> for DelegationMockCalls {
        fn from(value: IsOperatorCall) -> Self {
            Self::IsOperator(value)
        }
    }
    impl ::core::convert::From<MigrateQueuedWithdrawalsCall> for DelegationMockCalls {
        fn from(value: MigrateQueuedWithdrawalsCall) -> Self {
            Self::MigrateQueuedWithdrawals(value)
        }
    }
    impl ::core::convert::From<MinWithdrawalDelayBlocksCall> for DelegationMockCalls {
        fn from(value: MinWithdrawalDelayBlocksCall) -> Self {
            Self::MinWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<ModifyOperatorDetailsCall> for DelegationMockCalls {
        fn from(value: ModifyOperatorDetailsCall) -> Self {
            Self::ModifyOperatorDetails(value)
        }
    }
    impl ::core::convert::From<OperatorDetailsCall> for DelegationMockCalls {
        fn from(value: OperatorDetailsCall) -> Self {
            Self::OperatorDetails(value)
        }
    }
    impl ::core::convert::From<OperatorSaltIsSpentCall> for DelegationMockCalls {
        fn from(value: OperatorSaltIsSpentCall) -> Self {
            Self::OperatorSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<OperatorSharesCall> for DelegationMockCalls {
        fn from(value: OperatorSharesCall) -> Self {
            Self::OperatorShares(value)
        }
    }
    impl ::core::convert::From<QueueWithdrawalsCall> for DelegationMockCalls {
        fn from(value: QueueWithdrawalsCall) -> Self {
            Self::QueueWithdrawals(value)
        }
    }
    impl ::core::convert::From<RegisterAsOperatorCall> for DelegationMockCalls {
        fn from(value: RegisterAsOperatorCall) -> Self {
            Self::RegisterAsOperator(value)
        }
    }
    impl ::core::convert::From<RemoveSharesCall> for DelegationMockCalls {
        fn from(value: RemoveSharesCall) -> Self {
            Self::RemoveShares(value)
        }
    }
    impl ::core::convert::From<SetIsOperatorCall> for DelegationMockCalls {
        fn from(value: SetIsOperatorCall) -> Self {
            Self::SetIsOperator(value)
        }
    }
    impl ::core::convert::From<SetOperatorSharesCall> for DelegationMockCalls {
        fn from(value: SetOperatorSharesCall) -> Self {
            Self::SetOperatorShares(value)
        }
    }
    impl ::core::convert::From<StakerNonceCall> for DelegationMockCalls {
        fn from(value: StakerNonceCall) -> Self {
            Self::StakerNonce(value)
        }
    }
    impl ::core::convert::From<StakerOptOutWindowBlocksCall> for DelegationMockCalls {
        fn from(value: StakerOptOutWindowBlocksCall) -> Self {
            Self::StakerOptOutWindowBlocks(value)
        }
    }
    impl ::core::convert::From<StrategyWithdrawalDelayBlocksCall>
    for DelegationMockCalls {
        fn from(value: StrategyWithdrawalDelayBlocksCall) -> Self {
            Self::StrategyWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<UndelegateCall> for DelegationMockCalls {
        fn from(value: UndelegateCall) -> Self {
            Self::Undelegate(value)
        }
    }
    impl ::core::convert::From<UpdateAVSMetadataURICall> for DelegationMockCalls {
        fn from(value: UpdateAVSMetadataURICall) -> Self {
            Self::UpdateAVSMetadataURI(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorMetadataURICall> for DelegationMockCalls {
        fn from(value: UpdateOperatorMetadataURICall) -> Self {
            Self::UpdateOperatorMetadataURI(value)
        }
    }
    impl ::core::convert::From<WithdrawSharesAsTokensCall> for DelegationMockCalls {
        fn from(value: WithdrawSharesAsTokensCall) -> Self {
            Self::WithdrawSharesAsTokens(value)
        }
    }
    ///Container type for all return fields from the `DELEGATION_APPROVAL_TYPEHASH` function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`
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
    pub struct DelegationApprovalTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    pub struct DomainTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKER_DELEGATION_TYPEHASH` function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`
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
    pub struct StakerDelegationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateApproverDigestHash` function with signature `calculateApproverDigestHash(address,address,uint256)` and selector `0x74d898d7`
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
    pub struct CalculateApproverDigestHashReturn {
        pub approver_digest_hash: [u8; 32],
    }
    ///Container type for all return fields from the `calculateCurrentStakerDelegationDigestHash` function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`
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
    pub struct CalculateCurrentStakerDelegationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateDelegationApprovalDigestHash` function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`
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
    pub struct CalculateDelegationApprovalDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateOperatorAVSRegistrationDigestHash` function with signature `calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)` and selector `0xa1060c88`
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
    pub struct CalculateOperatorAVSRegistrationDigestHashReturn {
        pub digest_hash: [u8; 32],
    }
    ///Container type for all return fields from the `calculateStakerDelegationDigestHash` function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`
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
    pub struct CalculateStakerDelegationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateStakerDigestHash` function with signature `calculateStakerDigestHash(address,address,uint256)` and selector `0x96568271`
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
    pub struct CalculateStakerDigestHashReturn {
        pub staker_digest_hash: [u8; 32],
    }
    ///Container type for all return fields from the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`
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
    pub struct CalculateWithdrawalRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
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
    pub struct CumulativeWithdrawalsQueuedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegatedTo` function with signature `delegatedTo(address)` and selector `0x65da1264`
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
    pub struct DelegatedToReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationApprover` function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`
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
    pub struct DelegationApproverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationApproverSaltIsSpent` function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`
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
    pub struct DelegationApproverSaltIsSpentReturn(pub bool);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `earningsReceiver` function with signature `earningsReceiver(address)` and selector `0x5f966f14`
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
    pub struct EarningsReceiverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getOperatorShares` function with signature `getOperatorShares(address,address[])` and selector `0x90041347`
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
    pub struct GetOperatorSharesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getWithdrawalDelay` function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`
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
    pub struct GetWithdrawalDelayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isDelegated` function with signature `isDelegated(address)` and selector `0x3e28391d`
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
    pub struct IsDelegatedReturn(pub bool);
    ///Container type for all return fields from the `isNotDelegated` function with signature `isNotDelegated(address)` and selector `0x1d3696b7`
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
    pub struct IsNotDelegatedReturn(pub bool);
    ///Container type for all return fields from the `isOperator` function with signature `isOperator(address)` and selector `0x6d70f7ae`
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
    pub struct IsOperatorReturn(pub bool);
    ///Container type for all return fields from the `minWithdrawalDelayBlocks` function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`
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
    pub struct MinWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operatorDetails` function with signature `operatorDetails(address)` and selector `0xc5e480db`
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
    pub struct OperatorDetailsReturn(pub OperatorDetails);
    ///Container type for all return fields from the `operatorSaltIsSpent` function with signature `operatorSaltIsSpent(address,bytes32)` and selector `0x374823b5`
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
    pub struct OperatorSaltIsSpentReturn(pub bool);
    ///Container type for all return fields from the `operatorShares` function with signature `operatorShares(address,address)` and selector `0x778e55f3`
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
    pub struct OperatorSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queueWithdrawals` function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`
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
    pub struct QueueWithdrawalsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `stakerNonce` function with signature `stakerNonce(address)` and selector `0x29c77d4f`
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
    pub struct StakerNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerOptOutWindowBlocks` function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`
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
    pub struct StakerOptOutWindowBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `strategyWithdrawalDelayBlocks` function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`
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
    pub struct StrategyWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `undelegate` function with signature `undelegate(address)` and selector `0xda8be864`
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
    pub struct UndelegateReturn {
        pub withdrawal_root: ::std::vec::Vec<[u8; 32]>,
    }
}
