pub use index_registry::*;
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
pub mod index_registry {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_registryCoordinator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IRegistryCoordinator",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OPERATOR_DOES_NOT_EXIST_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPERATOR_DOES_NOT_EXIST_ID",
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
                    ::std::borrow::ToOwned::to_owned("currentOperatorIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentOperatorIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("getLatestOperatorUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestOperatorUpdate",
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
                                    name: ::std::borrow::ToOwned::to_owned("operatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IIndexRegistry.OperatorUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getLatestQuorumUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestQuorumUpdate",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IIndexRegistry.QuorumUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorListAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorListAtBlockNumber",
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorUpdateAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorUpdateAtIndex",
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
                                    name: ::std::borrow::ToOwned::to_owned("operatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arrayIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IIndexRegistry.OperatorUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getQuorumUpdateAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumUpdateAtIndex",
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
                                    name: ::std::borrow::ToOwned::to_owned("quorumIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IIndexRegistry.QuorumUpdate",
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
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
                    ::std::borrow::ToOwned::to_owned("totalOperatorsForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "totalOperatorsForQuorum",
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
                    ::std::borrow::ToOwned::to_owned("QuorumIndexUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("QuorumIndexUpdate"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newOperatorIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
    pub static INDEXREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x13\xEC8\x03\x80a\x13\xEC\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x0CV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80a\0Ea\0LV[PPa\x01<V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15a\x01\nW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15a\x01\x1EW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x015W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x12\x80a\x01l`\09`\0\x81\x81a\x01B\x01R\x81\x81a\x02u\x01R\x81\x81a\x04\x1B\x01Ra\x07\xED\x01Ra\x12\x80`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB3W`\x005`\xE0\x1C\x80c\x89\x02bE\x11a\0qW\x80c\x89\x02bE\x14a\x01\xB3W\x80c\xA4\x8B\xB0\xAC\x14a\x01\xD3W\x80c\xBD)\xB8\xCD\x14a\x01\xE6W\x80c\xCA\xA3\xCDv\x14a\x01\xF9W\x80c\xE2\xE6\x85\x80\x14a\x02\x0FW\x80c\xF3A\t\"\x14a\x02UW`\0\x80\xFD[\x80b\xBF\xF0M\x14a\0\xB8W\x80c\x12\xD1\xD7M\x14a\0\xE1W\x80c&\xD9A\xF2\x14a\x01\x15W\x80c.\xD5\x83\xE5\x14a\x01*W\x80cm\x14\xA9\x87\x14a\x01=W\x80c\x81!\x90o\x14a\x01|W[`\0\x80\xFD[a\0\xCBa\0\xC66`\x04a\x0E\xC7V[a\x02hV[`@Qa\0\xD8\x91\x90a\x0FCV[`@Q\x80\x91\x03\x90\xF3[a\0\xF4a\0\xEF6`\x04a\x0F\xB7V[a\x03\xCAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01(a\x01#6`\x04a\x0F\xEAV[a\x04\x10V[\0[a\0\xF4a\x0186`\x04a\x10\x05V[a\x054V[a\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD8V[a\x01\x8Fa\x01\x8A6`\x04a\x0F\xEAV[a\x05\xBAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01\xC6a\x01\xC16`\x04a\x0F\xB7V[a\x06\x01V[`@Qa\0\xD8\x91\x90a\x10HV[a\x01\x8Fa\x01\xE16`\x04a\x0F\xB7V[a\x07kV[a\x01(a\x01\xF46`\x04a\x0E\xC7V[a\x07\xE2V[a\x02\x01`\0\x81V[`@Q\x90\x81R` \x01a\0\xD8V[a\x02@a\x02\x1D6`\x04a\x10\x80V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD8V[a\x02@a\x02c6`\x04a\x0F\xEAV[a\x08\xF0V[``3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x10\xAAV[`@Q\x80\x91\x03\x90\xFD[`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xD6Wa\x02\xD6a\x11\x1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xFFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x03\xBFW`\0\x85\x85\x83\x81\x81\x10a\x03!Wa\x03!a\x113V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x03ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x11IV[`\0a\x03e\x83a\t\x0FV[\x90Pa\x03|\x89\x84a\x03w`\x01\x85a\x11\xB4V[a\n\x08V[\x80\x85\x85\x81Q\x81\x10a\x03\x8FWa\x03\x8Fa\x113V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPPPP\x80\x80a\x03\xB7\x90a\x11\xD9V[\x91PPa\x03\x05V[P\x90P[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x03\xE8\x83\x83a\n\x92V[`@\x80Q\x80\x82\x01\x90\x91R\x81Tc\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01T` \x82\x01R\x90P[\x92\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04XW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x10\xAAV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x90 T\x15a\x04\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FIndexRegistry.createQuorum: quor`D\x82\x01Rpum already exists`x\x1B`d\x82\x01R`\x84\x01a\x02\xB2V[`\xFF\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x82\x84\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x92R\x90\x91 \x80T\x90\x91\x84\x16\x90\x81\x10a\x05\x81Wa\x05\x81a\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x90P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05\xD7\x82a\n\xEAV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16` \x82\x01R\x92\x91PPV[```\0a\x06\x0F\x84\x84a\x0B,V[\x90P`\0\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x062Wa\x062a\x11\x1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06[W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07bWa\x06z\x86\x82\x87a\x0CaV[\x82\x82\x81Q\x81\x10a\x06\x8CWa\x06\x8Ca\x113V[` \x02` \x01\x01\x81\x81RPP`\0\x80\x1B\x82\x82\x81Q\x81\x10a\x06\xAEWa\x06\xAEa\x113V[` \x02` \x01\x01Q\x14\x15a\x07PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`]`$\x82\x01R\x7FIndexRegistry.getOperatorListAtB`D\x82\x01R\x7FlockNumber: operator does not ex`d\x82\x01R\x7Fist at the given block number\0\0\0`\x84\x82\x01R`\xA4\x01a\x02\xB2V[\x80a\x07Z\x81a\x11\xD9V[\x91PPa\x06aV[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x07\xA9Wa\x07\xA9a\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x10\xAAV[`\0[\x81\x81\x10\x15a\x08\xEAW`\0\x83\x83\x83\x81\x81\x10a\x08IWa\x08Ia\x113V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x08\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x11IV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x81 Tc\xFF\xFF\xFF\xFF\x16\x90a\x08\xB0\x84a\r8V[\x90P`\0a\x08\xBE\x85\x83a\rrV[\x90P\x80\x89\x14a\x08\xD2Wa\x08\xD2\x81\x86\x85a\n\x08V[PPPPP\x80\x80a\x08\xE2\x90a\x11\xD9V[\x91PPa\x08-V[PPPPV[`\0a\x08\xFB\x82a\n\xEAV[T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91PPV[`\0\x80a\t\x1B\x83a\n\xEAV[\x80T\x90\x91P`\0\x90a\t;\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x11\xF4V[\x90Pa\tH\x84\x83\x83a\r\x9CV[`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\tf`\x01\x84a\x11\xB4V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta\x03\xC3W`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\t\x9F`\x01\x84a\x11\xB4V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x83Q\x80\x85\x01\x90\x94RC\x83\x16\x84R\x83\x85\x01\x82\x81R\x81T`\x01\x80\x82\x01\x84U\x92\x84R\x95\x90\x92 \x93Q`\x02\x90\x95\x02\x90\x93\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x91Q\x91\x01U\x93\x92PPPV[`\0a\n\x14\x83\x83a\n\x92V[\x90Pa\n\"\x83\x83\x83\x87a\x0E<V[`\xFF\x83\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x85\x91\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\xFF\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x90a\n\xC3`\x01\x83a\x12\x1CV[\x81T\x81\x10a\n\xD3Wa\n\xD3a\x113V[\x90`\0R` `\0 \x90`\x02\x02\x01\x91PP\x92\x91PPV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x90a\x0B\n`\x01\x83a\x12\x1CV[\x81T\x81\x10a\x0B\x1AWa\x0B\x1Aa\x113V[\x90`\0R` `\0 \x01\x91PP\x91\x90PV[`\xFF\x82\x16`\0\x90\x81R`\x03` R`@\x81 T\x80[\x80\x15a\x0B\xD4W`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x81 a\x0Bd`\x01\x84a\x12\x1CV[\x81T\x81\x10a\x0BtWa\x0Bta\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\x0B\xC1W` \x01Q\x92Pa\x04\n\x91PPV[P\x80a\x0B\xCC\x81a\x123V[\x91PPa\x0BAV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x01a\x02\xB2V[`\xFF\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a\r,W`\xFF\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 a\x0C\xBB`\x01\x84a\x12\x1CV[\x81T\x81\x10a\x0C\xCBWa\x0C\xCBa\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\r\x19W` \x01Q\x92Pa\x03\xC3\x91PPV[P\x80a\r$\x81a\x123V[\x91PPa\x0C\x87V[P`\0\x95\x94PPPPPV[`\0\x80a\rD\x83a\n\xEAV[\x80T\x90\x91P`\0\x90a\re\x90`\x01\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x11\xB4V[\x90Pa\x03\xC3\x84\x83\x83a\r\x9CV[`\0\x80a\r\x7F\x84\x84a\n\x92V[`\x01\x81\x01T\x90\x91Pa\r\x94\x85\x85\x84`\0a\x0E<V[\x94\x93PPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\r\xD3W\x81Tc\xFF\xFF\xFF\xFF\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x17\x82UPPPV[`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x85\x81\x16\x83\x85\x01\x90\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x0E[W`\x01\x82\x01\x81\x90Ua\x08\xEAV[`\xFF\x93\x90\x93\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x96\x87\x16\x85R\x82R\x80\x84 \x81Q\x80\x83\x01\x90\x92RC\x87\x16\x82R\x81\x83\x01\x97\x88R\x80T`\x01\x80\x82\x01\x83U\x91\x86R\x92\x90\x94 \x90Q\x91\x90\x92\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x94\x16\x17\x83U\x92Q\x91\x90\x92\x01UPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0E\xDCW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xFBW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F\x0FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\x1EW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x0F0W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\x81W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F_V[P\x90\x96\x95PPPPPPV[\x805`\xFF\x81\x16\x81\x14a\x0F\x9EW`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x9EW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xCAW`\0\x80\xFD[a\x0F\xD3\x83a\x0F\x8DV[\x91Pa\x0F\xE1` \x84\x01a\x0F\xA3V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xFCW`\0\x80\xFD[a\x03\xC3\x82a\x0F\x8DV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\x1AW`\0\x80\xFD[a\x10#\x84a\x0F\x8DV[\x92Pa\x101` \x85\x01a\x0F\xA3V[\x91Pa\x10?`@\x85\x01a\x0F\xA3V[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\x81W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10dV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x93W`\0\x80\xFD[a\x10\x9C\x83a\x0F\x8DV[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R`M\x90\x82\x01R\x7FIndexRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the registr``\x82\x01Rl<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x99\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`5\x90\x82\x01R\x7FIndexRegistry.registerOperator: `@\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x11\xD1Wa\x11\xD1a\x11\x9EV[\x03\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a\x11\xEDWa\x11\xEDa\x11\x9EV[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x12\x13Wa\x12\x13a\x11\x9EV[\x01\x94\x93PPPPV[`\0\x82\x82\x10\x15a\x12.Wa\x12.a\x11\x9EV[P\x03\x90V[`\0\x81a\x12BWa\x12Ba\x11\x9EV[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 Z\xBAIfy\x0FOkfY\xF0g\xB7\xBB\x99}\xA5X\x1D\xBC\xF3\x12\x8Eg\xAC\xF1\x15<\xB1\x8D\\qdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static INDEXREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB3W`\x005`\xE0\x1C\x80c\x89\x02bE\x11a\0qW\x80c\x89\x02bE\x14a\x01\xB3W\x80c\xA4\x8B\xB0\xAC\x14a\x01\xD3W\x80c\xBD)\xB8\xCD\x14a\x01\xE6W\x80c\xCA\xA3\xCDv\x14a\x01\xF9W\x80c\xE2\xE6\x85\x80\x14a\x02\x0FW\x80c\xF3A\t\"\x14a\x02UW`\0\x80\xFD[\x80b\xBF\xF0M\x14a\0\xB8W\x80c\x12\xD1\xD7M\x14a\0\xE1W\x80c&\xD9A\xF2\x14a\x01\x15W\x80c.\xD5\x83\xE5\x14a\x01*W\x80cm\x14\xA9\x87\x14a\x01=W\x80c\x81!\x90o\x14a\x01|W[`\0\x80\xFD[a\0\xCBa\0\xC66`\x04a\x0E\xC7V[a\x02hV[`@Qa\0\xD8\x91\x90a\x0FCV[`@Q\x80\x91\x03\x90\xF3[a\0\xF4a\0\xEF6`\x04a\x0F\xB7V[a\x03\xCAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01(a\x01#6`\x04a\x0F\xEAV[a\x04\x10V[\0[a\0\xF4a\x0186`\x04a\x10\x05V[a\x054V[a\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD8V[a\x01\x8Fa\x01\x8A6`\x04a\x0F\xEAV[a\x05\xBAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01\xC6a\x01\xC16`\x04a\x0F\xB7V[a\x06\x01V[`@Qa\0\xD8\x91\x90a\x10HV[a\x01\x8Fa\x01\xE16`\x04a\x0F\xB7V[a\x07kV[a\x01(a\x01\xF46`\x04a\x0E\xC7V[a\x07\xE2V[a\x02\x01`\0\x81V[`@Q\x90\x81R` \x01a\0\xD8V[a\x02@a\x02\x1D6`\x04a\x10\x80V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD8V[a\x02@a\x02c6`\x04a\x0F\xEAV[a\x08\xF0V[``3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x10\xAAV[`@Q\x80\x91\x03\x90\xFD[`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xD6Wa\x02\xD6a\x11\x1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xFFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x03\xBFW`\0\x85\x85\x83\x81\x81\x10a\x03!Wa\x03!a\x113V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x03ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x11IV[`\0a\x03e\x83a\t\x0FV[\x90Pa\x03|\x89\x84a\x03w`\x01\x85a\x11\xB4V[a\n\x08V[\x80\x85\x85\x81Q\x81\x10a\x03\x8FWa\x03\x8Fa\x113V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPPPP\x80\x80a\x03\xB7\x90a\x11\xD9V[\x91PPa\x03\x05V[P\x90P[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x03\xE8\x83\x83a\n\x92V[`@\x80Q\x80\x82\x01\x90\x91R\x81Tc\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01T` \x82\x01R\x90P[\x92\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04XW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x10\xAAV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x90 T\x15a\x04\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FIndexRegistry.createQuorum: quor`D\x82\x01Rpum already exists`x\x1B`d\x82\x01R`\x84\x01a\x02\xB2V[`\xFF\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x82\x84\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x92R\x90\x91 \x80T\x90\x91\x84\x16\x90\x81\x10a\x05\x81Wa\x05\x81a\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x90P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05\xD7\x82a\n\xEAV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16` \x82\x01R\x92\x91PPV[```\0a\x06\x0F\x84\x84a\x0B,V[\x90P`\0\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x062Wa\x062a\x11\x1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06[W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07bWa\x06z\x86\x82\x87a\x0CaV[\x82\x82\x81Q\x81\x10a\x06\x8CWa\x06\x8Ca\x113V[` \x02` \x01\x01\x81\x81RPP`\0\x80\x1B\x82\x82\x81Q\x81\x10a\x06\xAEWa\x06\xAEa\x113V[` \x02` \x01\x01Q\x14\x15a\x07PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`]`$\x82\x01R\x7FIndexRegistry.getOperatorListAtB`D\x82\x01R\x7FlockNumber: operator does not ex`d\x82\x01R\x7Fist at the given block number\0\0\0`\x84\x82\x01R`\xA4\x01a\x02\xB2V[\x80a\x07Z\x81a\x11\xD9V[\x91PPa\x06aV[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x07\xA9Wa\x07\xA9a\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x10\xAAV[`\0[\x81\x81\x10\x15a\x08\xEAW`\0\x83\x83\x83\x81\x81\x10a\x08IWa\x08Ia\x113V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x08\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xB2\x90a\x11IV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x81 Tc\xFF\xFF\xFF\xFF\x16\x90a\x08\xB0\x84a\r8V[\x90P`\0a\x08\xBE\x85\x83a\rrV[\x90P\x80\x89\x14a\x08\xD2Wa\x08\xD2\x81\x86\x85a\n\x08V[PPPPP\x80\x80a\x08\xE2\x90a\x11\xD9V[\x91PPa\x08-V[PPPPV[`\0a\x08\xFB\x82a\n\xEAV[T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91PPV[`\0\x80a\t\x1B\x83a\n\xEAV[\x80T\x90\x91P`\0\x90a\t;\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x11\xF4V[\x90Pa\tH\x84\x83\x83a\r\x9CV[`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\tf`\x01\x84a\x11\xB4V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta\x03\xC3W`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\t\x9F`\x01\x84a\x11\xB4V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x83Q\x80\x85\x01\x90\x94RC\x83\x16\x84R\x83\x85\x01\x82\x81R\x81T`\x01\x80\x82\x01\x84U\x92\x84R\x95\x90\x92 \x93Q`\x02\x90\x95\x02\x90\x93\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x91Q\x91\x01U\x93\x92PPPV[`\0a\n\x14\x83\x83a\n\x92V[\x90Pa\n\"\x83\x83\x83\x87a\x0E<V[`\xFF\x83\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x85\x91\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\xFF\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x90a\n\xC3`\x01\x83a\x12\x1CV[\x81T\x81\x10a\n\xD3Wa\n\xD3a\x113V[\x90`\0R` `\0 \x90`\x02\x02\x01\x91PP\x92\x91PPV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x90a\x0B\n`\x01\x83a\x12\x1CV[\x81T\x81\x10a\x0B\x1AWa\x0B\x1Aa\x113V[\x90`\0R` `\0 \x01\x91PP\x91\x90PV[`\xFF\x82\x16`\0\x90\x81R`\x03` R`@\x81 T\x80[\x80\x15a\x0B\xD4W`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x81 a\x0Bd`\x01\x84a\x12\x1CV[\x81T\x81\x10a\x0BtWa\x0Bta\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\x0B\xC1W` \x01Q\x92Pa\x04\n\x91PPV[P\x80a\x0B\xCC\x81a\x123V[\x91PPa\x0BAV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x01a\x02\xB2V[`\xFF\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a\r,W`\xFF\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 a\x0C\xBB`\x01\x84a\x12\x1CV[\x81T\x81\x10a\x0C\xCBWa\x0C\xCBa\x113V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\r\x19W` \x01Q\x92Pa\x03\xC3\x91PPV[P\x80a\r$\x81a\x123V[\x91PPa\x0C\x87V[P`\0\x95\x94PPPPPV[`\0\x80a\rD\x83a\n\xEAV[\x80T\x90\x91P`\0\x90a\re\x90`\x01\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x11\xB4V[\x90Pa\x03\xC3\x84\x83\x83a\r\x9CV[`\0\x80a\r\x7F\x84\x84a\n\x92V[`\x01\x81\x01T\x90\x91Pa\r\x94\x85\x85\x84`\0a\x0E<V[\x94\x93PPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\r\xD3W\x81Tc\xFF\xFF\xFF\xFF\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x17\x82UPPPV[`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x85\x81\x16\x83\x85\x01\x90\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x0E[W`\x01\x82\x01\x81\x90Ua\x08\xEAV[`\xFF\x93\x90\x93\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x96\x87\x16\x85R\x82R\x80\x84 \x81Q\x80\x83\x01\x90\x92RC\x87\x16\x82R\x81\x83\x01\x97\x88R\x80T`\x01\x80\x82\x01\x83U\x91\x86R\x92\x90\x94 \x90Q\x91\x90\x92\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x94\x16\x17\x83U\x92Q\x91\x90\x92\x01UPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0E\xDCW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xFBW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F\x0FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\x1EW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x0F0W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\x81W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F_V[P\x90\x96\x95PPPPPPV[\x805`\xFF\x81\x16\x81\x14a\x0F\x9EW`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x9EW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xCAW`\0\x80\xFD[a\x0F\xD3\x83a\x0F\x8DV[\x91Pa\x0F\xE1` \x84\x01a\x0F\xA3V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xFCW`\0\x80\xFD[a\x03\xC3\x82a\x0F\x8DV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\x1AW`\0\x80\xFD[a\x10#\x84a\x0F\x8DV[\x92Pa\x101` \x85\x01a\x0F\xA3V[\x91Pa\x10?`@\x85\x01a\x0F\xA3V[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\x81W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10dV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x93W`\0\x80\xFD[a\x10\x9C\x83a\x0F\x8DV[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R`M\x90\x82\x01R\x7FIndexRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the registr``\x82\x01Rl<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x99\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`5\x90\x82\x01R\x7FIndexRegistry.registerOperator: `@\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x11\xD1Wa\x11\xD1a\x11\x9EV[\x03\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a\x11\xEDWa\x11\xEDa\x11\x9EV[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x12\x13Wa\x12\x13a\x11\x9EV[\x01\x94\x93PPPPV[`\0\x82\x82\x10\x15a\x12.Wa\x12.a\x11\x9EV[P\x03\x90V[`\0\x81a\x12BWa\x12Ba\x11\x9EV[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 Z\xBAIfy\x0FOkfY\xF0g\xB7\xBB\x99}\xA5X\x1D\xBC\xF3\x12\x8Eg\xAC\xF1\x15<\xB1\x8D\\qdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static INDEXREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct IndexRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IndexRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IndexRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IndexRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IndexRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IndexRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IndexRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INDEXREGISTRY_ABI.clone(),
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
                INDEXREGISTRY_ABI.clone(),
                INDEXREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `OPERATOR_DOES_NOT_EXIST_ID` (0xcaa3cd76) function
        pub fn operator_does_not_exist_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([202, 163, 205, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentOperatorIndex` (0xe2e68580) function
        pub fn current_operator_index(
            &self,
            p0: u8,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([226, 230, 133, 128], (p0, p1))
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
        ///Calls the contract's `getLatestOperatorUpdate` (0x12d1d74d) function
        pub fn get_latest_operator_update(
            &self,
            quorum_number: u8,
            operator_index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorUpdate> {
            self.0
                .method_hash([18, 209, 215, 77], (quorum_number, operator_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestQuorumUpdate` (0x8121906f) function
        pub fn get_latest_quorum_update(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumUpdate> {
            self.0
                .method_hash([129, 33, 144, 111], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorListAtBlockNumber` (0x89026245) function
        pub fn get_operator_list_at_block_number(
            &self,
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([137, 2, 98, 69], (quorum_number, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorUpdateAtIndex` (0x2ed583e5) function
        pub fn get_operator_update_at_index(
            &self,
            quorum_number: u8,
            operator_index: u32,
            array_index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorUpdate> {
            self.0
                .method_hash(
                    [46, 213, 131, 229],
                    (quorum_number, operator_index, array_index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumUpdateAtIndex` (0xa48bb0ac) function
        pub fn get_quorum_update_at_index(
            &self,
            quorum_number: u8,
            quorum_index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumUpdate> {
            self.0
                .method_hash([164, 139, 176, 172], (quorum_number, quorum_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeQuorum` (0x26d941f2) function
        pub fn initialize_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 217, 65, 242], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x00bff04d) function
        pub fn register_operator(
            &self,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([0, 191, 240, 77], (operator_id, quorum_numbers))
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
        ///Calls the contract's `totalOperatorsForQuorum` (0xf3410922) function
        pub fn total_operators_for_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([243, 65, 9, 34], quorum_number)
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
        ///Gets the contract's `QuorumIndexUpdate` event
        pub fn quorum_index_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuorumIndexUpdateFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IndexRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IndexRegistry<M> {
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
        name = "QuorumIndexUpdate",
        abi = "QuorumIndexUpdate(bytes32,uint8,uint32)"
    )]
    pub struct QuorumIndexUpdateFilter {
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub new_operator_index: u32,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IndexRegistryEvents {
        InitializedFilter(InitializedFilter),
        QuorumIndexUpdateFilter(QuorumIndexUpdateFilter),
    }
    impl ::ethers::contract::EthLogDecode for IndexRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(IndexRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = QuorumIndexUpdateFilter::decode_log(log) {
                return Ok(IndexRegistryEvents::QuorumIndexUpdateFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IndexRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumIndexUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for IndexRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<QuorumIndexUpdateFilter> for IndexRegistryEvents {
        fn from(value: QuorumIndexUpdateFilter) -> Self {
            Self::QuorumIndexUpdateFilter(value)
        }
    }
    ///Container type for all input parameters for the `OPERATOR_DOES_NOT_EXIST_ID` function with signature `OPERATOR_DOES_NOT_EXIST_ID()` and selector `0xcaa3cd76`
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
    #[ethcall(name = "OPERATOR_DOES_NOT_EXIST_ID", abi = "OPERATOR_DOES_NOT_EXIST_ID()")]
    pub struct OperatorDoesNotExistIdCall;
    ///Container type for all input parameters for the `currentOperatorIndex` function with signature `currentOperatorIndex(uint8,bytes32)` and selector `0xe2e68580`
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
        name = "currentOperatorIndex",
        abi = "currentOperatorIndex(uint8,bytes32)"
    )]
    pub struct CurrentOperatorIndexCall(pub u8, pub [u8; 32]);
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
    ///Container type for all input parameters for the `getLatestOperatorUpdate` function with signature `getLatestOperatorUpdate(uint8,uint32)` and selector `0x12d1d74d`
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
        name = "getLatestOperatorUpdate",
        abi = "getLatestOperatorUpdate(uint8,uint32)"
    )]
    pub struct GetLatestOperatorUpdateCall {
        pub quorum_number: u8,
        pub operator_index: u32,
    }
    ///Container type for all input parameters for the `getLatestQuorumUpdate` function with signature `getLatestQuorumUpdate(uint8)` and selector `0x8121906f`
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
    #[ethcall(name = "getLatestQuorumUpdate", abi = "getLatestQuorumUpdate(uint8)")]
    pub struct GetLatestQuorumUpdateCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getOperatorListAtBlockNumber` function with signature `getOperatorListAtBlockNumber(uint8,uint32)` and selector `0x89026245`
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
        name = "getOperatorListAtBlockNumber",
        abi = "getOperatorListAtBlockNumber(uint8,uint32)"
    )]
    pub struct GetOperatorListAtBlockNumberCall {
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getOperatorUpdateAtIndex` function with signature `getOperatorUpdateAtIndex(uint8,uint32,uint32)` and selector `0x2ed583e5`
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
        name = "getOperatorUpdateAtIndex",
        abi = "getOperatorUpdateAtIndex(uint8,uint32,uint32)"
    )]
    pub struct GetOperatorUpdateAtIndexCall {
        pub quorum_number: u8,
        pub operator_index: u32,
        pub array_index: u32,
    }
    ///Container type for all input parameters for the `getQuorumUpdateAtIndex` function with signature `getQuorumUpdateAtIndex(uint8,uint32)` and selector `0xa48bb0ac`
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
        name = "getQuorumUpdateAtIndex",
        abi = "getQuorumUpdateAtIndex(uint8,uint32)"
    )]
    pub struct GetQuorumUpdateAtIndexCall {
        pub quorum_number: u8,
        pub quorum_index: u32,
    }
    ///Container type for all input parameters for the `initializeQuorum` function with signature `initializeQuorum(uint8)` and selector `0x26d941f2`
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
    #[ethcall(name = "initializeQuorum", abi = "initializeQuorum(uint8)")]
    pub struct InitializeQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(bytes32,bytes)` and selector `0x00bff04d`
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
    #[ethcall(name = "registerOperator", abi = "registerOperator(bytes32,bytes)")]
    pub struct RegisterOperatorCall {
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
    ///Container type for all input parameters for the `totalOperatorsForQuorum` function with signature `totalOperatorsForQuorum(uint8)` and selector `0xf3410922`
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
    #[ethcall(name = "totalOperatorsForQuorum", abi = "totalOperatorsForQuorum(uint8)")]
    pub struct TotalOperatorsForQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IndexRegistryCalls {
        OperatorDoesNotExistId(OperatorDoesNotExistIdCall),
        CurrentOperatorIndex(CurrentOperatorIndexCall),
        DeregisterOperator(DeregisterOperatorCall),
        GetLatestOperatorUpdate(GetLatestOperatorUpdateCall),
        GetLatestQuorumUpdate(GetLatestQuorumUpdateCall),
        GetOperatorListAtBlockNumber(GetOperatorListAtBlockNumberCall),
        GetOperatorUpdateAtIndex(GetOperatorUpdateAtIndexCall),
        GetQuorumUpdateAtIndex(GetQuorumUpdateAtIndexCall),
        InitializeQuorum(InitializeQuorumCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        TotalOperatorsForQuorum(TotalOperatorsForQuorumCall),
    }
    impl ::ethers::core::abi::AbiDecode for IndexRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <OperatorDoesNotExistIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorDoesNotExistId(decoded));
            }
            if let Ok(decoded) = <CurrentOperatorIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentOperatorIndex(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <GetLatestOperatorUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestOperatorUpdate(decoded));
            }
            if let Ok(decoded) = <GetLatestQuorumUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestQuorumUpdate(decoded));
            }
            if let Ok(decoded) = <GetOperatorListAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorListAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetOperatorUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorUpdateAtIndex(decoded));
            }
            if let Ok(decoded) = <GetQuorumUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumUpdateAtIndex(decoded));
            }
            if let Ok(decoded) = <InitializeQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeQuorum(decoded));
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
            if let Ok(decoded) = <TotalOperatorsForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalOperatorsForQuorum(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IndexRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OperatorDoesNotExistId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentOperatorIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestOperatorUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestQuorumUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorListAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalOperatorsForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IndexRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OperatorDoesNotExistId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentOperatorIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeregisterOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestOperatorUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestQuorumUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorListAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorUpdateAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumUpdateAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalOperatorsForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OperatorDoesNotExistIdCall> for IndexRegistryCalls {
        fn from(value: OperatorDoesNotExistIdCall) -> Self {
            Self::OperatorDoesNotExistId(value)
        }
    }
    impl ::core::convert::From<CurrentOperatorIndexCall> for IndexRegistryCalls {
        fn from(value: CurrentOperatorIndexCall) -> Self {
            Self::CurrentOperatorIndex(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for IndexRegistryCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetLatestOperatorUpdateCall> for IndexRegistryCalls {
        fn from(value: GetLatestOperatorUpdateCall) -> Self {
            Self::GetLatestOperatorUpdate(value)
        }
    }
    impl ::core::convert::From<GetLatestQuorumUpdateCall> for IndexRegistryCalls {
        fn from(value: GetLatestQuorumUpdateCall) -> Self {
            Self::GetLatestQuorumUpdate(value)
        }
    }
    impl ::core::convert::From<GetOperatorListAtBlockNumberCall> for IndexRegistryCalls {
        fn from(value: GetOperatorListAtBlockNumberCall) -> Self {
            Self::GetOperatorListAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetOperatorUpdateAtIndexCall> for IndexRegistryCalls {
        fn from(value: GetOperatorUpdateAtIndexCall) -> Self {
            Self::GetOperatorUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumUpdateAtIndexCall> for IndexRegistryCalls {
        fn from(value: GetQuorumUpdateAtIndexCall) -> Self {
            Self::GetQuorumUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<InitializeQuorumCall> for IndexRegistryCalls {
        fn from(value: InitializeQuorumCall) -> Self {
            Self::InitializeQuorum(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for IndexRegistryCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for IndexRegistryCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<TotalOperatorsForQuorumCall> for IndexRegistryCalls {
        fn from(value: TotalOperatorsForQuorumCall) -> Self {
            Self::TotalOperatorsForQuorum(value)
        }
    }
    ///Container type for all return fields from the `OPERATOR_DOES_NOT_EXIST_ID` function with signature `OPERATOR_DOES_NOT_EXIST_ID()` and selector `0xcaa3cd76`
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
    pub struct OperatorDoesNotExistIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `currentOperatorIndex` function with signature `currentOperatorIndex(uint8,bytes32)` and selector `0xe2e68580`
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
    pub struct CurrentOperatorIndexReturn(pub u32);
    ///Container type for all return fields from the `getLatestOperatorUpdate` function with signature `getLatestOperatorUpdate(uint8,uint32)` and selector `0x12d1d74d`
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
    pub struct GetLatestOperatorUpdateReturn(pub OperatorUpdate);
    ///Container type for all return fields from the `getLatestQuorumUpdate` function with signature `getLatestQuorumUpdate(uint8)` and selector `0x8121906f`
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
    pub struct GetLatestQuorumUpdateReturn(pub QuorumUpdate);
    ///Container type for all return fields from the `getOperatorListAtBlockNumber` function with signature `getOperatorListAtBlockNumber(uint8,uint32)` and selector `0x89026245`
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
    pub struct GetOperatorListAtBlockNumberReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getOperatorUpdateAtIndex` function with signature `getOperatorUpdateAtIndex(uint8,uint32,uint32)` and selector `0x2ed583e5`
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
    pub struct GetOperatorUpdateAtIndexReturn(pub OperatorUpdate);
    ///Container type for all return fields from the `getQuorumUpdateAtIndex` function with signature `getQuorumUpdateAtIndex(uint8,uint32)` and selector `0xa48bb0ac`
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
    pub struct GetQuorumUpdateAtIndexReturn(pub QuorumUpdate);
    ///Container type for all return fields from the `registerOperator` function with signature `registerOperator(bytes32,bytes)` and selector `0x00bff04d`
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
    pub struct RegisterOperatorReturn(pub ::std::vec::Vec<u32>);
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
    ///Container type for all return fields from the `totalOperatorsForQuorum` function with signature `totalOperatorsForQuorum(uint8)` and selector `0xf3410922`
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
    pub struct TotalOperatorsForQuorumReturn(pub u32);
}
