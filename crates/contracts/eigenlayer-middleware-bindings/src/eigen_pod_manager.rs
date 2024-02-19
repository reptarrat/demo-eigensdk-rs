pub use eigen_pod_manager::*;
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
pub mod eigen_pod_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_ethPOS"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IETHPOSDeposit"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_eigenPodBeacon"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IBeacon"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategyManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStrategyManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_slasher"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_delegationManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IDelegationManager",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("beaconChainETHStrategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beaconChainETHStrategy",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beaconChainOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beaconChainOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IBeaconChainOracle",
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
                    ::std::borrow::ToOwned::to_owned("createPod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createPod"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegationManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegationManager"),
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
                    ::std::borrow::ToOwned::to_owned("denebForkTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("denebForkTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("eigenPodBeacon"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eigenPodBeacon"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBeacon"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ethPOS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ethPOS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IETHPOSDeposit"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBlockRootAtTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBlockRootAtTimestamp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("getPod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IEigenPod"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasPod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasPod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxPods"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_beaconChainOracle",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IBeaconChainOracle",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initPausedStatus"),
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
                    ::std::borrow::ToOwned::to_owned("maxPods"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxPods"),
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
                    ::std::borrow::ToOwned::to_owned("numPods"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numPods"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("ownerToPod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerToPod"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IEigenPod"),
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
                    ::std::borrow::ToOwned::to_owned("podOwnerShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("podOwnerShares"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                        "recordBeaconChainETHBalanceUpdate",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recordBeaconChainETHBalanceUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sharesDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("removeShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDenebForkTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDenebForkTimestamp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newDenebForkTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("setMaxPods"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxPods"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxPods"),
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
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slasher"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("updateBeaconChainOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateBeaconChainOracle",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newBeaconChainOracle",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IBeaconChainOracle",
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
                    ::std::borrow::ToOwned::to_owned("withdrawSharesAsTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawSharesAsTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BeaconChainETHDeposited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BeaconChainETHDeposited",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "BeaconChainETHWithdrawalCompleted",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BeaconChainETHWithdrawalCompleted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegatedAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
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
                    ::std::borrow::ToOwned::to_owned("BeaconOracleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BeaconOracleUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOracleAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DenebForkTimestampUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DenebForkTimestampUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("MaxPodsUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MaxPodsUpdated"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("PodDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PodDeployed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eigenPod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
                    ::std::borrow::ToOwned::to_owned("PodSharesUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PodSharesUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sharesDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
    pub static EIGENPODMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\x006\xC68\x03\x80b\x006\xC6\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01KV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x80R\x80\x85\x16`\xA0R\x80\x84\x16`\xC0R\x80\x83\x16`\xE0R\x81\x16a\x01\0Rb\0\0eb\0\0pV[PPPPPb\0\x01\xCBV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x010W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01HW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01dW`\0\x80\xFD[\x85Qb\0\x01q\x81b\0\x012V[` \x87\x01Q\x90\x95Pb\0\x01\x84\x81b\0\x012V[`@\x87\x01Q\x90\x94Pb\0\x01\x97\x81b\0\x012V[``\x87\x01Q\x90\x93Pb\0\x01\xAA\x81b\0\x012V[`\x80\x87\x01Q\x90\x92Pb\0\x01\xBD\x81b\0\x012V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa4\x85b\0\x02A`\09`\0\x81\x81a\x06\x82\x01R\x81\x81a\x07\xE8\x01R\x81\x81a\x0C\n\x01R\x81\x81a\x14\xE3\x01R\x81\x81a\x19x\x01Ra\x1Ah\x01R`\0a\x05x\x01R`\0a\x03\x1C\x01R`\0\x81\x81a\x02\xB0\x01R\x81\x81a\x14b\x01Ra!\xF7\x01R`\0a\x04J\x01Ra4\x85`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01\x18W\x80c\xC0\xCC\xBF\x10\x11a\0\xA0W\x80c\xDA\xF1,\xD4\x11a\0oW\x80c\xDA\xF1,\xD4\x14a\x06PW\x80c\xEAM<\x9B\x14a\x06pW\x80c\xF2\xFD\xE3\x8B\x14a\x06\xA4W\x80c\xF6\x84\x8D$\x14a\x06\xC4W\x80c\xFA\xBC\x1C\xBC\x14a\x06\xFFW`\0\x80\xFD[\x80c\xC0\xCC\xBF\x10\x14a\x05\xDAW\x80c\xC1\xDE:\xEF\x14a\x05\xF0W\x80c\xC2\xC5\x1C@\x14a\x06\x10W\x80c\xD1\xC6L\xC9\x14a\x060W`\0\x80\xFD[\x80c\xA3\x84\x06\xA3\x11a\0\xE7W\x80c\xA3\x84\x06\xA3\x14a\x050W\x80c\xA6\xA5\t\xBE\x14a\x05PW\x80c\xB14Bq\x14a\x05fW\x80c\xBE\xFF\xBB\x89\x14a\x05\x9AW\x80c\xC0R\xBDa\x14a\x05\xBAW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04\xA1W\x80c\x91\x04\xC3\x19\x14a\x04\xBFW\x80c\x9BNF4\x14a\x04\xE7W\x80c\x9B\xA0bu\x14a\x04\xFAW`\0\x80\xFD[\x80cF=\xB08\x11a\x01\x9BW\x80c`\xF4\x06+\x11a\x01jW\x80c`\xF4\x06+\x14a\x03\xF6W\x80cqP\x18\xA6\x14a\x04#W\x80ct\xCD\xD7\x98\x14a\x048W\x80c\x84\xD8\x10b\x14a\x04lW\x80c\x88o\x11\x95\x14a\x04\x81W`\0\x80\xFD[\x80cF=\xB08\x14a\x03lW\x80cY\\jg\x14a\x03\x8CW\x80cZ\xC8j\xB7\x14a\x03\xA1W\x80c\\\x97Z\xBB\x14a\x03\xE1W`\0\x80\xFD[\x80c)+{+\x11a\x01\xD7W\x80c)+{+\x14a\x02\x9EW\x80c8{\x13\0\x14a\x02\xEAW\x80c9\xB7\x0E8\x14a\x03\nW\x80cD\xE7\x1C\x80\x14a\x03>W`\0\x80\xFD[\x80c\x0C\xF2hm\x14a\x02\tW\x80c\x0E\x81\x07<\x14a\x02+W\x80c\x10\xD6z/\x14a\x02^W\x80c\x13d9\xDD\x14a\x02~W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02)a\x02$6`\x04a%\xAEV[a\x07\x1FV[\0[4\x80\x15a\x027W`\0\x80\xFD[Pa\x02Ka\x02F6`\x04a%\xDCV[a\x07\xDBV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW`\0\x80\xFD[Pa\x02)a\x02y6`\x04a&\x08V[a\n\x10V[4\x80\x15a\x02\x8AW`\0\x80\xFD[Pa\x02)a\x02\x996`\x04a%\xAEV[a\n\xC0V[4\x80\x15a\x02\xAAW`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x02)a\x03\x056`\x04a&%V[a\x0B\xFFV[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03JW`\0\x80\xFD[Pa\x03Sa\x0F\x9DV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x03xW`\0\x80\xFD[Pa\x02)a\x03\x876`\x04a&fV[a\x0F\xC6V[4\x80\x15a\x03\x98W`\0\x80\xFD[Pa\x02)a\x11RV[4\x80\x15a\x03\xADW`\0\x80\xFD[Pa\x03\xD1a\x03\xBC6`\x04a&\x90V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02UV[4\x80\x15a\x03\xEDW`\0\x80\xFD[P`fTa\x02KV[4\x80\x15a\x04\x02W`\0\x80\xFD[Pa\x02Ka\x04\x116`\x04a&\x08V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x02)a\x12\x19V[4\x80\x15a\x04DW`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04xW`\0\x80\xFD[Pa\x02\xD2a\x12-V[4\x80\x15a\x04\x8DW`\0\x80\xFD[P`eTa\x02\xD2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xADW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xD2V[4\x80\x15a\x04\xCBW`\0\x80\xFD[Pa\x02\xD2s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x02)a\x04\xF56`\x04a&\xFCV[a\x13\x17V[4\x80\x15a\x05\x06W`\0\x80\xFD[Pa\x02\xD2a\x05\x156`\x04a&\x08V[`\x98` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05<W`\0\x80\xFD[Pa\x02\xD2a\x05K6`\x04a&\x08V[a\x14\x06V[4\x80\x15a\x05\\W`\0\x80\xFD[Pa\x02K`\x99T\x81V[4\x80\x15a\x05rW`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xA6W`\0\x80\xFD[Pa\x02)a\x05\xB56`\x04a%\xDCV[a\x14\xD8V[4\x80\x15a\x05\xC6W`\0\x80\xFD[P`\x97Ta\x02\xD2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xE6W`\0\x80\xFD[Pa\x02K`\x9AT\x81V[4\x80\x15a\x05\xFCW`\0\x80\xFD[Pa\x02)a\x06\x0B6`\x04a&\x08V[a\x16\xEFV[4\x80\x15a\x06\x1CW`\0\x80\xFD[Pa\x02)a\x06+6`\x04a%\xDCV[a\x17\0V[4\x80\x15a\x06<W`\0\x80\xFD[Pa\x02Ka\x06K6`\x04a&fV[a\x1B\x03V[4\x80\x15a\x06\\W`\0\x80\xFD[Pa\x02)a\x06k6`\x04a'pV[a\x1C\x0EV[4\x80\x15a\x06|W`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xB0W`\0\x80\xFD[Pa\x02)a\x06\xBF6`\x04a&\x08V[a\x1DAV[4\x80\x15a\x06\xD0W`\0\x80\xFD[Pa\x03\xD1a\x06\xDF6`\x04a&\x08V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x98` R`@\x90 T\x16\x15\x15\x90V[4\x80\x15a\x07\x0BW`\0\x80\xFD[Pa\x02)a\x07\x1A6`\x04a%\xAEV[a\x1D\xB7V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x96\x91\x90a'\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a'\xE8V[`@Q\x80\x91\x03\x90\xFD[a\x07\xD8\x81a\x1F\x13V[PV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a(2V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x08\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FEigenPodManager.addShares: podOw`D\x82\x01R\x7Fner cannot be zero address\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`\0\x82\x12\x15a\t\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPodManager.addShares: share`D\x82\x01Rss cannot be negative``\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[a\t\x1Dc;\x9A\xCA\0\x83a(\xA6V[\x15a\t\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEigenPodManager.addShares: share`D\x82\x01R\x7Fs must be a whole Gwei amount\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 T\x90a\t\xB4\x84\x83a(\xD0V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x9B` R`@\x90\x81\x90 \x83\x90UQ\x91\x92P\x90`\0\x80Q` a4\x10\x839\x81Q\x91R\x90a\t\xF3\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\n\x05\x82\x82a\x1FTV[\x92PPP[\x92\x91PPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\ncW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x87\x91\x90a'\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a'\xE8V[a\x07\xD8\x81a\x1F\x96V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B,\x91\x90a)\x11V[a\x0BHW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a)3V[`fT\x81\x81\x16\x14a\x0B\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0CGW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a(2V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0C\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: podOwner cannot be zero `d\x82\x01Rfaddress`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\r>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: destination cannot be ze`d\x82\x01Riro address`\xB0\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\0\x81\x12\x15a\r\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: shares cannot be negativ`d\x82\x01R`e`\xF8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[a\r\xBBc;\x9A\xCA\0\x82a(\xA6V[\x15a\x0E/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: shares must be a whole G`d\x82\x01Ri\x1D\xD9ZH\x18[[\xDD[\x9D`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 T\x90\x81\x12\x15a\x0F\"W`\0a\x0E[\x82a){V[\x90P\x80\x83\x11\x15a\x0E\xC0W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9B` R`@\x81 Ua\x0E\x88\x81\x84a)\x98V[\x92P\x84`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a4\x10\x839\x81Q\x91R\x82`@Qa\x0E\xB3\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x0F V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9B` R`@\x81 \x80T\x85\x92\x90a\x0E\xE8\x90\x84\x90a(\xD0V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\0\x80Q` a4\x10\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPV[P[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\x98` R`@\x90\x81\x90 T\x90QcbH:!`\xE1\x1B\x81R\x85\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x16\x90c\xC4\x90tB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x7FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x93W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x9CT`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x0F\xC1Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91PP\x90V[\x91\x90PV[a\x0F\xCEa \x8DV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x10`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FEigenPodManager.setDenebForkTime`D\x82\x01R\x7Fstamp: cannot set newDenebForkTi`d\x82\x01Rk\x06\xD6W7F\x16\xD7\x02\x07F\xF2\x03`\xA4\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x9CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x10\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEigenPodManager.setDenebForkTime`D\x82\x01R\x7Fstamp: cannot set denebForkTimes`d\x82\x01Rrtamp more than once`h\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x9C\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x19 \x0Bo\xDA\xD5\x8F\x91\xB2\xF4\x96\xB0\xC4D\xFCK\xE3\xEF\xF7J~$\xB0wp\xE0Jq7\xBF\xD9\xDB\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xBE\x91\x90a)\x11V[a\x11\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a)3V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x12!a \x8DV[a\x12+`\0a \xE7V[V[`fT`\0\x90\x81\x90`\x01\x90\x81\x16\x14\x15a\x12\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x07\xC6V[3`\0\x90\x81R`\x98` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x13\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FEigenPodManager.createPod: Sende`D\x82\x01Rr\x1C\x88\x18[\x1C\x99XY\x1EH\x1A\x18\\\xC8\x18H\x1C\x1B\xD9`j\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\0a\x13\x10a!9V[\x92PPP\x90V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x13lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x07\xC6V[3`\0\x90\x81R`\x98` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x13\x95Wa\x13\x92a!9V[\x90P[`@Qc&\xD3\x91\x8D`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x9BNF4\x904\x90a\x13\xCB\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a)\xD8V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x13\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xF8W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x98` R`@\x81 T\x90\x91\x16\x80a\n\nWa\x14\xD1\x83`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B`@Q\x80a\t@\x01`@R\x80a\t\x0E\x81R` \x01a+\x02a\t\x0E\x919`@\x80Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R\x80\x82\x01\x91\x90\x91R`\0``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x14\xB6\x92\x91` \x01a*MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a#\x14V[\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15 W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a(2V[`\0\x81\x12\x15a\x15\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FEigenPodManager.removeShares: sh`D\x82\x01R\x7Fares cannot be negative\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[a\x15\xA5c;\x9A\xCA\0\x82a(\xA6V[\x15a\x16\x1AW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FEigenPodManager.removeShares: sh`D\x82\x01R\x7Fares must be a whole Gwei amount`d\x82\x01R`\x84\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x16>\x90\x83\x90a*bV[\x90P`\0\x81\x12\x15a\x16\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FEigenPodManager.removeShares: ca`D\x82\x01R\x7Fnnot result in pod owner having `d\x82\x01Rnnegative shares`\x88\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\0\x90\x81R`\x9B` R`@\x90 \x91\x90\x91UPV[a\x16\xF7a \x8DV[a\x07\xD8\x81a#pV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x98` R`@\x90 T\x83\x91\x163\x14a\x17{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FEigenPodManager.onlyEigenPod: no`D\x82\x01Rf\x1D\x08\x18H\x1C\x1B\xD9`\xCA\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\x02`\xC9T\x14\x15a\x17\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\xC6V[`\x02`\xC9U`\x01`\x01`\xA0\x1B\x03\x83\x16a\x18jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FEigenPodManager.recordBeaconChai`D\x82\x01R\x7FnETHBalanceUpdate: podOwner cann`d\x82\x01Rqot be zero address`p\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[a\x18xc;\x9A\xCA\0\x83a*\xA1V[\x15a\x19\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Z`$\x82\x01R\x7FEigenPodManager.recordBeaconChai`D\x82\x01R\x7FnETHBalanceUpdate: sharesDelta m`d\x82\x01R\x7Fust be a whole Gwei amount\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 T\x90a\x195\x84\x83a(\xD0V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9B` R`@\x81 \x82\x90U\x90\x91Pa\x19]\x83\x83a\x1FTV[\x90P\x80\x15a\x1A\xC5W`\0\x81\x12\x15a\x1A(W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x13-Ig\x87s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a\x19\xBC\x85a){V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\x1FW=`\0\x80>=`\0\xFD[PPPPa\x1A\xC5V[`@Qc\x14R\xB9\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x83\x01R`D\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c(\xA5s\xAE\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xC0W=`\0\x80>=`\0\xFD[PPPP[\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a4\x10\x839\x81Q\x91R\x86`@Qa\x1A\xEE\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PP`\x01`\xC9UPPPPV[`\x97T`@Qc2\x1A\xCC\xF9`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cd5\x99\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B~\x91\x90a*\xB5V[\x90P\x80a\n\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FEigenPodManager.getBlockRootAtTi`D\x82\x01R\x7Fmestamp: state root at timestamp`d\x82\x01Rq\x08\x1B\x9B\xDD\x08\x1EY]\x08\x19\x9A[\x98[\x1A^\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1C.WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1CHWP0;\x15\x80\x15a\x1CHWP`\0T`\xFF\x16`\x01\x14[a\x1C\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1C\xCEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1C\xD7\x86a\x1F\x13V[a\x1C\xE0\x85a#pV[a\x1C\xE9\x84a \xE7V[a\x1C\xF3\x83\x83a#\xBAV[\x80\x15a\x1D9W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[a\x1DIa \x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1D\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[a\x07\xD8\x81a \xE7V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E.\x91\x90a'\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a'\xE8V[`fT\x19\x81\x19`fT\x19\x16\x14a\x1E\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0B\xF4V[`\x9AT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FNe\xC4\x1A5\x97\xBD\xA72\xCAd\x98\x025\xCFQIAq\xD5\x859\x98v?\xB0]\xB4Z\xFA\xAC\xB3\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9AUV[`\0\x80\x83\x13a\x1FtW`\0\x82\x13a\x1FmWP`\0a\n\nV[P\x80a\n\nV[`\0\x82\x13a\x1F\x8CWa\x1F\x85\x83a){V[\x90Pa\n\nV[a\x1F\x85\x83\x83a*bV[`\x01`\x01`\xA0\x1B\x03\x81\x16a $W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\xC6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0`\x9AT`\x99T`\x01a!M\x91\x90a*\xCEV[\x11\x15a!\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FEigenPodManager._deployPod: pod `D\x82\x01Rl\x1B\x1A[Z]\x08\x1C\x99XX\xDA\x19Y`\x9A\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\x99`\0\x81Ta!\xC0\x90a*\xE6V[\x90\x91UP`@\x80Qa\t@\x81\x01\x90\x91Ra\t\x0E\x80\x82R`\0\x91a\"_\x91\x83\x913\x91a+\x02` \x83\x019`@\x80Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R\x80\x82\x01\x91\x90\x91R`\0``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"K\x92\x91` \x01a*MV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra$\xA4V[`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xC4\xD6m\xE8\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"\xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\xB7W=`\0\x80>=`\0\xFD[PP3`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x90\x81\x17\x90\x91U\x90Q\x92\x94P\x92P\x7F!\xC9\x9D\r\xB0\"\x13\xC3/\xFF[\x05\xCF\nq\x8A\xB5\xF8X\x80+\x91I\x8F\x80\xD8\"p(\x9D\x85j\x91\xA3\x91\x90PV[`@\x80Q`\x01`\x01`\xF8\x1B\x03\x19` \x80\x83\x01\x91\x90\x91Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16`!\x83\x01R`5\x82\x01\x85\x90R`U\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`u\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90a\x14\xD1V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\x08\xF0G\x07T\x94l\xCF\xBBDo\xF7\xFD-j\xE6\xAF\x1B\xBD\xAE\x19\xF8W\x94\xC0\xCC^\xD5\xE8\xCE\xB4\xF6\x90`\0\x90\xA2PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a#\xDBWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a$]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a$\xA0\x82a\x1F\x96V[PPV[`\0\x80\x84G\x10\x15a$\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCreate2: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x07\xC6V[\x82Qa%EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCreate2: bytecode length is zero`D\x82\x01R`d\x01a\x07\xC6V[\x83\x83Q` \x85\x01\x87\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a%\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FCreate2: Failed on deploy\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC6V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a%\xC0W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xD8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a%\xEFW`\0\x80\xFD[\x825a%\xFA\x81a%\xC7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a&\x1AW`\0\x80\xFD[\x815a\x14\xD1\x81a%\xC7V[`\0\x80`\0``\x84\x86\x03\x12\x15a&:W`\0\x80\xFD[\x835a&E\x81a%\xC7V[\x92P` \x84\x015a&U\x81a%\xC7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a&xW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xD1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a&\xA2W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x14\xD1W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a&\xC5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xDDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a&\xF5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a'\x14W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a',W`\0\x80\xFD[a'8\x89\x83\x8A\x01a&\xB3V[\x90\x97P\x95P` \x88\x015\x91P\x80\x82\x11\x15a'QW`\0\x80\xFD[Pa'^\x88\x82\x89\x01a&\xB3V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a'\x88W`\0\x80\xFD[\x855\x94P` \x86\x015a'\x9A\x81a%\xC7V[\x93P`@\x86\x015a'\xAA\x81a%\xC7V[\x92P``\x86\x015a'\xBA\x81a%\xC7V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'\xDDW`\0\x80\xFD[\x81Qa\x14\xD1\x81a%\xC7V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`@\x90\x82\x01\x81\x90R\x7FEigenPodManager.onlyDelegationMa\x90\x82\x01R\x7Fnager: not the DelegationManager``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a(\xB5Wa(\xB5a(\x90V[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15a(\xF2Wa(\xF2a(\xBAV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a)\x0BWa)\x0Ba(\xBAV[PP\x01\x90V[`\0` \x82\x84\x03\x12\x15a)#W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x14\xD1W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01`\xFF\x1B\x82\x14\x15a)\x91Wa)\x91a(\xBAV[P`\0\x03\x90V[`\0\x82\x82\x10\x15a)\xAAWa)\xAAa(\xBAV[P\x03\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R`\0a)\xEC``\x83\x01\x87\x89a)\xAFV[\x82\x81\x03` \x84\x01Ra)\xFF\x81\x86\x88a)\xAFV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0\x81Q`\0[\x81\x81\x10\x15a*3W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a*\x19V[\x81\x81\x11\x15a*BW`\0\x82\x86\x01R[P\x92\x90\x92\x01\x92\x91PPV[`\0a%\xA6a*\\\x83\x86a*\x12V[\x84a*\x12V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a*\x80Wa*\x80a(\xBAV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a*\x9BWa*\x9Ba(\xBAV[PP\x03\x90V[`\0\x82a*\xB0Wa*\xB0a(\x90V[P\x07\x90V[`\0` \x82\x84\x03\x12\x15a*\xC7W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15a*\xE1Wa*\xE1a(\xBAV[P\x01\x90V[`\0`\0\x19\x82\x14\x15a*\xFAWa*\xFAa(\xBAV[P`\x01\x01\x90V\xFE`\x80`@R`@Qa\t\x0E8\x03\x80a\t\x0E\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x04`V[a\0.\x82\x82`\0a\x005V[PPa\x05\x8AV[a\0>\x83a\x01\0V[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\x1C\xF3\xB0:l\xF1\x9F\xA2\xBA\xBAM\xF1H\xE9\xDC\xAB\xED\xEA\x7F\x8A\\\x07\x84\x0E ~\\\x08\x9B\xE9]>\x90`\0\x90\xA2`\0\x82Q\x11\x80a\0\x7FWP\x80[\x15a\0\xFBWa\0\xF9\x83`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xE9\x91\x90a\x05 V[\x83a\x02\xA3` \x1Ba\0)\x17` \x1CV[P[PPPV[a\x01\x13\x81a\x02\xCF` \x1Ba\0U\x17` \x1CV[a\x01rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC1967: new beacon is not a con`D\x82\x01Rd\x1D\x1C\x98X\xDD`\xDA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xE6\x81`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD7\x91\x90a\x05 V[a\x02\xCF` \x1Ba\0U\x17` \x1CV[a\x02KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC1967: beacon implementation i`D\x82\x01Ro\x1C\xC8\x1B\x9B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x82\x1B`d\x82\x01R`\x84\x01a\x01iV[\x80a\x02\x82\x7F\xA3\xF0\xADt\xE5B:\xEB\xFD\x80\xD3\xEFCFW\x835\xA9\xA7*\xEA\xEEY\xFFl\xB3X+5\x13=P`\0\x1Ba\x02\xDE` \x1Ba\0d\x17` \x1CV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[``a\x02\xC8\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x08\xE7`'\x919a\x02\xE1V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x02\xFE\x91\x90a\x05;V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x039W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03>V[``\x91P[P\x90\x92P\x90Pa\x03P\x86\x83\x83\x87a\x03ZV[\x96\x95PPPPPPV[``\x83\x15a\x03\xC6W\x82Qa\x03\xBFW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01iV[P\x81a\x03\xD0V[a\x03\xD0\x83\x83a\x03\xD8V[\x94\x93PPPPV[\x81Q\x15a\x03\xE8W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01i\x91\x90a\x05WV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x19W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x04OW\x81\x81\x01Q\x83\x82\x01R` \x01a\x047V[\x83\x81\x11\x15a\0\xF9WPP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x04sW`\0\x80\xFD[a\x04|\x83a\x04\x02V[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x04\x99W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\xADW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x04\xBFWa\x04\xBFa\x04\x1EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\xE7Wa\x04\xE7a\x04\x1EV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x05\0W`\0\x80\xFD[a\x05\x11\x83` \x83\x01` \x88\x01a\x044V[\x80\x95PPPPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x052W`\0\x80\xFD[a\x02\xC8\x82a\x04\x02V[`\0\x82Qa\x05M\x81\x84` \x87\x01a\x044V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x05v\x81`@\x85\x01` \x87\x01a\x044V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[a\x03N\x80a\x05\x99`\09`\0\xF3\xFE`\x80`@R6a\0\x13Wa\0\x11a\0\x17V[\0[a\0\x11[a\0'a\0\"a\0gV[a\x01\0V[V[``a\0N\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x02\xF2`'\x919a\x01$V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[`\0a\0\x9A\x7F\xA3\xF0\xADt\xE5B:\xEB\xFD\x80\xD3\xEFCFW\x835\xA9\xA7*\xEA\xEEY\xFFl\xB3X+5\x13=PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xFB\x91\x90a\x02IV[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x01\x1FW=`\0\xF3[=`\0\xFD[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x01A\x91\x90a\x02\xA2V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01|W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x81V[``\x91P[P\x91P\x91Pa\x01\x92\x86\x83\x83\x87a\x01\x9CV[\x96\x95PPPPPPV[``\x83\x15a\x02\rW\x82Qa\x02\x06W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x02\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x81a\x02\x17V[a\x02\x17\x83\x83a\x02\x1FV[\x94\x93PPPPV[\x81Q\x15a\x02/W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xFD\x91\x90a\x02\xBEV[`\0` \x82\x84\x03\x12\x15a\x02[W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0NW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x02\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x02uV[\x83\x81\x11\x15a\x02\x9CW`\0\x84\x84\x01R[PPPPV[`\0\x82Qa\x02\xB4\x81\x84` \x87\x01a\x02rV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x02\xDD\x81`@\x85\x01` \x87\x01a\x02rV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \xD5\x1E\x81\xD3\xBC^\xD2\n&\xAE\xB0]\xCE~\x82\\P; a\xAAxb\x80'0\x0C\x8De\xB9\xD8\x9AdsolcC\0\x08\x0C\x003Address: low-level delegate call failedN+y\x1D\xED\xCC\xD9\xFB0\x14\x1B\x08\x8C\xAB\xF5\xC1J\x89\x12\xB5/Y7\\\x95\xC0\x10p\x0B\x8Ca\x93EigenPodManager.withdrawSharesAs\xA2dipfsX\"\x12 IMY\x7F\xC82\xE8\xD8\xF0\xA7\xBD\xDC\x1C\xE2\xF4\t\xC0\x18Ga~p_\xFD\xF2Nj\xAE1+\x9D\xB1dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static EIGENPODMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01\x18W\x80c\xC0\xCC\xBF\x10\x11a\0\xA0W\x80c\xDA\xF1,\xD4\x11a\0oW\x80c\xDA\xF1,\xD4\x14a\x06PW\x80c\xEAM<\x9B\x14a\x06pW\x80c\xF2\xFD\xE3\x8B\x14a\x06\xA4W\x80c\xF6\x84\x8D$\x14a\x06\xC4W\x80c\xFA\xBC\x1C\xBC\x14a\x06\xFFW`\0\x80\xFD[\x80c\xC0\xCC\xBF\x10\x14a\x05\xDAW\x80c\xC1\xDE:\xEF\x14a\x05\xF0W\x80c\xC2\xC5\x1C@\x14a\x06\x10W\x80c\xD1\xC6L\xC9\x14a\x060W`\0\x80\xFD[\x80c\xA3\x84\x06\xA3\x11a\0\xE7W\x80c\xA3\x84\x06\xA3\x14a\x050W\x80c\xA6\xA5\t\xBE\x14a\x05PW\x80c\xB14Bq\x14a\x05fW\x80c\xBE\xFF\xBB\x89\x14a\x05\x9AW\x80c\xC0R\xBDa\x14a\x05\xBAW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04\xA1W\x80c\x91\x04\xC3\x19\x14a\x04\xBFW\x80c\x9BNF4\x14a\x04\xE7W\x80c\x9B\xA0bu\x14a\x04\xFAW`\0\x80\xFD[\x80cF=\xB08\x11a\x01\x9BW\x80c`\xF4\x06+\x11a\x01jW\x80c`\xF4\x06+\x14a\x03\xF6W\x80cqP\x18\xA6\x14a\x04#W\x80ct\xCD\xD7\x98\x14a\x048W\x80c\x84\xD8\x10b\x14a\x04lW\x80c\x88o\x11\x95\x14a\x04\x81W`\0\x80\xFD[\x80cF=\xB08\x14a\x03lW\x80cY\\jg\x14a\x03\x8CW\x80cZ\xC8j\xB7\x14a\x03\xA1W\x80c\\\x97Z\xBB\x14a\x03\xE1W`\0\x80\xFD[\x80c)+{+\x11a\x01\xD7W\x80c)+{+\x14a\x02\x9EW\x80c8{\x13\0\x14a\x02\xEAW\x80c9\xB7\x0E8\x14a\x03\nW\x80cD\xE7\x1C\x80\x14a\x03>W`\0\x80\xFD[\x80c\x0C\xF2hm\x14a\x02\tW\x80c\x0E\x81\x07<\x14a\x02+W\x80c\x10\xD6z/\x14a\x02^W\x80c\x13d9\xDD\x14a\x02~W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02)a\x02$6`\x04a%\xAEV[a\x07\x1FV[\0[4\x80\x15a\x027W`\0\x80\xFD[Pa\x02Ka\x02F6`\x04a%\xDCV[a\x07\xDBV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW`\0\x80\xFD[Pa\x02)a\x02y6`\x04a&\x08V[a\n\x10V[4\x80\x15a\x02\x8AW`\0\x80\xFD[Pa\x02)a\x02\x996`\x04a%\xAEV[a\n\xC0V[4\x80\x15a\x02\xAAW`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x02)a\x03\x056`\x04a&%V[a\x0B\xFFV[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03JW`\0\x80\xFD[Pa\x03Sa\x0F\x9DV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x03xW`\0\x80\xFD[Pa\x02)a\x03\x876`\x04a&fV[a\x0F\xC6V[4\x80\x15a\x03\x98W`\0\x80\xFD[Pa\x02)a\x11RV[4\x80\x15a\x03\xADW`\0\x80\xFD[Pa\x03\xD1a\x03\xBC6`\x04a&\x90V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02UV[4\x80\x15a\x03\xEDW`\0\x80\xFD[P`fTa\x02KV[4\x80\x15a\x04\x02W`\0\x80\xFD[Pa\x02Ka\x04\x116`\x04a&\x08V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x02)a\x12\x19V[4\x80\x15a\x04DW`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04xW`\0\x80\xFD[Pa\x02\xD2a\x12-V[4\x80\x15a\x04\x8DW`\0\x80\xFD[P`eTa\x02\xD2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xADW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xD2V[4\x80\x15a\x04\xCBW`\0\x80\xFD[Pa\x02\xD2s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x02)a\x04\xF56`\x04a&\xFCV[a\x13\x17V[4\x80\x15a\x05\x06W`\0\x80\xFD[Pa\x02\xD2a\x05\x156`\x04a&\x08V[`\x98` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05<W`\0\x80\xFD[Pa\x02\xD2a\x05K6`\x04a&\x08V[a\x14\x06V[4\x80\x15a\x05\\W`\0\x80\xFD[Pa\x02K`\x99T\x81V[4\x80\x15a\x05rW`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xA6W`\0\x80\xFD[Pa\x02)a\x05\xB56`\x04a%\xDCV[a\x14\xD8V[4\x80\x15a\x05\xC6W`\0\x80\xFD[P`\x97Ta\x02\xD2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xE6W`\0\x80\xFD[Pa\x02K`\x9AT\x81V[4\x80\x15a\x05\xFCW`\0\x80\xFD[Pa\x02)a\x06\x0B6`\x04a&\x08V[a\x16\xEFV[4\x80\x15a\x06\x1CW`\0\x80\xFD[Pa\x02)a\x06+6`\x04a%\xDCV[a\x17\0V[4\x80\x15a\x06<W`\0\x80\xFD[Pa\x02Ka\x06K6`\x04a&fV[a\x1B\x03V[4\x80\x15a\x06\\W`\0\x80\xFD[Pa\x02)a\x06k6`\x04a'pV[a\x1C\x0EV[4\x80\x15a\x06|W`\0\x80\xFD[Pa\x02\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xB0W`\0\x80\xFD[Pa\x02)a\x06\xBF6`\x04a&\x08V[a\x1DAV[4\x80\x15a\x06\xD0W`\0\x80\xFD[Pa\x03\xD1a\x06\xDF6`\x04a&\x08V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x98` R`@\x90 T\x16\x15\x15\x90V[4\x80\x15a\x07\x0BW`\0\x80\xFD[Pa\x02)a\x07\x1A6`\x04a%\xAEV[a\x1D\xB7V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x96\x91\x90a'\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a'\xE8V[`@Q\x80\x91\x03\x90\xFD[a\x07\xD8\x81a\x1F\x13V[PV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a(2V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x08\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FEigenPodManager.addShares: podOw`D\x82\x01R\x7Fner cannot be zero address\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`\0\x82\x12\x15a\t\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPodManager.addShares: share`D\x82\x01Rss cannot be negative``\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[a\t\x1Dc;\x9A\xCA\0\x83a(\xA6V[\x15a\t\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEigenPodManager.addShares: share`D\x82\x01R\x7Fs must be a whole Gwei amount\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 T\x90a\t\xB4\x84\x83a(\xD0V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x9B` R`@\x90\x81\x90 \x83\x90UQ\x91\x92P\x90`\0\x80Q` a4\x10\x839\x81Q\x91R\x90a\t\xF3\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\n\x05\x82\x82a\x1FTV[\x92PPP[\x92\x91PPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\ncW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x87\x91\x90a'\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a'\xE8V[a\x07\xD8\x81a\x1F\x96V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B,\x91\x90a)\x11V[a\x0BHW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a)3V[`fT\x81\x81\x16\x14a\x0B\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0CGW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a(2V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0C\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: podOwner cannot be zero `d\x82\x01Rfaddress`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\r>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: destination cannot be ze`d\x82\x01Riro address`\xB0\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\0\x81\x12\x15a\r\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: shares cannot be negativ`d\x82\x01R`e`\xF8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[a\r\xBBc;\x9A\xCA\0\x82a(\xA6V[\x15a\x0E/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` a40\x839\x81Q\x91R`D\x82\x01R\x7FTokens: shares must be a whole G`d\x82\x01Ri\x1D\xD9ZH\x18[[\xDD[\x9D`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 T\x90\x81\x12\x15a\x0F\"W`\0a\x0E[\x82a){V[\x90P\x80\x83\x11\x15a\x0E\xC0W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9B` R`@\x81 Ua\x0E\x88\x81\x84a)\x98V[\x92P\x84`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a4\x10\x839\x81Q\x91R\x82`@Qa\x0E\xB3\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x0F V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9B` R`@\x81 \x80T\x85\x92\x90a\x0E\xE8\x90\x84\x90a(\xD0V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\0\x80Q` a4\x10\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPV[P[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\x98` R`@\x90\x81\x90 T\x90QcbH:!`\xE1\x1B\x81R\x85\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x16\x90c\xC4\x90tB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x7FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x93W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x9CT`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x0F\xC1Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91PP\x90V[\x91\x90PV[a\x0F\xCEa \x8DV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x10`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FEigenPodManager.setDenebForkTime`D\x82\x01R\x7Fstamp: cannot set newDenebForkTi`d\x82\x01Rk\x06\xD6W7F\x16\xD7\x02\x07F\xF2\x03`\xA4\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x9CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x10\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEigenPodManager.setDenebForkTime`D\x82\x01R\x7Fstamp: cannot set denebForkTimes`d\x82\x01Rrtamp more than once`h\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x9C\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x19 \x0Bo\xDA\xD5\x8F\x91\xB2\xF4\x96\xB0\xC4D\xFCK\xE3\xEF\xF7J~$\xB0wp\xE0Jq7\xBF\xD9\xDB\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xBE\x91\x90a)\x11V[a\x11\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a)3V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x12!a \x8DV[a\x12+`\0a \xE7V[V[`fT`\0\x90\x81\x90`\x01\x90\x81\x16\x14\x15a\x12\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x07\xC6V[3`\0\x90\x81R`\x98` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x13\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FEigenPodManager.createPod: Sende`D\x82\x01Rr\x1C\x88\x18[\x1C\x99XY\x1EH\x1A\x18\\\xC8\x18H\x1C\x1B\xD9`j\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\0a\x13\x10a!9V[\x92PPP\x90V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x13lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x07\xC6V[3`\0\x90\x81R`\x98` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x13\x95Wa\x13\x92a!9V[\x90P[`@Qc&\xD3\x91\x8D`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x9BNF4\x904\x90a\x13\xCB\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a)\xD8V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x13\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xF8W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x98` R`@\x81 T\x90\x91\x16\x80a\n\nWa\x14\xD1\x83`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B`@Q\x80a\t@\x01`@R\x80a\t\x0E\x81R` \x01a+\x02a\t\x0E\x919`@\x80Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R\x80\x82\x01\x91\x90\x91R`\0``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x14\xB6\x92\x91` \x01a*MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a#\x14V[\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15 W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a(2V[`\0\x81\x12\x15a\x15\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FEigenPodManager.removeShares: sh`D\x82\x01R\x7Fares cannot be negative\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[a\x15\xA5c;\x9A\xCA\0\x82a(\xA6V[\x15a\x16\x1AW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FEigenPodManager.removeShares: sh`D\x82\x01R\x7Fares must be a whole Gwei amount`d\x82\x01R`\x84\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x16>\x90\x83\x90a*bV[\x90P`\0\x81\x12\x15a\x16\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FEigenPodManager.removeShares: ca`D\x82\x01R\x7Fnnot result in pod owner having `d\x82\x01Rnnegative shares`\x88\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\0\x90\x81R`\x9B` R`@\x90 \x91\x90\x91UPV[a\x16\xF7a \x8DV[a\x07\xD8\x81a#pV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x98` R`@\x90 T\x83\x91\x163\x14a\x17{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FEigenPodManager.onlyEigenPod: no`D\x82\x01Rf\x1D\x08\x18H\x1C\x1B\xD9`\xCA\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\x02`\xC9T\x14\x15a\x17\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\xC6V[`\x02`\xC9U`\x01`\x01`\xA0\x1B\x03\x83\x16a\x18jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FEigenPodManager.recordBeaconChai`D\x82\x01R\x7FnETHBalanceUpdate: podOwner cann`d\x82\x01Rqot be zero address`p\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[a\x18xc;\x9A\xCA\0\x83a*\xA1V[\x15a\x19\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Z`$\x82\x01R\x7FEigenPodManager.recordBeaconChai`D\x82\x01R\x7FnETHBalanceUpdate: sharesDelta m`d\x82\x01R\x7Fust be a whole Gwei amount\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 T\x90a\x195\x84\x83a(\xD0V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9B` R`@\x81 \x82\x90U\x90\x91Pa\x19]\x83\x83a\x1FTV[\x90P\x80\x15a\x1A\xC5W`\0\x81\x12\x15a\x1A(W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x13-Ig\x87s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a\x19\xBC\x85a){V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\x1FW=`\0\x80>=`\0\xFD[PPPPa\x1A\xC5V[`@Qc\x14R\xB9\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x83\x01R`D\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c(\xA5s\xAE\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xC0W=`\0\x80>=`\0\xFD[PPPP[\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a4\x10\x839\x81Q\x91R\x86`@Qa\x1A\xEE\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PP`\x01`\xC9UPPPPV[`\x97T`@Qc2\x1A\xCC\xF9`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cd5\x99\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B~\x91\x90a*\xB5V[\x90P\x80a\n\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FEigenPodManager.getBlockRootAtTi`D\x82\x01R\x7Fmestamp: state root at timestamp`d\x82\x01Rq\x08\x1B\x9B\xDD\x08\x1EY]\x08\x19\x9A[\x98[\x1A^\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1C.WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1CHWP0;\x15\x80\x15a\x1CHWP`\0T`\xFF\x16`\x01\x14[a\x1C\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1C\xCEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1C\xD7\x86a\x1F\x13V[a\x1C\xE0\x85a#pV[a\x1C\xE9\x84a \xE7V[a\x1C\xF3\x83\x83a#\xBAV[\x80\x15a\x1D9W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[a\x1DIa \x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1D\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[a\x07\xD8\x81a \xE7V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E.\x91\x90a'\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xC6\x90a'\xE8V[`fT\x19\x81\x19`fT\x19\x16\x14a\x1E\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0B\xF4V[`\x9AT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FNe\xC4\x1A5\x97\xBD\xA72\xCAd\x98\x025\xCFQIAq\xD5\x859\x98v?\xB0]\xB4Z\xFA\xAC\xB3\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9AUV[`\0\x80\x83\x13a\x1FtW`\0\x82\x13a\x1FmWP`\0a\n\nV[P\x80a\n\nV[`\0\x82\x13a\x1F\x8CWa\x1F\x85\x83a){V[\x90Pa\n\nV[a\x1F\x85\x83\x83a*bV[`\x01`\x01`\xA0\x1B\x03\x81\x16a $W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\xC6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0`\x9AT`\x99T`\x01a!M\x91\x90a*\xCEV[\x11\x15a!\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FEigenPodManager._deployPod: pod `D\x82\x01Rl\x1B\x1A[Z]\x08\x1C\x99XX\xDA\x19Y`\x9A\x1B`d\x82\x01R`\x84\x01a\x07\xC6V[`\x99`\0\x81Ta!\xC0\x90a*\xE6V[\x90\x91UP`@\x80Qa\t@\x81\x01\x90\x91Ra\t\x0E\x80\x82R`\0\x91a\"_\x91\x83\x913\x91a+\x02` \x83\x019`@\x80Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R\x80\x82\x01\x91\x90\x91R`\0``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"K\x92\x91` \x01a*MV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra$\xA4V[`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xC4\xD6m\xE8\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"\xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\xB7W=`\0\x80>=`\0\xFD[PP3`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x90\x81\x17\x90\x91U\x90Q\x92\x94P\x92P\x7F!\xC9\x9D\r\xB0\"\x13\xC3/\xFF[\x05\xCF\nq\x8A\xB5\xF8X\x80+\x91I\x8F\x80\xD8\"p(\x9D\x85j\x91\xA3\x91\x90PV[`@\x80Q`\x01`\x01`\xF8\x1B\x03\x19` \x80\x83\x01\x91\x90\x91Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16`!\x83\x01R`5\x82\x01\x85\x90R`U\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`u\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90a\x14\xD1V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\x08\xF0G\x07T\x94l\xCF\xBBDo\xF7\xFD-j\xE6\xAF\x1B\xBD\xAE\x19\xF8W\x94\xC0\xCC^\xD5\xE8\xCE\xB4\xF6\x90`\0\x90\xA2PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a#\xDBWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a$]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a$\xA0\x82a\x1F\x96V[PPV[`\0\x80\x84G\x10\x15a$\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCreate2: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x07\xC6V[\x82Qa%EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCreate2: bytecode length is zero`D\x82\x01R`d\x01a\x07\xC6V[\x83\x83Q` \x85\x01\x87\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a%\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FCreate2: Failed on deploy\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC6V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a%\xC0W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xD8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a%\xEFW`\0\x80\xFD[\x825a%\xFA\x81a%\xC7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a&\x1AW`\0\x80\xFD[\x815a\x14\xD1\x81a%\xC7V[`\0\x80`\0``\x84\x86\x03\x12\x15a&:W`\0\x80\xFD[\x835a&E\x81a%\xC7V[\x92P` \x84\x015a&U\x81a%\xC7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a&xW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xD1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a&\xA2W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x14\xD1W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a&\xC5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xDDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a&\xF5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a'\x14W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a',W`\0\x80\xFD[a'8\x89\x83\x8A\x01a&\xB3V[\x90\x97P\x95P` \x88\x015\x91P\x80\x82\x11\x15a'QW`\0\x80\xFD[Pa'^\x88\x82\x89\x01a&\xB3V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a'\x88W`\0\x80\xFD[\x855\x94P` \x86\x015a'\x9A\x81a%\xC7V[\x93P`@\x86\x015a'\xAA\x81a%\xC7V[\x92P``\x86\x015a'\xBA\x81a%\xC7V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'\xDDW`\0\x80\xFD[\x81Qa\x14\xD1\x81a%\xC7V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`@\x90\x82\x01\x81\x90R\x7FEigenPodManager.onlyDelegationMa\x90\x82\x01R\x7Fnager: not the DelegationManager``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a(\xB5Wa(\xB5a(\x90V[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15a(\xF2Wa(\xF2a(\xBAV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a)\x0BWa)\x0Ba(\xBAV[PP\x01\x90V[`\0` \x82\x84\x03\x12\x15a)#W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x14\xD1W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01`\xFF\x1B\x82\x14\x15a)\x91Wa)\x91a(\xBAV[P`\0\x03\x90V[`\0\x82\x82\x10\x15a)\xAAWa)\xAAa(\xBAV[P\x03\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R`\0a)\xEC``\x83\x01\x87\x89a)\xAFV[\x82\x81\x03` \x84\x01Ra)\xFF\x81\x86\x88a)\xAFV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0\x81Q`\0[\x81\x81\x10\x15a*3W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a*\x19V[\x81\x81\x11\x15a*BW`\0\x82\x86\x01R[P\x92\x90\x92\x01\x92\x91PPV[`\0a%\xA6a*\\\x83\x86a*\x12V[\x84a*\x12V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a*\x80Wa*\x80a(\xBAV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a*\x9BWa*\x9Ba(\xBAV[PP\x03\x90V[`\0\x82a*\xB0Wa*\xB0a(\x90V[P\x07\x90V[`\0` \x82\x84\x03\x12\x15a*\xC7W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15a*\xE1Wa*\xE1a(\xBAV[P\x01\x90V[`\0`\0\x19\x82\x14\x15a*\xFAWa*\xFAa(\xBAV[P`\x01\x01\x90V\xFE`\x80`@R`@Qa\t\x0E8\x03\x80a\t\x0E\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x04`V[a\0.\x82\x82`\0a\x005V[PPa\x05\x8AV[a\0>\x83a\x01\0V[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\x1C\xF3\xB0:l\xF1\x9F\xA2\xBA\xBAM\xF1H\xE9\xDC\xAB\xED\xEA\x7F\x8A\\\x07\x84\x0E ~\\\x08\x9B\xE9]>\x90`\0\x90\xA2`\0\x82Q\x11\x80a\0\x7FWP\x80[\x15a\0\xFBWa\0\xF9\x83`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xE9\x91\x90a\x05 V[\x83a\x02\xA3` \x1Ba\0)\x17` \x1CV[P[PPPV[a\x01\x13\x81a\x02\xCF` \x1Ba\0U\x17` \x1CV[a\x01rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC1967: new beacon is not a con`D\x82\x01Rd\x1D\x1C\x98X\xDD`\xDA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xE6\x81`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD7\x91\x90a\x05 V[a\x02\xCF` \x1Ba\0U\x17` \x1CV[a\x02KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC1967: beacon implementation i`D\x82\x01Ro\x1C\xC8\x1B\x9B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x82\x1B`d\x82\x01R`\x84\x01a\x01iV[\x80a\x02\x82\x7F\xA3\xF0\xADt\xE5B:\xEB\xFD\x80\xD3\xEFCFW\x835\xA9\xA7*\xEA\xEEY\xFFl\xB3X+5\x13=P`\0\x1Ba\x02\xDE` \x1Ba\0d\x17` \x1CV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[``a\x02\xC8\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x08\xE7`'\x919a\x02\xE1V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x02\xFE\x91\x90a\x05;V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x039W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03>V[``\x91P[P\x90\x92P\x90Pa\x03P\x86\x83\x83\x87a\x03ZV[\x96\x95PPPPPPV[``\x83\x15a\x03\xC6W\x82Qa\x03\xBFW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01iV[P\x81a\x03\xD0V[a\x03\xD0\x83\x83a\x03\xD8V[\x94\x93PPPPV[\x81Q\x15a\x03\xE8W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01i\x91\x90a\x05WV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x19W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x04OW\x81\x81\x01Q\x83\x82\x01R` \x01a\x047V[\x83\x81\x11\x15a\0\xF9WPP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x04sW`\0\x80\xFD[a\x04|\x83a\x04\x02V[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x04\x99W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\xADW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x04\xBFWa\x04\xBFa\x04\x1EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\xE7Wa\x04\xE7a\x04\x1EV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x05\0W`\0\x80\xFD[a\x05\x11\x83` \x83\x01` \x88\x01a\x044V[\x80\x95PPPPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x052W`\0\x80\xFD[a\x02\xC8\x82a\x04\x02V[`\0\x82Qa\x05M\x81\x84` \x87\x01a\x044V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x05v\x81`@\x85\x01` \x87\x01a\x044V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[a\x03N\x80a\x05\x99`\09`\0\xF3\xFE`\x80`@R6a\0\x13Wa\0\x11a\0\x17V[\0[a\0\x11[a\0'a\0\"a\0gV[a\x01\0V[V[``a\0N\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x02\xF2`'\x919a\x01$V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[`\0a\0\x9A\x7F\xA3\xF0\xADt\xE5B:\xEB\xFD\x80\xD3\xEFCFW\x835\xA9\xA7*\xEA\xEEY\xFFl\xB3X+5\x13=PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xFB\x91\x90a\x02IV[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x01\x1FW=`\0\xF3[=`\0\xFD[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x01A\x91\x90a\x02\xA2V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01|W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x81V[``\x91P[P\x91P\x91Pa\x01\x92\x86\x83\x83\x87a\x01\x9CV[\x96\x95PPPPPPV[``\x83\x15a\x02\rW\x82Qa\x02\x06W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x02\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x81a\x02\x17V[a\x02\x17\x83\x83a\x02\x1FV[\x94\x93PPPPV[\x81Q\x15a\x02/W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xFD\x91\x90a\x02\xBEV[`\0` \x82\x84\x03\x12\x15a\x02[W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0NW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x02\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x02uV[\x83\x81\x11\x15a\x02\x9CW`\0\x84\x84\x01R[PPPPV[`\0\x82Qa\x02\xB4\x81\x84` \x87\x01a\x02rV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x02\xDD\x81`@\x85\x01` \x87\x01a\x02rV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \xD5\x1E\x81\xD3\xBC^\xD2\n&\xAE\xB0]\xCE~\x82\\P; a\xAAxb\x80'0\x0C\x8De\xB9\xD8\x9AdsolcC\0\x08\x0C\x003Address: low-level delegate call failedN+y\x1D\xED\xCC\xD9\xFB0\x14\x1B\x08\x8C\xAB\xF5\xC1J\x89\x12\xB5/Y7\\\x95\xC0\x10p\x0B\x8Ca\x93EigenPodManager.withdrawSharesAs\xA2dipfsX\"\x12 IMY\x7F\xC82\xE8\xD8\xF0\xA7\xBD\xDC\x1C\xE2\xF4\t\xC0\x18Ga~p_\xFD\xF2Nj\xAE1+\x9D\xB1dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static EIGENPODMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct EigenPodManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EigenPodManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EigenPodManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EigenPodManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EigenPodManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EigenPodManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EigenPodManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EIGENPODMANAGER_ABI.clone(),
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
                EIGENPODMANAGER_ABI.clone(),
                EIGENPODMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addShares` (0x0e81073c) function
        pub fn add_shares(
            &self,
            pod_owner: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([14, 129, 7, 60], (pod_owner, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beaconChainETHStrategy` (0x9104c319) function
        pub fn beacon_chain_eth_strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([145, 4, 195, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beaconChainOracle` (0xc052bd61) function
        pub fn beacon_chain_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([192, 82, 189, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPod` (0x84d81062) function
        pub fn create_pod(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([132, 216, 16, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationManager` (0xea4d3c9b) function
        pub fn delegation_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 77, 60, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `denebForkTimestamp` (0x44e71c80) function
        pub fn deneb_fork_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([68, 231, 28, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eigenPodBeacon` (0x292b7b2b) function
        pub fn eigen_pod_beacon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([41, 43, 123, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ethPOS` (0x74cdd798) function
        pub fn eth_pos(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([116, 205, 215, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockRootAtTimestamp` (0xd1c64cc9) function
        pub fn get_block_root_at_timestamp(
            &self,
            timestamp: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([209, 198, 76, 201], timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPod` (0xa38406a3) function
        pub fn get_pod(
            &self,
            pod_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([163, 132, 6, 163], pod_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasPod` (0xf6848d24) function
        pub fn has_pod(
            &self,
            pod_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([246, 132, 141, 36], pod_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xdaf12cd4) function
        pub fn initialize(
            &self,
            max_pods: ::ethers::core::types::U256,
            beacon_chain_oracle: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            init_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [218, 241, 44, 212],
                    (
                        max_pods,
                        beacon_chain_oracle,
                        initial_owner,
                        pauser_registry,
                        init_paused_status,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxPods` (0xc0ccbf10) function
        pub fn max_pods(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 204, 191, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numPods` (0xa6a509be) function
        pub fn num_pods(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 165, 9, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerToPod` (0x9ba06275) function
        pub fn owner_to_pod(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([155, 160, 98, 117], p0)
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
        ///Calls the contract's `podOwnerShares` (0x60f4062b) function
        pub fn pod_owner_shares(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([96, 244, 6, 43], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordBeaconChainETHBalanceUpdate` (0xc2c51c40) function
        pub fn record_beacon_chain_eth_balance_update(
            &self,
            pod_owner: ::ethers::core::types::Address,
            shares_delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 197, 28, 64], (pod_owner, shares_delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeShares` (0xbeffbb89) function
        pub fn remove_shares(
            &self,
            pod_owner: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 255, 187, 137], (pod_owner, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDenebForkTimestamp` (0x463db038) function
        pub fn set_deneb_fork_timestamp(
            &self,
            new_deneb_fork_timestamp: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 61, 176, 56], new_deneb_fork_timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxPods` (0x0cf2686d) function
        pub fn set_max_pods(
            &self,
            new_max_pods: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 242, 104, 109], new_max_pods)
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
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 52, 66, 113], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        ///Calls the contract's `updateBeaconChainOracle` (0xc1de3aef) function
        pub fn update_beacon_chain_oracle(
            &self,
            new_beacon_chain_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 222, 58, 239], new_beacon_chain_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawSharesAsTokens` (0x387b1300) function
        pub fn withdraw_shares_as_tokens(
            &self,
            pod_owner: ::ethers::core::types::Address,
            destination: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 123, 19, 0], (pod_owner, destination, shares))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BeaconChainETHDeposited` event
        pub fn beacon_chain_eth_deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconChainETHDepositedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconChainETHWithdrawalCompleted` event
        pub fn beacon_chain_eth_withdrawal_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconChainETHWithdrawalCompletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconOracleUpdated` event
        pub fn beacon_oracle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconOracleUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DenebForkTimestampUpdated` event
        pub fn deneb_fork_timestamp_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DenebForkTimestampUpdatedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `MaxPodsUpdated` event
        pub fn max_pods_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxPodsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
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
        ///Gets the contract's `PodDeployed` event
        pub fn pod_deployed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PodDeployedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PodSharesUpdated` event
        pub fn pod_shares_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PodSharesUpdatedFilter,
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
            EigenPodManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EigenPodManager<M> {
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
        name = "BeaconChainETHDeposited",
        abi = "BeaconChainETHDeposited(address,uint256)"
    )]
    pub struct BeaconChainETHDepositedFilter {
        #[ethevent(indexed)]
        pub pod_owner: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "BeaconChainETHWithdrawalCompleted",
        abi = "BeaconChainETHWithdrawalCompleted(address,uint256,uint96,address,address,bytes32)"
    )]
    pub struct BeaconChainETHWithdrawalCompletedFilter {
        #[ethevent(indexed)]
        pub pod_owner: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
        pub nonce: u128,
        pub delegated_address: ::ethers::core::types::Address,
        pub withdrawer: ::ethers::core::types::Address,
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
    #[ethevent(name = "BeaconOracleUpdated", abi = "BeaconOracleUpdated(address)")]
    pub struct BeaconOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub new_oracle_address: ::ethers::core::types::Address,
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
        name = "DenebForkTimestampUpdated",
        abi = "DenebForkTimestampUpdated(uint64)"
    )]
    pub struct DenebForkTimestampUpdatedFilter {
        pub new_value: u64,
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
    #[ethevent(name = "MaxPodsUpdated", abi = "MaxPodsUpdated(uint256,uint256)")]
    pub struct MaxPodsUpdatedFilter {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "PodDeployed", abi = "PodDeployed(address,address)")]
    pub struct PodDeployedFilter {
        #[ethevent(indexed)]
        pub eigen_pod: ::ethers::core::types::Address,
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
    #[ethevent(name = "PodSharesUpdated", abi = "PodSharesUpdated(address,int256)")]
    pub struct PodSharesUpdatedFilter {
        #[ethevent(indexed)]
        pub pod_owner: ::ethers::core::types::Address,
        pub shares_delta: ::ethers::core::types::I256,
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
    pub enum EigenPodManagerEvents {
        BeaconChainETHDepositedFilter(BeaconChainETHDepositedFilter),
        BeaconChainETHWithdrawalCompletedFilter(BeaconChainETHWithdrawalCompletedFilter),
        BeaconOracleUpdatedFilter(BeaconOracleUpdatedFilter),
        DenebForkTimestampUpdatedFilter(DenebForkTimestampUpdatedFilter),
        InitializedFilter(InitializedFilter),
        MaxPodsUpdatedFilter(MaxPodsUpdatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        PodDeployedFilter(PodDeployedFilter),
        PodSharesUpdatedFilter(PodSharesUpdatedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for EigenPodManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BeaconChainETHDepositedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::BeaconChainETHDepositedFilter(decoded));
            }
            if let Ok(decoded) = BeaconChainETHWithdrawalCompletedFilter::decode_log(
                log,
            ) {
                return Ok(
                    EigenPodManagerEvents::BeaconChainETHWithdrawalCompletedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = BeaconOracleUpdatedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::BeaconOracleUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DenebForkTimestampUpdatedFilter::decode_log(log) {
                return Ok(
                    EigenPodManagerEvents::DenebForkTimestampUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MaxPodsUpdatedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::MaxPodsUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = PodDeployedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::PodDeployedFilter(decoded));
            }
            if let Ok(decoded) = PodSharesUpdatedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::PodSharesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(EigenPodManagerEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EigenPodManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BeaconChainETHDepositedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconChainETHWithdrawalCompletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconOracleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DenebForkTimestampUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPodsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PodDeployedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PodSharesUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BeaconChainETHDepositedFilter> for EigenPodManagerEvents {
        fn from(value: BeaconChainETHDepositedFilter) -> Self {
            Self::BeaconChainETHDepositedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconChainETHWithdrawalCompletedFilter>
    for EigenPodManagerEvents {
        fn from(value: BeaconChainETHWithdrawalCompletedFilter) -> Self {
            Self::BeaconChainETHWithdrawalCompletedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconOracleUpdatedFilter> for EigenPodManagerEvents {
        fn from(value: BeaconOracleUpdatedFilter) -> Self {
            Self::BeaconOracleUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DenebForkTimestampUpdatedFilter>
    for EigenPodManagerEvents {
        fn from(value: DenebForkTimestampUpdatedFilter) -> Self {
            Self::DenebForkTimestampUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for EigenPodManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MaxPodsUpdatedFilter> for EigenPodManagerEvents {
        fn from(value: MaxPodsUpdatedFilter) -> Self {
            Self::MaxPodsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for EigenPodManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for EigenPodManagerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for EigenPodManagerEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<PodDeployedFilter> for EigenPodManagerEvents {
        fn from(value: PodDeployedFilter) -> Self {
            Self::PodDeployedFilter(value)
        }
    }
    impl ::core::convert::From<PodSharesUpdatedFilter> for EigenPodManagerEvents {
        fn from(value: PodSharesUpdatedFilter) -> Self {
            Self::PodSharesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for EigenPodManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addShares` function with signature `addShares(address,uint256)` and selector `0x0e81073c`
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
    #[ethcall(name = "addShares", abi = "addShares(address,uint256)")]
    pub struct AddSharesCall {
        pub pod_owner: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
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
    #[ethcall(name = "beaconChainETHStrategy", abi = "beaconChainETHStrategy()")]
    pub struct BeaconChainETHStrategyCall;
    ///Container type for all input parameters for the `beaconChainOracle` function with signature `beaconChainOracle()` and selector `0xc052bd61`
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
    #[ethcall(name = "beaconChainOracle", abi = "beaconChainOracle()")]
    pub struct BeaconChainOracleCall;
    ///Container type for all input parameters for the `createPod` function with signature `createPod()` and selector `0x84d81062`
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
    #[ethcall(name = "createPod", abi = "createPod()")]
    pub struct CreatePodCall;
    ///Container type for all input parameters for the `delegationManager` function with signature `delegationManager()` and selector `0xea4d3c9b`
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
    #[ethcall(name = "delegationManager", abi = "delegationManager()")]
    pub struct DelegationManagerCall;
    ///Container type for all input parameters for the `denebForkTimestamp` function with signature `denebForkTimestamp()` and selector `0x44e71c80`
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
    #[ethcall(name = "denebForkTimestamp", abi = "denebForkTimestamp()")]
    pub struct DenebForkTimestampCall;
    ///Container type for all input parameters for the `eigenPodBeacon` function with signature `eigenPodBeacon()` and selector `0x292b7b2b`
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
    #[ethcall(name = "eigenPodBeacon", abi = "eigenPodBeacon()")]
    pub struct EigenPodBeaconCall;
    ///Container type for all input parameters for the `ethPOS` function with signature `ethPOS()` and selector `0x74cdd798`
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
    #[ethcall(name = "ethPOS", abi = "ethPOS()")]
    pub struct EthPOSCall;
    ///Container type for all input parameters for the `getBlockRootAtTimestamp` function with signature `getBlockRootAtTimestamp(uint64)` and selector `0xd1c64cc9`
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
    #[ethcall(name = "getBlockRootAtTimestamp", abi = "getBlockRootAtTimestamp(uint64)")]
    pub struct GetBlockRootAtTimestampCall {
        pub timestamp: u64,
    }
    ///Container type for all input parameters for the `getPod` function with signature `getPod(address)` and selector `0xa38406a3`
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
    #[ethcall(name = "getPod", abi = "getPod(address)")]
    pub struct GetPodCall {
        pub pod_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasPod` function with signature `hasPod(address)` and selector `0xf6848d24`
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
    #[ethcall(name = "hasPod", abi = "hasPod(address)")]
    pub struct HasPodCall {
        pub pod_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,address,address,address,uint256)` and selector `0xdaf12cd4`
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
        name = "initialize",
        abi = "initialize(uint256,address,address,address,uint256)"
    )]
    pub struct InitializeCall {
        pub max_pods: ::ethers::core::types::U256,
        pub beacon_chain_oracle: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub init_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `maxPods` function with signature `maxPods()` and selector `0xc0ccbf10`
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
    #[ethcall(name = "maxPods", abi = "maxPods()")]
    pub struct MaxPodsCall;
    ///Container type for all input parameters for the `numPods` function with signature `numPods()` and selector `0xa6a509be`
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
    #[ethcall(name = "numPods", abi = "numPods()")]
    pub struct NumPodsCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `ownerToPod` function with signature `ownerToPod(address)` and selector `0x9ba06275`
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
    #[ethcall(name = "ownerToPod", abi = "ownerToPod(address)")]
    pub struct OwnerToPodCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `podOwnerShares` function with signature `podOwnerShares(address)` and selector `0x60f4062b`
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
    #[ethcall(name = "podOwnerShares", abi = "podOwnerShares(address)")]
    pub struct PodOwnerSharesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `recordBeaconChainETHBalanceUpdate` function with signature `recordBeaconChainETHBalanceUpdate(address,int256)` and selector `0xc2c51c40`
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
        name = "recordBeaconChainETHBalanceUpdate",
        abi = "recordBeaconChainETHBalanceUpdate(address,int256)"
    )]
    pub struct RecordBeaconChainETHBalanceUpdateCall {
        pub pod_owner: ::ethers::core::types::Address,
        pub shares_delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `removeShares` function with signature `removeShares(address,uint256)` and selector `0xbeffbb89`
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
    #[ethcall(name = "removeShares", abi = "removeShares(address,uint256)")]
    pub struct RemoveSharesCall {
        pub pod_owner: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setDenebForkTimestamp` function with signature `setDenebForkTimestamp(uint64)` and selector `0x463db038`
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
    #[ethcall(name = "setDenebForkTimestamp", abi = "setDenebForkTimestamp(uint64)")]
    pub struct SetDenebForkTimestampCall {
        pub new_deneb_fork_timestamp: u64,
    }
    ///Container type for all input parameters for the `setMaxPods` function with signature `setMaxPods(uint256)` and selector `0x0cf2686d`
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
    #[ethcall(name = "setMaxPods", abi = "setMaxPods(uint256)")]
    pub struct SetMaxPodsCall {
        pub new_max_pods: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `updateBeaconChainOracle` function with signature `updateBeaconChainOracle(address)` and selector `0xc1de3aef`
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
        name = "updateBeaconChainOracle",
        abi = "updateBeaconChainOracle(address)"
    )]
    pub struct UpdateBeaconChainOracleCall {
        pub new_beacon_chain_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawSharesAsTokens` function with signature `withdrawSharesAsTokens(address,address,uint256)` and selector `0x387b1300`
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
        abi = "withdrawSharesAsTokens(address,address,uint256)"
    )]
    pub struct WithdrawSharesAsTokensCall {
        pub pod_owner: ::ethers::core::types::Address,
        pub destination: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EigenPodManagerCalls {
        AddShares(AddSharesCall),
        BeaconChainETHStrategy(BeaconChainETHStrategyCall),
        BeaconChainOracle(BeaconChainOracleCall),
        CreatePod(CreatePodCall),
        DelegationManager(DelegationManagerCall),
        DenebForkTimestamp(DenebForkTimestampCall),
        EigenPodBeacon(EigenPodBeaconCall),
        EthPOS(EthPOSCall),
        GetBlockRootAtTimestamp(GetBlockRootAtTimestampCall),
        GetPod(GetPodCall),
        HasPod(HasPodCall),
        Initialize(InitializeCall),
        MaxPods(MaxPodsCall),
        NumPods(NumPodsCall),
        Owner(OwnerCall),
        OwnerToPod(OwnerToPodCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        PodOwnerShares(PodOwnerSharesCall),
        RecordBeaconChainETHBalanceUpdate(RecordBeaconChainETHBalanceUpdateCall),
        RemoveShares(RemoveSharesCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDenebForkTimestamp(SetDenebForkTimestampCall),
        SetMaxPods(SetMaxPodsCall),
        SetPauserRegistry(SetPauserRegistryCall),
        Slasher(SlasherCall),
        Stake(StakeCall),
        StrategyManager(StrategyManagerCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateBeaconChainOracle(UpdateBeaconChainOracleCall),
        WithdrawSharesAsTokens(WithdrawSharesAsTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for EigenPodManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddShares(decoded));
            }
            if let Ok(decoded) = <BeaconChainETHStrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeaconChainETHStrategy(decoded));
            }
            if let Ok(decoded) = <BeaconChainOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeaconChainOracle(decoded));
            }
            if let Ok(decoded) = <CreatePodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatePod(decoded));
            }
            if let Ok(decoded) = <DelegationManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationManager(decoded));
            }
            if let Ok(decoded) = <DenebForkTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DenebForkTimestamp(decoded));
            }
            if let Ok(decoded) = <EigenPodBeaconCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EigenPodBeacon(decoded));
            }
            if let Ok(decoded) = <EthPOSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EthPOS(decoded));
            }
            if let Ok(decoded) = <GetBlockRootAtTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBlockRootAtTimestamp(decoded));
            }
            if let Ok(decoded) = <GetPodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPod(decoded));
            }
            if let Ok(decoded) = <HasPodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasPod(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MaxPodsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPods(decoded));
            }
            if let Ok(decoded) = <NumPodsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumPods(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <OwnerToPodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerToPod(decoded));
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
            if let Ok(decoded) = <PodOwnerSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PodOwnerShares(decoded));
            }
            if let Ok(decoded) = <RecordBeaconChainETHBalanceUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordBeaconChainETHBalanceUpdate(decoded));
            }
            if let Ok(decoded) = <RemoveSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveShares(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetDenebForkTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDenebForkTimestamp(decoded));
            }
            if let Ok(decoded) = <SetMaxPodsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMaxPods(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyManager(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateBeaconChainOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateBeaconChainOracle(decoded));
            }
            if let Ok(decoded) = <WithdrawSharesAsTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawSharesAsTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EigenPodManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeaconChainETHStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeaconChainOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DenebForkTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EigenPodBeacon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EthPOS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBlockRootAtTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasPod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxPods(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumPods(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerToPod(element) => {
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
                Self::PodOwnerShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordBeaconChainETHBalanceUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDenebForkTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxPods(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategyManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBeaconChainOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawSharesAsTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EigenPodManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeaconChainETHStrategy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconChainOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePod(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::DenebForkTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenPodBeacon(element) => ::core::fmt::Display::fmt(element, f),
                Self::EthPOS(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlockRootAtTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPod(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasPod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPods(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPods(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerToPod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::PodOwnerShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordBeaconChainETHBalanceUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDenebForkTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxPods(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBeaconChainOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawSharesAsTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddSharesCall> for EigenPodManagerCalls {
        fn from(value: AddSharesCall) -> Self {
            Self::AddShares(value)
        }
    }
    impl ::core::convert::From<BeaconChainETHStrategyCall> for EigenPodManagerCalls {
        fn from(value: BeaconChainETHStrategyCall) -> Self {
            Self::BeaconChainETHStrategy(value)
        }
    }
    impl ::core::convert::From<BeaconChainOracleCall> for EigenPodManagerCalls {
        fn from(value: BeaconChainOracleCall) -> Self {
            Self::BeaconChainOracle(value)
        }
    }
    impl ::core::convert::From<CreatePodCall> for EigenPodManagerCalls {
        fn from(value: CreatePodCall) -> Self {
            Self::CreatePod(value)
        }
    }
    impl ::core::convert::From<DelegationManagerCall> for EigenPodManagerCalls {
        fn from(value: DelegationManagerCall) -> Self {
            Self::DelegationManager(value)
        }
    }
    impl ::core::convert::From<DenebForkTimestampCall> for EigenPodManagerCalls {
        fn from(value: DenebForkTimestampCall) -> Self {
            Self::DenebForkTimestamp(value)
        }
    }
    impl ::core::convert::From<EigenPodBeaconCall> for EigenPodManagerCalls {
        fn from(value: EigenPodBeaconCall) -> Self {
            Self::EigenPodBeacon(value)
        }
    }
    impl ::core::convert::From<EthPOSCall> for EigenPodManagerCalls {
        fn from(value: EthPOSCall) -> Self {
            Self::EthPOS(value)
        }
    }
    impl ::core::convert::From<GetBlockRootAtTimestampCall> for EigenPodManagerCalls {
        fn from(value: GetBlockRootAtTimestampCall) -> Self {
            Self::GetBlockRootAtTimestamp(value)
        }
    }
    impl ::core::convert::From<GetPodCall> for EigenPodManagerCalls {
        fn from(value: GetPodCall) -> Self {
            Self::GetPod(value)
        }
    }
    impl ::core::convert::From<HasPodCall> for EigenPodManagerCalls {
        fn from(value: HasPodCall) -> Self {
            Self::HasPod(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for EigenPodManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MaxPodsCall> for EigenPodManagerCalls {
        fn from(value: MaxPodsCall) -> Self {
            Self::MaxPods(value)
        }
    }
    impl ::core::convert::From<NumPodsCall> for EigenPodManagerCalls {
        fn from(value: NumPodsCall) -> Self {
            Self::NumPods(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for EigenPodManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnerToPodCall> for EigenPodManagerCalls {
        fn from(value: OwnerToPodCall) -> Self {
            Self::OwnerToPod(value)
        }
    }
    impl ::core::convert::From<PauseCall> for EigenPodManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for EigenPodManagerCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for EigenPodManagerCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for EigenPodManagerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for EigenPodManagerCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<PodOwnerSharesCall> for EigenPodManagerCalls {
        fn from(value: PodOwnerSharesCall) -> Self {
            Self::PodOwnerShares(value)
        }
    }
    impl ::core::convert::From<RecordBeaconChainETHBalanceUpdateCall>
    for EigenPodManagerCalls {
        fn from(value: RecordBeaconChainETHBalanceUpdateCall) -> Self {
            Self::RecordBeaconChainETHBalanceUpdate(value)
        }
    }
    impl ::core::convert::From<RemoveSharesCall> for EigenPodManagerCalls {
        fn from(value: RemoveSharesCall) -> Self {
            Self::RemoveShares(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for EigenPodManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetDenebForkTimestampCall> for EigenPodManagerCalls {
        fn from(value: SetDenebForkTimestampCall) -> Self {
            Self::SetDenebForkTimestamp(value)
        }
    }
    impl ::core::convert::From<SetMaxPodsCall> for EigenPodManagerCalls {
        fn from(value: SetMaxPodsCall) -> Self {
            Self::SetMaxPods(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for EigenPodManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for EigenPodManagerCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakeCall> for EigenPodManagerCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for EigenPodManagerCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for EigenPodManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for EigenPodManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateBeaconChainOracleCall> for EigenPodManagerCalls {
        fn from(value: UpdateBeaconChainOracleCall) -> Self {
            Self::UpdateBeaconChainOracle(value)
        }
    }
    impl ::core::convert::From<WithdrawSharesAsTokensCall> for EigenPodManagerCalls {
        fn from(value: WithdrawSharesAsTokensCall) -> Self {
            Self::WithdrawSharesAsTokens(value)
        }
    }
    ///Container type for all return fields from the `addShares` function with signature `addShares(address,uint256)` and selector `0x0e81073c`
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
    pub struct AddSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
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
    pub struct BeaconChainETHStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `beaconChainOracle` function with signature `beaconChainOracle()` and selector `0xc052bd61`
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
    pub struct BeaconChainOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `createPod` function with signature `createPod()` and selector `0x84d81062`
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
    pub struct CreatePodReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationManager` function with signature `delegationManager()` and selector `0xea4d3c9b`
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
    pub struct DelegationManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `denebForkTimestamp` function with signature `denebForkTimestamp()` and selector `0x44e71c80`
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
    pub struct DenebForkTimestampReturn(pub u64);
    ///Container type for all return fields from the `eigenPodBeacon` function with signature `eigenPodBeacon()` and selector `0x292b7b2b`
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
    pub struct EigenPodBeaconReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ethPOS` function with signature `ethPOS()` and selector `0x74cdd798`
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
    pub struct EthPOSReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getBlockRootAtTimestamp` function with signature `getBlockRootAtTimestamp(uint64)` and selector `0xd1c64cc9`
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
    pub struct GetBlockRootAtTimestampReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getPod` function with signature `getPod(address)` and selector `0xa38406a3`
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
    pub struct GetPodReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasPod` function with signature `hasPod(address)` and selector `0xf6848d24`
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
    pub struct HasPodReturn(pub bool);
    ///Container type for all return fields from the `maxPods` function with signature `maxPods()` and selector `0xc0ccbf10`
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
    pub struct MaxPodsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numPods` function with signature `numPods()` and selector `0xa6a509be`
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
    pub struct NumPodsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ownerToPod` function with signature `ownerToPod(address)` and selector `0x9ba06275`
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
    pub struct OwnerToPodReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `podOwnerShares` function with signature `podOwnerShares(address)` and selector `0x60f4062b`
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
    pub struct PodOwnerSharesReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
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
}
