pub use strategy_base_tvl_limits::*;
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
pub mod strategy_base_tvl_limits {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategyManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStrategyManager"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newShares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("explanation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("explanation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTVLLimits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTVLLimits"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxPerDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxTotalDeposits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_underlyingToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_underlyingToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("maxPerDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxPerDeposit"),
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
                    ::std::borrow::ToOwned::to_owned("maxTotalDeposits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxTotalDeposits"),
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
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
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
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
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("setTVLLimits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTVLLimits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxPerDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxTotalDeposits",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("shares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
                    ::std::borrow::ToOwned::to_owned("sharesToUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sharesToUnderlying"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountShares"),
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
                    ::std::borrow::ToOwned::to_owned("sharesToUnderlyingView"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sharesToUnderlyingView",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountShares"),
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
                    ::std::borrow::ToOwned::to_owned("strategyManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStrategyManager",
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
                    ::std::borrow::ToOwned::to_owned("totalShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalShares"),
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
                    ::std::borrow::ToOwned::to_owned("underlyingToShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underlyingToShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountUnderlying"),
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
                    ::std::borrow::ToOwned::to_owned("underlyingToSharesView"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "underlyingToSharesView",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountUnderlying"),
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
                    ::std::borrow::ToOwned::to_owned("underlyingToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underlyingToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
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
                    ::std::borrow::ToOwned::to_owned("userUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("userUnderlying"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userUnderlyingView"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("userUnderlyingView"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountShares"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxPerDepositUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxPerDepositUpdated",
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
                    ::std::borrow::ToOwned::to_owned("MaxTotalDepositsUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxTotalDepositsUpdated",
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
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
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
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STRATEGYBASETVLLIMITS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1DV8\x03\x80b\0\x1DV\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x16V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80b\0\0Lb\0\0TV[PPb\0\x01HV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x14W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01)W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01AW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1B\xDDb\0\x01y`\09`\0\x81\x81a\x02\x16\x01R\x81\x81a\x07\xA9\x01R\x81\x81a\x0B\xC3\x01Ra\x0C\x8E\x01Ra\x1B\xDD`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\xDEW\x80c\xABY!\xE1\x11a\0\x97W\x80c\xDFo\xAD\xC1\x11a\0qW\x80c\xDFo\xAD\xC1\x14a\x03fW\x80c\xE3\xDA\xE5\x1C\x14a\x03\x81W\x80c\xF3\xE78u\x14a\x03\x94W\x80c\xFA\xBC\x1C\xBC\x14a\x03\xA7W`\0\x80\xFD[\x80c\xABY!\xE1\x14a\x03+W\x80c\xCE|*\xC2\x14a\x03@W\x80c\xD9\xCA\xED\x12\x14a\x03SW`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x02\xC8W\x80ca\xB0\x1B]\x14a\x02\xD0W\x80cz\x8B&7\x14a\x02\xD9W\x80c\x88o\x11\x95\x14a\x02\xECW\x80c\x8C\x87\x10\x19\x14a\x03\x05W\x80c\x8Fjb@\x14a\x03\x18W`\0\x80\xFD[\x80c:\x98\xEF9\x11a\x01KW\x80cH\\\xC9U\x11a\x01%W\x80cH\\\xC9U\x14a\x02kW\x80cU<\xA5\xF8\x14a\x02~W\x80cY\\jg\x14a\x02\x91W\x80cZ\xC8j\xB7\x14a\x02\x99W`\0\x80\xFD[\x80c:\x98\xEF9\x14a\x028W\x80cC\xFE\x08\xB0\x14a\x02OW\x80cG\xE7\xEF$\x14a\x02XW`\0\x80\xFD[\x80c\x01\x9E')\x14a\x01\x93W\x80c\x10\xD6z/\x14a\x01\xA8W\x80c\x11\xC7\x0C\x9D\x14a\x01\xBBW\x80c\x13d9\xDD\x14a\x01\xCEW\x80c$\x95\xA5\x99\x14a\x01\xE1W\x80c9\xB7\x0E8\x14a\x02\x11W[`\0\x80\xFD[a\x01\xA6a\x01\xA16`\x04a\x17\xB2V[a\x03\xBAV[\0[a\x01\xA6a\x01\xB66`\x04a\x17\xFCV[a\x04\x9DV[a\x01\xA6a\x01\xC96`\x04a\x18\x19V[a\x05PV[a\x01\xA6a\x01\xDC6`\x04a\x18;V[a\x06\x05V[`2Ta\x01\xF4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02A`3T\x81V[`@Q\x90\x81R` \x01a\x02\x08V[a\x02A`dT\x81V[a\x02Aa\x02f6`\x04a\x18TV[a\x07IV[a\x01\xA6a\x02y6`\x04a\x18\x80V[a\tiV[a\x02Aa\x02\x8C6`\x04a\x17\xFCV[a\n7V[a\x01\xA6a\nKV[a\x02\xB8a\x02\xA76`\x04a\x18\xB9V[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x08V[`\x01Ta\x02AV[a\x02A`eT\x81V[a\x02Aa\x02\xE76`\x04a\x18;V[a\x0B\x17V[`\0Ta\x01\xF4\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02Aa\x03\x136`\x04a\x18;V[a\x0BbV[a\x02Aa\x03&6`\x04a\x17\xFCV[a\x0BmV[a\x033a\x0B{V[`@Qa\x02\x08\x91\x90a\x19\x0CV[a\x02Aa\x03N6`\x04a\x17\xFCV[a\x0B\x9BV[a\x01\xA6a\x03a6`\x04a\x19?V[a\x0C0V[`dT`eT`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x08V[a\x02Aa\x03\x8F6`\x04a\x18;V[a\x0E}V[a\x02Aa\x03\xA26`\x04a\x18;V[a\x0E\xB6V[a\x01\xA6a\x03\xB56`\x04a\x18;V[a\x0E\xC1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x03\xDAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x03\xF4WP0;\x15\x80\x15a\x03\xF4WP`\0T`\xFF\x16`\x01\x14[a\x04\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\x80V[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x04<W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x04F\x85\x85a\x10\x1DV[a\x04P\x83\x83a\x11*V[\x80\x15a\x04\x96W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x14\x91\x90a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\xEBV[a\x05M\x81a\x11\xBBV[PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\xEBV[a\x06\x01\x82\x82a\x10\x1DV[PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06v\x91\x90a\x1A5V[a\x06\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x1AWV[`\x01T\x81\x81\x16\x14a\x07\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x01\x80T`\0\x91\x82\x91\x81\x16\x14\x15a\x07\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x04\x10V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrategyBase.onlyStrategyManager`D\x82\x01R`d\x01a\x04\x10V[a\x08 \x84\x84a\x12\xC0V[`2T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14a\x08\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FStrategyBase.deposit: Can only d`D\x82\x01Ru2\xB87\xB9\xB4\xBA\x10:\xB722\xB96<\xB4\xB73\xAA7\xB5\xB2\xB7`Q\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`3T`\0a\x08\xADa\x03\xE8\x83a\x1A\xB5V[\x90P`\0a\x03\xE8a\x08\xBCa\x13\x98V[a\x08\xC6\x91\x90a\x1A\xB5V[\x90P`\0a\x08\xD4\x87\x83a\x1A\xCDV[\x90P\x80a\x08\xE1\x84\x89a\x1A\xE4V[a\x08\xEB\x91\x90a\x1B\x03V[\x95P\x85a\tQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FStrategyBase.deposit: newShares `D\x82\x01Rmcannot be zero`\x90\x1B`d\x82\x01R`\x84\x01a\x04\x10V[a\t[\x86\x85a\x1A\xB5V[`3UPPPPP\x92\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\x89WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xA3WP0;\x15\x80\x15a\t\xA3WP`\0T`\xFF\x16`\x01\x14[a\t\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\x80V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\t\xE2W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\t\xEC\x83\x83a\x11*V[\x80\x15a\n2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0a\nEa\x02\xE7\x83a\x0B\x9BV[\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBC\x91\x90a\x1A5V[a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x1AWV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x80a\x03\xE8`3Ta\x0B*\x91\x90a\x1A\xB5V[\x90P`\0a\x03\xE8a\x0B9a\x13\x98V[a\x0BC\x91\x90a\x1A\xB5V[\x90P\x81a\x0BP\x85\x83a\x1A\xE4V[a\x0BZ\x91\x90a\x1B\x03V[\x94\x93PPPPV[`\0a\nE\x82a\x0E}V[`\0a\nEa\x03\xA2\x83a\x0B\x9BV[```@Q\x80`\x80\x01`@R\x80`M\x81R` \x01a\x1B[`M\x919\x90P\x90V[`@Qc=?\x06\xC9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R0`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cz~\r\x92\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nE\x91\x90a\x1B%V[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x0C\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x04\x10V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrategyBase.onlyStrategyManager`D\x82\x01R`d\x01a\x04\x10V[`2T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\r~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FStrategyBase.withdraw: Can only `D\x82\x01R\x7Fwithdraw the strategy token\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[`3T\x80\x83\x11\x15a\x0E\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FStrategyBase.withdraw: amountSha`D\x82\x01R\x7Fres must be less than or equal t`d\x82\x01Rlo totalShares`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`\0a\x0E\x1Ba\x03\xE8\x83a\x1A\xB5V[\x90P`\0a\x03\xE8a\x0E*a\x13\x98V[a\x0E4\x91\x90a\x1A\xB5V[\x90P`\0\x82a\x0EC\x87\x84a\x1A\xE4V[a\x0EM\x91\x90a\x1B\x03V[\x90Pa\x0EY\x86\x85a\x1A\xCDV[`3U`2Ta\x0Es\x90`\x01`\x01`\xA0\x1B\x03\x16\x89\x83a\x14\nV[PPPPPPPPV[`\0\x80a\x03\xE8`3Ta\x0E\x90\x91\x90a\x1A\xB5V[\x90P`\0a\x03\xE8a\x0E\x9Fa\x13\x98V[a\x0E\xA9\x91\x90a\x1A\xB5V[\x90P\x80a\x0BP\x83\x86a\x1A\xE4V[`\0a\nE\x82a\x0B\x17V[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F8\x91\x90a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FhW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\xEBV[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a\x0F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07>V[`dT`@\x80Q\x91\x82R` \x82\x01\x84\x90R\x7F\xF9~\xD4\xE0\x83\xAC\xACg\x83\0%\xEC\xBCum\x8F\xE8G\xCD\xBD\xCAL\xEE?\xE1\xE1(\xE9\x8BT\xEC\xB5\x91\x01`@Q\x80\x91\x03\x90\xA1`eT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7Fj\xB1\x81\xE0D\x0B\xFB\xF4\xBA\xCD\xF2\xE9\x96ts\\\xE6c\x80\x05I\x06\x88\xC5\xF9\x94\xF59\x93S\xE4R\x91\x01`@Q\x80\x91\x03\x90\xA1\x80\x82\x11\x15a\x11\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FStrategyBaseTVLLimits._setTVLLim`D\x82\x01R\x7Fits: maxPerDeposit exceeds maxTo`d\x82\x01RjtalDeposits`\xA8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`d\x91\x90\x91U`eUV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x11\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`2\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ua\x06\x01\x81`\0a\x14\\V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`dT\x81\x11\x15a\x13*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FStrategyBaseTVLLimits: max per d`D\x82\x01Rn\x19\\\x1B\xDC\xDA]\x08\x19^\x18\xD9YY\x19Y`\x8A\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`eTa\x135a\x13\x98V[\x11\x15a\x06\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FStrategyBaseTVLLimits: max depos`D\x82\x01Rk\x1A]\x1C\xC8\x19^\x18\xD9YY\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`2T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x05\x91\x90a\x1B%V[\x90P\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\n2\x90\x84\x90a\x15HV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x14\x83WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\x15\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x06\x01\x82a\x11\xBBV[`\0a\x15\x9D\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x16\x1A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\n2W\x80\x80` \x01\x90Q\x81\x01\x90a\x15\xBB\x91\x90a\x1A5V[a\n2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x04\x10V[``a\x16)\x84\x84`\0\x85a\x163V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x16\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x16\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x04\x10V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x17\x07\x91\x90a\x1B>V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x17DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x17IV[``\x91P[P\x91P\x91Pa\x17Y\x82\x82\x86a\x17dV[\x97\x96PPPPPPPV[``\x83\x15a\x17sWP\x81a\x16,V[\x82Q\x15a\x17\x83W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x91\x90a\x19\x0CV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05MW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x17\xC8W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015a\x17\xE1\x81a\x17\x9DV[\x91P``\x85\x015a\x17\xF1\x81a\x17\x9DV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x18\x0EW`\0\x80\xFD[\x815a\x16,\x81a\x17\x9DV[`\0\x80`@\x83\x85\x03\x12\x15a\x18,W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x18MW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x18gW`\0\x80\xFD[\x825a\x18r\x81a\x17\x9DV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x93W`\0\x80\xFD[\x825a\x18\x9E\x81a\x17\x9DV[\x91P` \x83\x015a\x18\xAE\x81a\x17\x9DV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xCBW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x16,W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x18\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18\xDFV[\x83\x81\x11\x15a\x19\x06W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x19+\x81`@\x85\x01` \x87\x01a\x18\xDCV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19TW`\0\x80\xFD[\x835a\x19_\x81a\x17\x9DV[\x92P` \x84\x015a\x19o\x81a\x17\x9DV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x80\x82R`.\x90\x82\x01R\x7FInitializable: contract is alrea`@\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x19\xE0W`\0\x80\xFD[\x81Qa\x16,\x81a\x17\x9DV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1AGW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x16,W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1A\xC8Wa\x1A\xC8a\x1A\x9FV[P\x01\x90V[`\0\x82\x82\x10\x15a\x1A\xDFWa\x1A\xDFa\x1A\x9FV[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1A\xFEWa\x1A\xFEa\x1A\x9FV[P\x02\x90V[`\0\x82a\x1B WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x1B7W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82Qa\x1BP\x81\x84` \x87\x01a\x18\xDCV[\x91\x90\x91\x01\x92\x91PPV\xFEBase Strategy implementation to inherit from for more complex implementations\xA2dipfsX\"\x12 \x9A4'\xEBF1\xC4`\x16\xAC\xB2tD]e\xD3\xCAI\x9B\xBE(\xB9w\xBD\x85\x17$\x0B\x02\xDA\x15KdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STRATEGYBASETVLLIMITS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\xDEW\x80c\xABY!\xE1\x11a\0\x97W\x80c\xDFo\xAD\xC1\x11a\0qW\x80c\xDFo\xAD\xC1\x14a\x03fW\x80c\xE3\xDA\xE5\x1C\x14a\x03\x81W\x80c\xF3\xE78u\x14a\x03\x94W\x80c\xFA\xBC\x1C\xBC\x14a\x03\xA7W`\0\x80\xFD[\x80c\xABY!\xE1\x14a\x03+W\x80c\xCE|*\xC2\x14a\x03@W\x80c\xD9\xCA\xED\x12\x14a\x03SW`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x02\xC8W\x80ca\xB0\x1B]\x14a\x02\xD0W\x80cz\x8B&7\x14a\x02\xD9W\x80c\x88o\x11\x95\x14a\x02\xECW\x80c\x8C\x87\x10\x19\x14a\x03\x05W\x80c\x8Fjb@\x14a\x03\x18W`\0\x80\xFD[\x80c:\x98\xEF9\x11a\x01KW\x80cH\\\xC9U\x11a\x01%W\x80cH\\\xC9U\x14a\x02kW\x80cU<\xA5\xF8\x14a\x02~W\x80cY\\jg\x14a\x02\x91W\x80cZ\xC8j\xB7\x14a\x02\x99W`\0\x80\xFD[\x80c:\x98\xEF9\x14a\x028W\x80cC\xFE\x08\xB0\x14a\x02OW\x80cG\xE7\xEF$\x14a\x02XW`\0\x80\xFD[\x80c\x01\x9E')\x14a\x01\x93W\x80c\x10\xD6z/\x14a\x01\xA8W\x80c\x11\xC7\x0C\x9D\x14a\x01\xBBW\x80c\x13d9\xDD\x14a\x01\xCEW\x80c$\x95\xA5\x99\x14a\x01\xE1W\x80c9\xB7\x0E8\x14a\x02\x11W[`\0\x80\xFD[a\x01\xA6a\x01\xA16`\x04a\x17\xB2V[a\x03\xBAV[\0[a\x01\xA6a\x01\xB66`\x04a\x17\xFCV[a\x04\x9DV[a\x01\xA6a\x01\xC96`\x04a\x18\x19V[a\x05PV[a\x01\xA6a\x01\xDC6`\x04a\x18;V[a\x06\x05V[`2Ta\x01\xF4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02A`3T\x81V[`@Q\x90\x81R` \x01a\x02\x08V[a\x02A`dT\x81V[a\x02Aa\x02f6`\x04a\x18TV[a\x07IV[a\x01\xA6a\x02y6`\x04a\x18\x80V[a\tiV[a\x02Aa\x02\x8C6`\x04a\x17\xFCV[a\n7V[a\x01\xA6a\nKV[a\x02\xB8a\x02\xA76`\x04a\x18\xB9V[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x08V[`\x01Ta\x02AV[a\x02A`eT\x81V[a\x02Aa\x02\xE76`\x04a\x18;V[a\x0B\x17V[`\0Ta\x01\xF4\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02Aa\x03\x136`\x04a\x18;V[a\x0BbV[a\x02Aa\x03&6`\x04a\x17\xFCV[a\x0BmV[a\x033a\x0B{V[`@Qa\x02\x08\x91\x90a\x19\x0CV[a\x02Aa\x03N6`\x04a\x17\xFCV[a\x0B\x9BV[a\x01\xA6a\x03a6`\x04a\x19?V[a\x0C0V[`dT`eT`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x08V[a\x02Aa\x03\x8F6`\x04a\x18;V[a\x0E}V[a\x02Aa\x03\xA26`\x04a\x18;V[a\x0E\xB6V[a\x01\xA6a\x03\xB56`\x04a\x18;V[a\x0E\xC1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x03\xDAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x03\xF4WP0;\x15\x80\x15a\x03\xF4WP`\0T`\xFF\x16`\x01\x14[a\x04\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\x80V[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x04<W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x04F\x85\x85a\x10\x1DV[a\x04P\x83\x83a\x11*V[\x80\x15a\x04\x96W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x14\x91\x90a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\xEBV[a\x05M\x81a\x11\xBBV[PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\xEBV[a\x06\x01\x82\x82a\x10\x1DV[PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06v\x91\x90a\x1A5V[a\x06\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x1AWV[`\x01T\x81\x81\x16\x14a\x07\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x01\x80T`\0\x91\x82\x91\x81\x16\x14\x15a\x07\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x04\x10V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrategyBase.onlyStrategyManager`D\x82\x01R`d\x01a\x04\x10V[a\x08 \x84\x84a\x12\xC0V[`2T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14a\x08\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FStrategyBase.deposit: Can only d`D\x82\x01Ru2\xB87\xB9\xB4\xBA\x10:\xB722\xB96<\xB4\xB73\xAA7\xB5\xB2\xB7`Q\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`3T`\0a\x08\xADa\x03\xE8\x83a\x1A\xB5V[\x90P`\0a\x03\xE8a\x08\xBCa\x13\x98V[a\x08\xC6\x91\x90a\x1A\xB5V[\x90P`\0a\x08\xD4\x87\x83a\x1A\xCDV[\x90P\x80a\x08\xE1\x84\x89a\x1A\xE4V[a\x08\xEB\x91\x90a\x1B\x03V[\x95P\x85a\tQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FStrategyBase.deposit: newShares `D\x82\x01Rmcannot be zero`\x90\x1B`d\x82\x01R`\x84\x01a\x04\x10V[a\t[\x86\x85a\x1A\xB5V[`3UPPPPP\x92\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\x89WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xA3WP0;\x15\x80\x15a\t\xA3WP`\0T`\xFF\x16`\x01\x14[a\t\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\x80V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\t\xE2W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\t\xEC\x83\x83a\x11*V[\x80\x15a\n2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0a\nEa\x02\xE7\x83a\x0B\x9BV[\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBC\x91\x90a\x1A5V[a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x1AWV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x80a\x03\xE8`3Ta\x0B*\x91\x90a\x1A\xB5V[\x90P`\0a\x03\xE8a\x0B9a\x13\x98V[a\x0BC\x91\x90a\x1A\xB5V[\x90P\x81a\x0BP\x85\x83a\x1A\xE4V[a\x0BZ\x91\x90a\x1B\x03V[\x94\x93PPPPV[`\0a\nE\x82a\x0E}V[`\0a\nEa\x03\xA2\x83a\x0B\x9BV[```@Q\x80`\x80\x01`@R\x80`M\x81R` \x01a\x1B[`M\x919\x90P\x90V[`@Qc=?\x06\xC9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R0`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cz~\r\x92\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nE\x91\x90a\x1B%V[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x0C\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x04\x10V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrategyBase.onlyStrategyManager`D\x82\x01R`d\x01a\x04\x10V[`2T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\r~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FStrategyBase.withdraw: Can only `D\x82\x01R\x7Fwithdraw the strategy token\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[`3T\x80\x83\x11\x15a\x0E\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FStrategyBase.withdraw: amountSha`D\x82\x01R\x7Fres must be less than or equal t`d\x82\x01Rlo totalShares`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`\0a\x0E\x1Ba\x03\xE8\x83a\x1A\xB5V[\x90P`\0a\x03\xE8a\x0E*a\x13\x98V[a\x0E4\x91\x90a\x1A\xB5V[\x90P`\0\x82a\x0EC\x87\x84a\x1A\xE4V[a\x0EM\x91\x90a\x1B\x03V[\x90Pa\x0EY\x86\x85a\x1A\xCDV[`3U`2Ta\x0Es\x90`\x01`\x01`\xA0\x1B\x03\x16\x89\x83a\x14\nV[PPPPPPPPV[`\0\x80a\x03\xE8`3Ta\x0E\x90\x91\x90a\x1A\xB5V[\x90P`\0a\x03\xE8a\x0E\x9Fa\x13\x98V[a\x0E\xA9\x91\x90a\x1A\xB5V[\x90P\x80a\x0BP\x83\x86a\x1A\xE4V[`\0a\nE\x82a\x0B\x17V[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F8\x91\x90a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FhW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x90a\x19\xEBV[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a\x0F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07>V[`dT`@\x80Q\x91\x82R` \x82\x01\x84\x90R\x7F\xF9~\xD4\xE0\x83\xAC\xACg\x83\0%\xEC\xBCum\x8F\xE8G\xCD\xBD\xCAL\xEE?\xE1\xE1(\xE9\x8BT\xEC\xB5\x91\x01`@Q\x80\x91\x03\x90\xA1`eT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7Fj\xB1\x81\xE0D\x0B\xFB\xF4\xBA\xCD\xF2\xE9\x96ts\\\xE6c\x80\x05I\x06\x88\xC5\xF9\x94\xF59\x93S\xE4R\x91\x01`@Q\x80\x91\x03\x90\xA1\x80\x82\x11\x15a\x11\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FStrategyBaseTVLLimits._setTVLLim`D\x82\x01R\x7Fits: maxPerDeposit exceeds maxTo`d\x82\x01RjtalDeposits`\xA8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`d\x91\x90\x91U`eUV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x11\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`2\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ua\x06\x01\x81`\0a\x14\\V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`dT\x81\x11\x15a\x13*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FStrategyBaseTVLLimits: max per d`D\x82\x01Rn\x19\\\x1B\xDC\xDA]\x08\x19^\x18\xD9YY\x19Y`\x8A\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`eTa\x135a\x13\x98V[\x11\x15a\x06\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FStrategyBaseTVLLimits: max depos`D\x82\x01Rk\x1A]\x1C\xC8\x19^\x18\xD9YY\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`2T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x05\x91\x90a\x1B%V[\x90P\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\n2\x90\x84\x90a\x15HV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x14\x83WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\x15\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x10V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x06\x01\x82a\x11\xBBV[`\0a\x15\x9D\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x16\x1A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\n2W\x80\x80` \x01\x90Q\x81\x01\x90a\x15\xBB\x91\x90a\x1A5V[a\n2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x04\x10V[``a\x16)\x84\x84`\0\x85a\x163V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x16\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x04\x10V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x16\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x04\x10V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x17\x07\x91\x90a\x1B>V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x17DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x17IV[``\x91P[P\x91P\x91Pa\x17Y\x82\x82\x86a\x17dV[\x97\x96PPPPPPPV[``\x83\x15a\x17sWP\x81a\x16,V[\x82Q\x15a\x17\x83W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x10\x91\x90a\x19\x0CV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05MW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x17\xC8W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015a\x17\xE1\x81a\x17\x9DV[\x91P``\x85\x015a\x17\xF1\x81a\x17\x9DV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x18\x0EW`\0\x80\xFD[\x815a\x16,\x81a\x17\x9DV[`\0\x80`@\x83\x85\x03\x12\x15a\x18,W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x18MW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x18gW`\0\x80\xFD[\x825a\x18r\x81a\x17\x9DV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x93W`\0\x80\xFD[\x825a\x18\x9E\x81a\x17\x9DV[\x91P` \x83\x015a\x18\xAE\x81a\x17\x9DV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xCBW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x16,W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x18\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18\xDFV[\x83\x81\x11\x15a\x19\x06W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x19+\x81`@\x85\x01` \x87\x01a\x18\xDCV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19TW`\0\x80\xFD[\x835a\x19_\x81a\x17\x9DV[\x92P` \x84\x015a\x19o\x81a\x17\x9DV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x80\x82R`.\x90\x82\x01R\x7FInitializable: contract is alrea`@\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x19\xE0W`\0\x80\xFD[\x81Qa\x16,\x81a\x17\x9DV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1AGW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x16,W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1A\xC8Wa\x1A\xC8a\x1A\x9FV[P\x01\x90V[`\0\x82\x82\x10\x15a\x1A\xDFWa\x1A\xDFa\x1A\x9FV[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1A\xFEWa\x1A\xFEa\x1A\x9FV[P\x02\x90V[`\0\x82a\x1B WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x1B7W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82Qa\x1BP\x81\x84` \x87\x01a\x18\xDCV[\x91\x90\x91\x01\x92\x91PPV\xFEBase Strategy implementation to inherit from for more complex implementations\xA2dipfsX\"\x12 \x9A4'\xEBF1\xC4`\x16\xAC\xB2tD]e\xD3\xCAI\x9B\xBE(\xB9w\xBD\x85\x17$\x0B\x02\xDA\x15KdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STRATEGYBASETVLLIMITS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StrategyBaseTVLLimits<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StrategyBaseTVLLimits<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StrategyBaseTVLLimits<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StrategyBaseTVLLimits<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StrategyBaseTVLLimits<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StrategyBaseTVLLimits))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StrategyBaseTVLLimits<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STRATEGYBASETVLLIMITS_ABI.clone(),
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
                STRATEGYBASETVLLIMITS_ABI.clone(),
                STRATEGYBASETVLLIMITS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deposit` (0x47e7ef24) function
        pub fn deposit(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 231, 239, 36], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `explanation` (0xab5921e1) function
        pub fn explanation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([171, 89, 33, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTVLLimits` (0xdf6fadc1) function
        pub fn get_tvl_limits(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([223, 111, 173, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x019e2729) function
        pub fn initialize_with_max_per_deposit_and_max_total_deposits(
            &self,
            max_per_deposit: ::ethers::core::types::U256,
            max_total_deposits: ::ethers::core::types::U256,
            underlying_token: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 158, 39, 41],
                    (
                        max_per_deposit,
                        max_total_deposits,
                        underlying_token,
                        pauser_registry,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            underlying_token: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (underlying_token, pauser_registry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxPerDeposit` (0x43fe08b0) function
        pub fn max_per_deposit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([67, 254, 8, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxTotalDeposits` (0x61b01b5d) function
        pub fn max_total_deposits(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 176, 27, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x136439dd) function
        pub fn pause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 100, 57, 221], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseAll` (0x595c6a67) function
        pub fn pause_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 92, 106, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5ac86ab7) function
        pub fn paused_with_index(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 200, 106, 183], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauserRegistry` (0x886f1195) function
        pub fn pauser_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauserRegistry` (0x10d67a2f) function
        pub fn set_pauser_registry(
            &self,
            new_pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 122, 47], new_pauser_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTVLLimits` (0x11c70c9d) function
        pub fn set_tvl_limits(
            &self,
            new_max_per_deposit: ::ethers::core::types::U256,
            new_max_total_deposits: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [17, 199, 12, 157],
                    (new_max_per_deposit, new_max_total_deposits),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shares` (0xce7c2ac2) function
        pub fn shares(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 124, 42, 194], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharesToUnderlying` (0xf3e73875) function
        pub fn shares_to_underlying(
            &self,
            amount_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 231, 56, 117], amount_shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharesToUnderlyingView` (0x7a8b2637) function
        pub fn shares_to_underlying_view(
            &self,
            amount_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 139, 38, 55], amount_shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyManager` (0x39b70e38) function
        pub fn strategy_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 183, 14, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalShares` (0x3a98ef39) function
        pub fn total_shares(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 152, 239, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToShares` (0x8c871019) function
        pub fn underlying_to_shares(
            &self,
            amount_underlying: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 135, 16, 25], amount_underlying)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToSharesView` (0xe3dae51c) function
        pub fn underlying_to_shares_view(
            &self,
            amount_underlying: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 218, 229, 28], amount_underlying)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToken` (0x2495a599) function
        pub fn underlying_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userUnderlying` (0x8f6a6240) function
        pub fn user_underlying(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 106, 98, 64], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userUnderlyingView` (0x553ca5f8) function
        pub fn user_underlying_view(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 60, 165, 248], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xd9caed12) function
        pub fn withdraw(
            &self,
            recipient: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 202, 237, 18], (recipient, token, amount_shares))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxPerDepositUpdated` event
        pub fn max_per_deposit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxPerDepositUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxTotalDepositsUpdated` event
        pub fn max_total_deposits_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxTotalDepositsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PauserRegistrySet` event
        pub fn pauser_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRegistrySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyBaseTVLLimitsEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StrategyBaseTVLLimits<M> {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        name = "MaxPerDepositUpdated",
        abi = "MaxPerDepositUpdated(uint256,uint256)"
    )]
    pub struct MaxPerDepositUpdatedFilter {
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
        name = "MaxTotalDepositsUpdated",
        abi = "MaxTotalDepositsUpdated(uint256,uint256)"
    )]
    pub struct MaxTotalDepositsUpdatedFilter {
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
    #[ethevent(name = "Paused", abi = "Paused(address,uint256)")]
    pub struct PausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
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
    #[ethevent(name = "PauserRegistrySet", abi = "PauserRegistrySet(address,address)")]
    pub struct PauserRegistrySetFilter {
        pub pauser_registry: ::ethers::core::types::Address,
        pub new_pauser_registry: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address,uint256)")]
    pub struct UnpausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StrategyBaseTVLLimitsEvents {
        InitializedFilter(InitializedFilter),
        MaxPerDepositUpdatedFilter(MaxPerDepositUpdatedFilter),
        MaxTotalDepositsUpdatedFilter(MaxTotalDepositsUpdatedFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for StrategyBaseTVLLimitsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StrategyBaseTVLLimitsEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MaxPerDepositUpdatedFilter::decode_log(log) {
                return Ok(
                    StrategyBaseTVLLimitsEvents::MaxPerDepositUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MaxTotalDepositsUpdatedFilter::decode_log(log) {
                return Ok(
                    StrategyBaseTVLLimitsEvents::MaxTotalDepositsUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(StrategyBaseTVLLimitsEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(StrategyBaseTVLLimitsEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(StrategyBaseTVLLimitsEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StrategyBaseTVLLimitsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPerDepositUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxTotalDepositsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for StrategyBaseTVLLimitsEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MaxPerDepositUpdatedFilter>
    for StrategyBaseTVLLimitsEvents {
        fn from(value: MaxPerDepositUpdatedFilter) -> Self {
            Self::MaxPerDepositUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxTotalDepositsUpdatedFilter>
    for StrategyBaseTVLLimitsEvents {
        fn from(value: MaxTotalDepositsUpdatedFilter) -> Self {
            Self::MaxTotalDepositsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for StrategyBaseTVLLimitsEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for StrategyBaseTVLLimitsEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for StrategyBaseTVLLimitsEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,uint256)` and selector `0x47e7ef24`
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
    #[ethcall(name = "deposit", abi = "deposit(address,uint256)")]
    pub struct DepositCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `explanation` function with signature `explanation()` and selector `0xab5921e1`
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
    #[ethcall(name = "explanation", abi = "explanation()")]
    pub struct ExplanationCall;
    ///Container type for all input parameters for the `getTVLLimits` function with signature `getTVLLimits()` and selector `0xdf6fadc1`
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
    #[ethcall(name = "getTVLLimits", abi = "getTVLLimits()")]
    pub struct GetTVLLimitsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256,address,address)` and selector `0x019e2729`
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
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256,address,address)")]
    pub struct InitializeWithMaxPerDepositAndMaxTotalDepositsCall {
        pub max_per_deposit: ::ethers::core::types::U256,
        pub max_total_deposits: ::ethers::core::types::U256,
        pub underlying_token: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub underlying_token: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxPerDeposit` function with signature `maxPerDeposit()` and selector `0x43fe08b0`
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
    #[ethcall(name = "maxPerDeposit", abi = "maxPerDeposit()")]
    pub struct MaxPerDepositCall;
    ///Container type for all input parameters for the `maxTotalDeposits` function with signature `maxTotalDeposits()` and selector `0x61b01b5d`
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
    #[ethcall(name = "maxTotalDeposits", abi = "maxTotalDeposits()")]
    pub struct MaxTotalDepositsCall;
    ///Container type for all input parameters for the `pause` function with signature `pause(uint256)` and selector `0x136439dd`
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
    #[ethcall(name = "pause", abi = "pause(uint256)")]
    pub struct PauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pauseAll` function with signature `pauseAll()` and selector `0x595c6a67`
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
    #[ethcall(name = "pauseAll", abi = "pauseAll()")]
    pub struct PauseAllCall;
    ///Container type for all input parameters for the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
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
    #[ethcall(name = "paused", abi = "paused(uint8)")]
    pub struct PausedWithIndexCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
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
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `setPauserRegistry` function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`
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
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTVLLimits` function with signature `setTVLLimits(uint256,uint256)` and selector `0x11c70c9d`
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
    #[ethcall(name = "setTVLLimits", abi = "setTVLLimits(uint256,uint256)")]
    pub struct SetTVLLimitsCall {
        pub new_max_per_deposit: ::ethers::core::types::U256,
        pub new_max_total_deposits: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `shares` function with signature `shares(address)` and selector `0xce7c2ac2`
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
    #[ethcall(name = "shares", abi = "shares(address)")]
    pub struct SharesCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sharesToUnderlying` function with signature `sharesToUnderlying(uint256)` and selector `0xf3e73875`
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
    #[ethcall(name = "sharesToUnderlying", abi = "sharesToUnderlying(uint256)")]
    pub struct SharesToUnderlyingCall {
        pub amount_shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sharesToUnderlyingView` function with signature `sharesToUnderlyingView(uint256)` and selector `0x7a8b2637`
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
    #[ethcall(name = "sharesToUnderlyingView", abi = "sharesToUnderlyingView(uint256)")]
    pub struct SharesToUnderlyingViewCall {
        pub amount_shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
    #[ethcall(name = "strategyManager", abi = "strategyManager()")]
    pub struct StrategyManagerCall;
    ///Container type for all input parameters for the `totalShares` function with signature `totalShares()` and selector `0x3a98ef39`
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
    #[ethcall(name = "totalShares", abi = "totalShares()")]
    pub struct TotalSharesCall;
    ///Container type for all input parameters for the `underlyingToShares` function with signature `underlyingToShares(uint256)` and selector `0x8c871019`
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
    #[ethcall(name = "underlyingToShares", abi = "underlyingToShares(uint256)")]
    pub struct UnderlyingToSharesCall {
        pub amount_underlying: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `underlyingToSharesView` function with signature `underlyingToSharesView(uint256)` and selector `0xe3dae51c`
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
    #[ethcall(name = "underlyingToSharesView", abi = "underlyingToSharesView(uint256)")]
    pub struct UnderlyingToSharesViewCall {
        pub amount_underlying: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `underlyingToken` function with signature `underlyingToken()` and selector `0x2495a599`
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
    #[ethcall(name = "underlyingToken", abi = "underlyingToken()")]
    pub struct UnderlyingTokenCall;
    ///Container type for all input parameters for the `unpause` function with signature `unpause(uint256)` and selector `0xfabc1cbc`
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
    #[ethcall(name = "unpause", abi = "unpause(uint256)")]
    pub struct UnpauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `userUnderlying` function with signature `userUnderlying(address)` and selector `0x8f6a6240`
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
    #[ethcall(name = "userUnderlying", abi = "userUnderlying(address)")]
    pub struct UserUnderlyingCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userUnderlyingView` function with signature `userUnderlyingView(address)` and selector `0x553ca5f8`
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
    #[ethcall(name = "userUnderlyingView", abi = "userUnderlyingView(address)")]
    pub struct UserUnderlyingViewCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,address,uint256)` and selector `0xd9caed12`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address,address,uint256)")]
    pub struct WithdrawCall {
        pub recipient: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount_shares: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StrategyBaseTVLLimitsCalls {
        Deposit(DepositCall),
        Explanation(ExplanationCall),
        GetTVLLimits(GetTVLLimitsCall),
        InitializeWithMaxPerDepositAndMaxTotalDeposits(
            InitializeWithMaxPerDepositAndMaxTotalDepositsCall,
        ),
        Initialize(InitializeCall),
        MaxPerDeposit(MaxPerDepositCall),
        MaxTotalDeposits(MaxTotalDepositsCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetTVLLimits(SetTVLLimitsCall),
        Shares(SharesCall),
        SharesToUnderlying(SharesToUnderlyingCall),
        SharesToUnderlyingView(SharesToUnderlyingViewCall),
        StrategyManager(StrategyManagerCall),
        TotalShares(TotalSharesCall),
        UnderlyingToShares(UnderlyingToSharesCall),
        UnderlyingToSharesView(UnderlyingToSharesViewCall),
        UnderlyingToken(UnderlyingTokenCall),
        Unpause(UnpauseCall),
        UserUnderlying(UserUnderlyingCall),
        UserUnderlyingView(UserUnderlyingViewCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for StrategyBaseTVLLimitsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <ExplanationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Explanation(decoded));
            }
            if let Ok(decoded) = <GetTVLLimitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTVLLimits(decoded));
            }
            if let Ok(decoded) = <InitializeWithMaxPerDepositAndMaxTotalDepositsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeWithMaxPerDepositAndMaxTotalDeposits(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MaxPerDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPerDeposit(decoded));
            }
            if let Ok(decoded) = <MaxTotalDepositsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxTotalDeposits(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) = <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetTVLLimitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTVLLimits(decoded));
            }
            if let Ok(decoded) = <SharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Shares(decoded));
            }
            if let Ok(decoded) = <SharesToUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SharesToUnderlying(decoded));
            }
            if let Ok(decoded) = <SharesToUnderlyingViewCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SharesToUnderlyingView(decoded));
            }
            if let Ok(decoded) = <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyManager(decoded));
            }
            if let Ok(decoded) = <TotalSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalShares(decoded));
            }
            if let Ok(decoded) = <UnderlyingToSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyingToShares(decoded));
            }
            if let Ok(decoded) = <UnderlyingToSharesViewCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyingToSharesView(decoded));
            }
            if let Ok(decoded) = <UnderlyingTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyingToken(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UserUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserUnderlying(decoded));
            }
            if let Ok(decoded) = <UserUnderlyingViewCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserUnderlyingView(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StrategyBaseTVLLimitsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Explanation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTVLLimits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeWithMaxPerDepositAndMaxTotalDeposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxPerDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxTotalDeposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PausedWithIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTVLLimits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Shares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SharesToUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SharesToUnderlyingView(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingToSharesView(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UserUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserUnderlyingView(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StrategyBaseTVLLimitsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Explanation(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTVLLimits(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeWithMaxPerDepositAndMaxTotalDeposits(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPerDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxTotalDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTVLLimits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Shares(element) => ::core::fmt::Display::fmt(element, f),
                Self::SharesToUnderlying(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SharesToUnderlyingView(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingToShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnderlyingToSharesView(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnderlyingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserUnderlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserUnderlyingView(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExplanationCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: ExplanationCall) -> Self {
            Self::Explanation(value)
        }
    }
    impl ::core::convert::From<GetTVLLimitsCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: GetTVLLimitsCall) -> Self {
            Self::GetTVLLimits(value)
        }
    }
    impl ::core::convert::From<InitializeWithMaxPerDepositAndMaxTotalDepositsCall>
    for StrategyBaseTVLLimitsCalls {
        fn from(value: InitializeWithMaxPerDepositAndMaxTotalDepositsCall) -> Self {
            Self::InitializeWithMaxPerDepositAndMaxTotalDeposits(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MaxPerDepositCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: MaxPerDepositCall) -> Self {
            Self::MaxPerDeposit(value)
        }
    }
    impl ::core::convert::From<MaxTotalDepositsCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: MaxTotalDepositsCall) -> Self {
            Self::MaxTotalDeposits(value)
        }
    }
    impl ::core::convert::From<PauseCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetTVLLimitsCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: SetTVLLimitsCall) -> Self {
            Self::SetTVLLimits(value)
        }
    }
    impl ::core::convert::From<SharesCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: SharesCall) -> Self {
            Self::Shares(value)
        }
    }
    impl ::core::convert::From<SharesToUnderlyingCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: SharesToUnderlyingCall) -> Self {
            Self::SharesToUnderlying(value)
        }
    }
    impl ::core::convert::From<SharesToUnderlyingViewCall>
    for StrategyBaseTVLLimitsCalls {
        fn from(value: SharesToUnderlyingViewCall) -> Self {
            Self::SharesToUnderlyingView(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<TotalSharesCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: TotalSharesCall) -> Self {
            Self::TotalShares(value)
        }
    }
    impl ::core::convert::From<UnderlyingToSharesCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: UnderlyingToSharesCall) -> Self {
            Self::UnderlyingToShares(value)
        }
    }
    impl ::core::convert::From<UnderlyingToSharesViewCall>
    for StrategyBaseTVLLimitsCalls {
        fn from(value: UnderlyingToSharesViewCall) -> Self {
            Self::UnderlyingToSharesView(value)
        }
    }
    impl ::core::convert::From<UnderlyingTokenCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: UnderlyingTokenCall) -> Self {
            Self::UnderlyingToken(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UserUnderlyingCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: UserUnderlyingCall) -> Self {
            Self::UserUnderlying(value)
        }
    }
    impl ::core::convert::From<UserUnderlyingViewCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: UserUnderlyingViewCall) -> Self {
            Self::UserUnderlyingView(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for StrategyBaseTVLLimitsCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `deposit` function with signature `deposit(address,uint256)` and selector `0x47e7ef24`
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
    pub struct DepositReturn {
        pub new_shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `explanation` function with signature `explanation()` and selector `0xab5921e1`
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
    pub struct ExplanationReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getTVLLimits` function with signature `getTVLLimits()` and selector `0xdf6fadc1`
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
    pub struct GetTVLLimitsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `maxPerDeposit` function with signature `maxPerDeposit()` and selector `0x43fe08b0`
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
    pub struct MaxPerDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxTotalDeposits` function with signature `maxTotalDeposits()` and selector `0x61b01b5d`
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
    pub struct MaxTotalDepositsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
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
    pub struct PausedWithIndexReturn(pub bool);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
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
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `shares` function with signature `shares(address)` and selector `0xce7c2ac2`
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
    pub struct SharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sharesToUnderlying` function with signature `sharesToUnderlying(uint256)` and selector `0xf3e73875`
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
    pub struct SharesToUnderlyingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sharesToUnderlyingView` function with signature `sharesToUnderlyingView(uint256)` and selector `0x7a8b2637`
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
    pub struct SharesToUnderlyingViewReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
    pub struct StrategyManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalShares` function with signature `totalShares()` and selector `0x3a98ef39`
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
    pub struct TotalSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToShares` function with signature `underlyingToShares(uint256)` and selector `0x8c871019`
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
    pub struct UnderlyingToSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToSharesView` function with signature `underlyingToSharesView(uint256)` and selector `0xe3dae51c`
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
    pub struct UnderlyingToSharesViewReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToken` function with signature `underlyingToken()` and selector `0x2495a599`
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
    pub struct UnderlyingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `userUnderlying` function with signature `userUnderlying(address)` and selector `0x8f6a6240`
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
    pub struct UserUnderlyingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userUnderlyingView` function with signature `userUnderlyingView(address)` and selector `0x553ca5f8`
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
    pub struct UserUnderlyingViewReturn(pub ::ethers::core::types::U256);
}
