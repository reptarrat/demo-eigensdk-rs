pub use bls_signature_checker::*;
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
pub mod bls_signature_checker {
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
                    ::std::borrow::ToOwned::to_owned("blsApkRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blsApkRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBLSApkRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "referenceBlockNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.QuorumStakeTotals",
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
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
                    ::std::borrow::ToOwned::to_owned("setStaleStakesForbidden"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStaleStakesForbidden",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStakeRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("staleStakesForbidden"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "staleStakesForbidden",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("trySignatureAndApkVerification"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "trySignatureAndApkVerification",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apkG2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pairingSuccessful"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("siganatureIsValid"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("StaleStakesForbiddenUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StaleStakesForbiddenUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
    pub static BLSSIGNATURECHECKER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0).8\x03\x80b\0).\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80\x81\x90R`@\x80Qch0H5`\xE0\x1B\x81R\x90Qch0H5\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\0\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xA6\x91\x90b\0\x01\xDDV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01$\x91\x90b\0\x01\xDDV[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\xA0Q`\x01`\x01`\xA0\x1B\x03\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xA4\x91\x90b\0\x01\xDDV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0RP`\0\x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x02\x04V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xDAW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x01\xF0W`\0\x80\xFD[\x81Qb\0\x01\xFD\x81b\0\x01\xC4V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa&\xB9b\0\x02u`\09`\0\x81\x81a\x01\xA1\x01Ra\x0B\xB1\x01R`\0\x81\x81`\xD6\x01Ra\r\x94\x01R`\0\x81\x81a\x01\x15\x01R\x81\x81a\x0Fj\x01Ra\x11,\x01R`\0\x81\x81a\x01<\x01R\x81\x81a\x03O\x01R\x81\x81a\x08z\x01R\x81\x81a\n\x12\x01Ra\x0CN\x01Ra&\xB9`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0[W\x80cm\x14\xA9\x87\x14a\x017W\x80cn\xFBF6\x14a\x01^W\x80c\xB9\x8D\t\x08\x14a\x01\x7FW\x80c\xDF\\\xF7#\x14a\x01\x9CW`\0\x80\xFD[\x80c\x17\x1F\x1D[\x14a\0\x8DW\x80cAl~^\x14a\0\xBCW\x80c]\xF4YF\x14a\0\xD1W\x80ch0H5\x14a\x01\x10W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\x1F\xB3V[a\x01\xC3V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCFa\0\xCA6`\x04a \x04V[a\x03MV[\0[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01qa\x01l6`\x04a\"\xDCV[a\x04\xC7V[`@Qa\0\xB3\x92\x91\x90a#\xCFV[`\0Ta\x01\x8C\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xB3V[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x02\x0BWa\x02\x0Ba$\x18V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x020Wa\x020a$\x18V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x02LWa\x02La$\x18V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x02\xA9\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x02\xCC\x91\x90a$.V[\x90Pa\x03?a\x02\xE5a\x02\xDE\x88\x84a\x13\xE1V[\x86\x90a\x14xV[a\x02\xEDa\x15\x0CV[a\x035a\x03&\x85a\x03 `@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x13\xE1V[a\x03/\x8Ca\x15\xCCV[\x90a\x14xV[\x88b\x01\xD4\xC0a\x16\\V[\x90\x98\x90\x97P\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCF\x91\x90a$PV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x84a\x05>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04wV[`@\x83\x01QQ\x85\x14\x80\x15a\x05VWP`\xA0\x83\x01QQ\x85\x14[\x80\x15a\x05fWP`\xC0\x83\x01QQ\x85\x14[\x80\x15a\x05vWP`\xE0\x83\x01QQ\x85\x14[a\x05\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x82QQ` \x84\x01QQ\x14a\x06XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a&d\x839\x81Q\x91R\x90\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x06\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x01a\x04wV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R``\x80\x84R\x90\x83\x01R\x90\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x08Wa\x07\x08a\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x071W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07OWa\x07Oa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07xW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x85` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xACWa\x07\xACa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xD5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x86\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xF5Wa\x07\xF5a\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\x1EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81` \x01\x81\x90RP`\0a\x08\xF0\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\x9A\xA1e=\x92P`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xEB\x91\x90a$yV[a\x18\x80V[\x90P`\0[\x87` \x01QQ\x81\x10\x15a\x0B\x8BWa\t:\x88` \x01Q\x82\x81Q\x81\x10a\t\x1BWa\t\x1Ba$\x18V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83` \x01Q\x82\x81Q\x81\x10a\tPWa\tPa$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\n\x10W` \x83\x01Qa\tq`\x01\x83a$\xB2V[\x81Q\x81\x10a\t\x81Wa\t\x81a$\x18V[` \x02` \x01\x01Q`\0\x1C\x83` \x01Q\x82\x81Q\x81\x10a\t\xA2Wa\t\xA2a$\x18V[` \x02` \x01\x01Q`\0\x1C\x11a\n\x10W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x04wV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\nUWa\nUa$\x18V[` \x02` \x01\x01Q\x8B\x8B`\0\x01Q\x85\x81Q\x81\x10a\ntWa\nta$\x18V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xB1\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xF2\x91\x90a$\xC9V[`\x01`\x01`\xC0\x1B\x03\x16\x83`\0\x01Q\x82\x81Q\x81\x10a\x0B\x11Wa\x0B\x11a$\x18V[` \x02` \x01\x01\x81\x81RPPa\x0Bwa\x02\xDEa\x0BK\x84\x86`\0\x01Q\x85\x81Q\x81\x10a\x0B=Wa\x0B=a$\x18V[` \x02` \x01\x01Q\x16a\x19\x13V[\x8A` \x01Q\x84\x81Q\x81\x10a\x0BaWa\x0Baa$\x18V[` \x02` \x01\x01Qa\x19>\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80a\x0B\x83\x81a$\xF2V[\x91PPa\x08\xF5V[PPa\x0B\x96\x83a\x1A\"V[`\0\x80T\x91\x94P`\xFF\x90\x91\x16\x90\x81a\x0B\xAFW`\0a\x0C1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4H\xFE\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C1\x91\x90a%\rV[\x90P`\0[\x8A\x81\x10\x15a\x12\xB0W\x82\x15a\r\x92W\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a\x0C\x8DWa\x0C\x8Da$\x18V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x92\x015`\xF8\x1C`\x04\x83\x01RP`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xF1\x91\x90a%\rV[a\x0C\xFB\x91\x90a%&V[\x10\x15a\r\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x04wV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a\r\xD3Wa\r\xD3a$\x18V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a\r\xF7Wa\r\xF7a$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ESW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ew\x91\x90a%>V[`\x01`\x01`@\x1B\x03\x19\x16a\x0E\x9A\x8A`@\x01Q\x83\x81Q\x81\x10a\t\x1BWa\t\x1Ba$\x18V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x0F6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x04wV[a\x0Ff\x89`@\x01Q\x82\x81Q\x81\x10a\x0FOWa\x0FOa$\x18V[` \x02` \x01\x01Q\x87a\x14x\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a\x0F\xA9Wa\x0F\xA9a$\x18V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a\x0F\xCDWa\x0F\xCDa$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10M\x91\x90a%iV[\x85` \x01Q\x82\x81Q\x81\x10a\x10cWa\x10ca$\x18V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q\x82\x90\x81\x10a\x10\x8FWa\x10\x8Fa$\x18V[` \x02` \x01\x01Q\x85`\0\x01Q\x82\x81Q\x81\x10a\x10\xADWa\x10\xADa$\x18V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0\x80[\x8A` \x01QQ\x81\x10\x15a\x12\x9BWa\x11%\x86`\0\x01Q\x82\x81Q\x81\x10a\x10\xF7Wa\x10\xF7a$\x18V[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a\x11\x11Wa\x11\x11a$\x18V[`\x01\x92\x015`\xF8\x1C\x92\x90\x92\x1C\x81\x16\x14\x91\x90PV[\x15a\x12\x89W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a\x11kWa\x11ka$\x18V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a\x11\x8FWa\x11\x8Fa$\x18V[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a\x11\xADWa\x11\xADa$\x18V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x11\xC6Wa\x11\xC6a$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12N\x91\x90a%iV[\x87Q\x80Q\x85\x90\x81\x10a\x12bWa\x12ba$\x18V[` \x02` \x01\x01\x81\x81Qa\x12v\x91\x90a%\x92V[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x90\x91\x01\x90[\x80a\x12\x93\x81a$\xF2V[\x91PPa\x10\xD1V[PP\x80\x80a\x12\xA8\x90a$\xF2V[\x91PPa\x0C6V[PPP`\0\x80a\x12\xCA\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x01\xC3V[\x91P\x91P\x81a\x13;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x80a\x13\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04wV[PP`\0\x87\x82` \x01Q`@Q` \x01a\x13\xB7\x92\x91\x90a%\xBAV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xFDa\x1DtV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x140Wa\x142V[\xFE[P\x80a\x14pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04wV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14\x94a\x1D\x92V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x140WP\x80a\x14pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04wV[a\x15\x14a\x1D\xB0V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a\x15\xFC`\0\x80Q` a&D\x839\x81Q\x91R\x86a$.V[\x90P[a\x16\x08\x81a\x1A\xBDV[\x90\x93P\x91P`\0\x80Q` a&D\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a\x16BW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a&D\x839\x81Q\x91R`\x01\x82\x08\x90Pa\x15\xFFV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a\x16\x8Ea\x1D\xD5V[`\0[`\x02\x81\x10\x15a\x18SW`\0a\x16\xA7\x82`\x06a&\x02V[\x90P\x84\x82`\x02\x81\x10a\x16\xBBWa\x16\xBBa$\x18V[` \x02\x01QQ\x83a\x16\xCD\x83`\0a%&V[`\x0C\x81\x10a\x16\xDDWa\x16\xDDa$\x18V[` \x02\x01R\x84\x82`\x02\x81\x10a\x16\xF4Wa\x16\xF4a$\x18V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x17\x0B\x91\x90a%&V[`\x0C\x81\x10a\x17\x1BWa\x17\x1Ba$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x172Wa\x172a$\x18V[` \x02\x01QQQ\x83a\x17E\x83`\x02a%&V[`\x0C\x81\x10a\x17UWa\x17Ua$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17lWa\x17la$\x18V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x17\x85\x83`\x03a%&V[`\x0C\x81\x10a\x17\x95Wa\x17\x95a$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\xACWa\x17\xACa$\x18V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x17\xC7Wa\x17\xC7a$\x18V[` \x02\x01Q\x83a\x17\xD8\x83`\x04a%&V[`\x0C\x81\x10a\x17\xE8Wa\x17\xE8a$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\xFFWa\x17\xFFa$\x18V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x18\x1AWa\x18\x1Aa$\x18V[` \x02\x01Q\x83a\x18+\x83`\x05a%&V[`\x0C\x81\x10a\x18;Wa\x18;a$\x18V[` \x02\x01RP\x80a\x18K\x81a$\xF2V[\x91PPa\x16\x91V[Pa\x18\\a\x1D\xF4V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`\0\x80a\x18\x8C\x84a\x1B?V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x19\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x04wV[\x90P[\x92\x91PPV[`\0\x80[\x82\x15a\x19\rWa\x19(`\x01\x84a$\xB2V[\x90\x92\x16\x91\x80a\x196\x81a&!V[\x91PPa\x19\x17V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a\x19\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x04wV[\x81a\xFF\xFF\x16`\x01\x14\x15a\x19\xAEWP\x81a\x19\rV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x1A\x17W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a\x19\xFAWa\x19\xF7\x84\x84a\x14xV[\x93P[a\x1A\x04\x83\x84a\x14xV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a\x19\xCAV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x1AGWP` \x82\x01Q\x15[\x15a\x1AeWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a&D\x839\x81Q\x91R\x84` \x01Qa\x1A\x98\x91\x90a$.V[a\x1A\xB0\x90`\0\x80Q` a&D\x839\x81Q\x91Ra$\xB2V[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` a&D\x839\x81Q\x91R`\x03`\0\x80Q` a&D\x839\x81Q\x91R\x86`\0\x80Q` a&D\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a\x1B3\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a&D\x839\x81Q\x91Ra\x1C\xCCV[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15a\x1B\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x81Qa\x1B\xD6WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a\x1B\xECWa\x1B\xECa$\x18V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\x1C\xC3W\x84\x81\x81Q\x81\x10a\x1C\x1AWa\x1C\x1Aa$\x18V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\x1C\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x91\x81\x17\x91a\x1C\xBC\x81a$\xF2V[\x90Pa\x1B\xFFV[P\x90\x93\x92PPPV[`\0\x80a\x1C\xD7a\x1D\xF4V[a\x1C\xDFa\x1E\x12V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a\x140WP\x82a\x1DiW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04wV[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x1D\xC3a\x1E0V[\x81R` \x01a\x1D\xD0a\x1E0V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\x86Wa\x1E\x86a\x1ENV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\x86Wa\x1E\x86a\x1ENV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xD7Wa\x1E\xD7a\x1ENV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1E\xF1W`\0\x80\xFD[a\x1E\xF9a\x1EdV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x1F W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1FBWa\x1FBa\x1ENV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a\x1FYW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1A\x17W\x805\x83R` \x92\x83\x01\x92\x01a\x1F[V[`\0`\x80\x82\x84\x03\x12\x15a\x1F\x85W`\0\x80\xFD[a\x1F\x8Da\x1EdV[\x90Pa\x1F\x99\x83\x83a\x1F\x0FV[\x81Ra\x1F\xA8\x83`@\x84\x01a\x1F\x0FV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a\x1F\xCAW`\0\x80\xFD[\x845\x93Pa\x1F\xDB\x86` \x87\x01a\x1E\xDFV[\x92Pa\x1F\xEA\x86``\x87\x01a\x1FsV[\x91Pa\x1F\xF9\x86`\xE0\x87\x01a\x1E\xDFV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a \x16W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x19\nW`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xB8W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a SWa Sa\x1ENV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a nW`\0\x80\xFD[\x815` a \x83a ~\x83a :V[a\x1E\xAFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a \xA2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xC4Wa \xB7\x81a &V[\x83R\x91\x83\x01\x91\x83\x01a \xA6V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a \xE0W`\0\x80\xFD[\x815` a \xF0a ~\x83a :V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a!\x0FW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xC4Wa!%\x88\x82a\x1E\xDFV[\x83R\x91\x83\x01\x91`@\x01a!\x13V[`\0\x82`\x1F\x83\x01\x12a!DW`\0\x80\xFD[\x815` a!Ta ~\x83a :V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a!sW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xC4W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x96W`\0\x80\x81\xFD[a!\xA4\x89\x86\x83\x8B\x01\x01a ]V[\x84RP\x91\x83\x01\x91\x83\x01a!wV[`\0a\x01\x80\x82\x84\x03\x12\x15a!\xC5W`\0\x80\xFD[a!\xCDa\x1E\x8CV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\xE6W`\0\x80\xFD[a!\xF2\x85\x83\x86\x01a ]V[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\"\x08W`\0\x80\xFD[a\"\x14\x85\x83\x86\x01a \xCFV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\"-W`\0\x80\xFD[a\"9\x85\x83\x86\x01a \xCFV[`@\x84\x01Ra\"K\x85``\x86\x01a\x1FsV[``\x84\x01Ra\"]\x85`\xE0\x86\x01a\x1E\xDFV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a\"wW`\0\x80\xFD[a\"\x83\x85\x83\x86\x01a ]V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a\"\x9DW`\0\x80\xFD[a\"\xA9\x85\x83\x86\x01a ]V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a\"\xC3W`\0\x80\xFD[Pa\"\xD0\x84\x82\x85\x01a!3V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\"\xF4W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#\x12W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a#&W`\0\x80\xFD[\x815\x81\x81\x11\x15a#5W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a#GW`\0\x80\xFD[` \x83\x01\x96P\x94Pa#[`@\x89\x01a &V[\x93P``\x88\x015\x91P\x80\x82\x11\x15a#qW`\0\x80\xFD[Pa#~\x88\x82\x89\x01a!\xB2V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a#\xC4W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a#\x9FV[P\x94\x95\x94PPPPPV[`@\x81R`\0\x83Q`@\x80\x84\x01Ra#\xEA`\x80\x84\x01\x82a#\x8BV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01Ra$\x07\x82\x82a#\x8BV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a$KWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x82\x84\x03\x12\x15a$bW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\x8BW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a$\xC4Wa$\xC4a$\x9CV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a$\xDBW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0`\0\x19\x82\x14\x15a%\x06Wa%\x06a$\x9CV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a%\x1FW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15a%9Wa%9a$\x9CV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a%PW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a%{W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a%\xB2Wa%\xB2a$\x9CV[\x03\x93\x92PPPV[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R`\0`\x04\x82\x01\x83Q` \x80\x86\x01`\0[\x83\x81\x10\x15a%\xF5W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a%\xD9V[P\x92\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a&\x1CWa&\x1Ca$\x9CV[P\x02\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a&9Wa&9a$\x9CV[`\x01\x01\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \x9E \xC2\xE4?\x1C\xE2\xC8\xE2\xF9\xF75\xE2`<\xC57\xFC%\xA8\x82\xFE\xCE\xFA\x18?Q\xBEj\xCF\xDB\xBFdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSSIGNATURECHECKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0[W\x80cm\x14\xA9\x87\x14a\x017W\x80cn\xFBF6\x14a\x01^W\x80c\xB9\x8D\t\x08\x14a\x01\x7FW\x80c\xDF\\\xF7#\x14a\x01\x9CW`\0\x80\xFD[\x80c\x17\x1F\x1D[\x14a\0\x8DW\x80cAl~^\x14a\0\xBCW\x80c]\xF4YF\x14a\0\xD1W\x80ch0H5\x14a\x01\x10W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\x1F\xB3V[a\x01\xC3V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCFa\0\xCA6`\x04a \x04V[a\x03MV[\0[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01qa\x01l6`\x04a\"\xDCV[a\x04\xC7V[`@Qa\0\xB3\x92\x91\x90a#\xCFV[`\0Ta\x01\x8C\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xB3V[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x02\x0BWa\x02\x0Ba$\x18V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x020Wa\x020a$\x18V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x02LWa\x02La$\x18V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x02\xA9\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x02\xCC\x91\x90a$.V[\x90Pa\x03?a\x02\xE5a\x02\xDE\x88\x84a\x13\xE1V[\x86\x90a\x14xV[a\x02\xEDa\x15\x0CV[a\x035a\x03&\x85a\x03 `@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x13\xE1V[a\x03/\x8Ca\x15\xCCV[\x90a\x14xV[\x88b\x01\xD4\xC0a\x16\\V[\x90\x98\x90\x97P\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCF\x91\x90a$PV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x84a\x05>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04wV[`@\x83\x01QQ\x85\x14\x80\x15a\x05VWP`\xA0\x83\x01QQ\x85\x14[\x80\x15a\x05fWP`\xC0\x83\x01QQ\x85\x14[\x80\x15a\x05vWP`\xE0\x83\x01QQ\x85\x14[a\x05\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x82QQ` \x84\x01QQ\x14a\x06XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a&d\x839\x81Q\x91R\x90\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x06\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x01a\x04wV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R``\x80\x84R\x90\x83\x01R\x90\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x08Wa\x07\x08a\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x071W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07OWa\x07Oa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07xW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x85` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xACWa\x07\xACa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xD5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x86\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xF5Wa\x07\xF5a\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\x1EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81` \x01\x81\x90RP`\0a\x08\xF0\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\x9A\xA1e=\x92P`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xEB\x91\x90a$yV[a\x18\x80V[\x90P`\0[\x87` \x01QQ\x81\x10\x15a\x0B\x8BWa\t:\x88` \x01Q\x82\x81Q\x81\x10a\t\x1BWa\t\x1Ba$\x18V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83` \x01Q\x82\x81Q\x81\x10a\tPWa\tPa$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\n\x10W` \x83\x01Qa\tq`\x01\x83a$\xB2V[\x81Q\x81\x10a\t\x81Wa\t\x81a$\x18V[` \x02` \x01\x01Q`\0\x1C\x83` \x01Q\x82\x81Q\x81\x10a\t\xA2Wa\t\xA2a$\x18V[` \x02` \x01\x01Q`\0\x1C\x11a\n\x10W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x04wV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\nUWa\nUa$\x18V[` \x02` \x01\x01Q\x8B\x8B`\0\x01Q\x85\x81Q\x81\x10a\ntWa\nta$\x18V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xB1\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xF2\x91\x90a$\xC9V[`\x01`\x01`\xC0\x1B\x03\x16\x83`\0\x01Q\x82\x81Q\x81\x10a\x0B\x11Wa\x0B\x11a$\x18V[` \x02` \x01\x01\x81\x81RPPa\x0Bwa\x02\xDEa\x0BK\x84\x86`\0\x01Q\x85\x81Q\x81\x10a\x0B=Wa\x0B=a$\x18V[` \x02` \x01\x01Q\x16a\x19\x13V[\x8A` \x01Q\x84\x81Q\x81\x10a\x0BaWa\x0Baa$\x18V[` \x02` \x01\x01Qa\x19>\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80a\x0B\x83\x81a$\xF2V[\x91PPa\x08\xF5V[PPa\x0B\x96\x83a\x1A\"V[`\0\x80T\x91\x94P`\xFF\x90\x91\x16\x90\x81a\x0B\xAFW`\0a\x0C1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4H\xFE\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C1\x91\x90a%\rV[\x90P`\0[\x8A\x81\x10\x15a\x12\xB0W\x82\x15a\r\x92W\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a\x0C\x8DWa\x0C\x8Da$\x18V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x92\x015`\xF8\x1C`\x04\x83\x01RP`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xF1\x91\x90a%\rV[a\x0C\xFB\x91\x90a%&V[\x10\x15a\r\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x04wV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a\r\xD3Wa\r\xD3a$\x18V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a\r\xF7Wa\r\xF7a$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ESW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ew\x91\x90a%>V[`\x01`\x01`@\x1B\x03\x19\x16a\x0E\x9A\x8A`@\x01Q\x83\x81Q\x81\x10a\t\x1BWa\t\x1Ba$\x18V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x0F6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x04wV[a\x0Ff\x89`@\x01Q\x82\x81Q\x81\x10a\x0FOWa\x0FOa$\x18V[` \x02` \x01\x01Q\x87a\x14x\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a\x0F\xA9Wa\x0F\xA9a$\x18V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a\x0F\xCDWa\x0F\xCDa$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10M\x91\x90a%iV[\x85` \x01Q\x82\x81Q\x81\x10a\x10cWa\x10ca$\x18V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q\x82\x90\x81\x10a\x10\x8FWa\x10\x8Fa$\x18V[` \x02` \x01\x01Q\x85`\0\x01Q\x82\x81Q\x81\x10a\x10\xADWa\x10\xADa$\x18V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0\x80[\x8A` \x01QQ\x81\x10\x15a\x12\x9BWa\x11%\x86`\0\x01Q\x82\x81Q\x81\x10a\x10\xF7Wa\x10\xF7a$\x18V[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a\x11\x11Wa\x11\x11a$\x18V[`\x01\x92\x015`\xF8\x1C\x92\x90\x92\x1C\x81\x16\x14\x91\x90PV[\x15a\x12\x89W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a\x11kWa\x11ka$\x18V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a\x11\x8FWa\x11\x8Fa$\x18V[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a\x11\xADWa\x11\xADa$\x18V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x11\xC6Wa\x11\xC6a$\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12N\x91\x90a%iV[\x87Q\x80Q\x85\x90\x81\x10a\x12bWa\x12ba$\x18V[` \x02` \x01\x01\x81\x81Qa\x12v\x91\x90a%\x92V[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x90\x91\x01\x90[\x80a\x12\x93\x81a$\xF2V[\x91PPa\x10\xD1V[PP\x80\x80a\x12\xA8\x90a$\xF2V[\x91PPa\x0C6V[PPP`\0\x80a\x12\xCA\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x01\xC3V[\x91P\x91P\x81a\x13;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x80a\x13\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R`\0\x80Q` a&d\x839\x81Q\x91R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04wV[PP`\0\x87\x82` \x01Q`@Q` \x01a\x13\xB7\x92\x91\x90a%\xBAV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xFDa\x1DtV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x140Wa\x142V[\xFE[P\x80a\x14pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04wV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14\x94a\x1D\x92V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x140WP\x80a\x14pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04wV[a\x15\x14a\x1D\xB0V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a\x15\xFC`\0\x80Q` a&D\x839\x81Q\x91R\x86a$.V[\x90P[a\x16\x08\x81a\x1A\xBDV[\x90\x93P\x91P`\0\x80Q` a&D\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a\x16BW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a&D\x839\x81Q\x91R`\x01\x82\x08\x90Pa\x15\xFFV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a\x16\x8Ea\x1D\xD5V[`\0[`\x02\x81\x10\x15a\x18SW`\0a\x16\xA7\x82`\x06a&\x02V[\x90P\x84\x82`\x02\x81\x10a\x16\xBBWa\x16\xBBa$\x18V[` \x02\x01QQ\x83a\x16\xCD\x83`\0a%&V[`\x0C\x81\x10a\x16\xDDWa\x16\xDDa$\x18V[` \x02\x01R\x84\x82`\x02\x81\x10a\x16\xF4Wa\x16\xF4a$\x18V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x17\x0B\x91\x90a%&V[`\x0C\x81\x10a\x17\x1BWa\x17\x1Ba$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x172Wa\x172a$\x18V[` \x02\x01QQQ\x83a\x17E\x83`\x02a%&V[`\x0C\x81\x10a\x17UWa\x17Ua$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17lWa\x17la$\x18V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x17\x85\x83`\x03a%&V[`\x0C\x81\x10a\x17\x95Wa\x17\x95a$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\xACWa\x17\xACa$\x18V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x17\xC7Wa\x17\xC7a$\x18V[` \x02\x01Q\x83a\x17\xD8\x83`\x04a%&V[`\x0C\x81\x10a\x17\xE8Wa\x17\xE8a$\x18V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\xFFWa\x17\xFFa$\x18V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x18\x1AWa\x18\x1Aa$\x18V[` \x02\x01Q\x83a\x18+\x83`\x05a%&V[`\x0C\x81\x10a\x18;Wa\x18;a$\x18V[` \x02\x01RP\x80a\x18K\x81a$\xF2V[\x91PPa\x16\x91V[Pa\x18\\a\x1D\xF4V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`\0\x80a\x18\x8C\x84a\x1B?V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x19\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x04wV[\x90P[\x92\x91PPV[`\0\x80[\x82\x15a\x19\rWa\x19(`\x01\x84a$\xB2V[\x90\x92\x16\x91\x80a\x196\x81a&!V[\x91PPa\x19\x17V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a\x19\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x04wV[\x81a\xFF\xFF\x16`\x01\x14\x15a\x19\xAEWP\x81a\x19\rV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x1A\x17W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a\x19\xFAWa\x19\xF7\x84\x84a\x14xV[\x93P[a\x1A\x04\x83\x84a\x14xV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a\x19\xCAV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x1AGWP` \x82\x01Q\x15[\x15a\x1AeWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a&D\x839\x81Q\x91R\x84` \x01Qa\x1A\x98\x91\x90a$.V[a\x1A\xB0\x90`\0\x80Q` a&D\x839\x81Q\x91Ra$\xB2V[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` a&D\x839\x81Q\x91R`\x03`\0\x80Q` a&D\x839\x81Q\x91R\x86`\0\x80Q` a&D\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a\x1B3\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a&D\x839\x81Q\x91Ra\x1C\xCCV[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15a\x1B\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x81Qa\x1B\xD6WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a\x1B\xECWa\x1B\xECa$\x18V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\x1C\xC3W\x84\x81\x81Q\x81\x10a\x1C\x1AWa\x1C\x1Aa$\x18V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\x1C\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x04wV[\x91\x81\x17\x91a\x1C\xBC\x81a$\xF2V[\x90Pa\x1B\xFFV[P\x90\x93\x92PPPV[`\0\x80a\x1C\xD7a\x1D\xF4V[a\x1C\xDFa\x1E\x12V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a\x140WP\x82a\x1DiW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04wV[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x1D\xC3a\x1E0V[\x81R` \x01a\x1D\xD0a\x1E0V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\x86Wa\x1E\x86a\x1ENV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\x86Wa\x1E\x86a\x1ENV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xD7Wa\x1E\xD7a\x1ENV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1E\xF1W`\0\x80\xFD[a\x1E\xF9a\x1EdV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x1F W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1FBWa\x1FBa\x1ENV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a\x1FYW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1A\x17W\x805\x83R` \x92\x83\x01\x92\x01a\x1F[V[`\0`\x80\x82\x84\x03\x12\x15a\x1F\x85W`\0\x80\xFD[a\x1F\x8Da\x1EdV[\x90Pa\x1F\x99\x83\x83a\x1F\x0FV[\x81Ra\x1F\xA8\x83`@\x84\x01a\x1F\x0FV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a\x1F\xCAW`\0\x80\xFD[\x845\x93Pa\x1F\xDB\x86` \x87\x01a\x1E\xDFV[\x92Pa\x1F\xEA\x86``\x87\x01a\x1FsV[\x91Pa\x1F\xF9\x86`\xE0\x87\x01a\x1E\xDFV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a \x16W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x19\nW`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xB8W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a SWa Sa\x1ENV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a nW`\0\x80\xFD[\x815` a \x83a ~\x83a :V[a\x1E\xAFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a \xA2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xC4Wa \xB7\x81a &V[\x83R\x91\x83\x01\x91\x83\x01a \xA6V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a \xE0W`\0\x80\xFD[\x815` a \xF0a ~\x83a :V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a!\x0FW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xC4Wa!%\x88\x82a\x1E\xDFV[\x83R\x91\x83\x01\x91`@\x01a!\x13V[`\0\x82`\x1F\x83\x01\x12a!DW`\0\x80\xFD[\x815` a!Ta ~\x83a :V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a!sW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xC4W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x96W`\0\x80\x81\xFD[a!\xA4\x89\x86\x83\x8B\x01\x01a ]V[\x84RP\x91\x83\x01\x91\x83\x01a!wV[`\0a\x01\x80\x82\x84\x03\x12\x15a!\xC5W`\0\x80\xFD[a!\xCDa\x1E\x8CV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\xE6W`\0\x80\xFD[a!\xF2\x85\x83\x86\x01a ]V[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\"\x08W`\0\x80\xFD[a\"\x14\x85\x83\x86\x01a \xCFV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\"-W`\0\x80\xFD[a\"9\x85\x83\x86\x01a \xCFV[`@\x84\x01Ra\"K\x85``\x86\x01a\x1FsV[``\x84\x01Ra\"]\x85`\xE0\x86\x01a\x1E\xDFV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a\"wW`\0\x80\xFD[a\"\x83\x85\x83\x86\x01a ]V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a\"\x9DW`\0\x80\xFD[a\"\xA9\x85\x83\x86\x01a ]V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a\"\xC3W`\0\x80\xFD[Pa\"\xD0\x84\x82\x85\x01a!3V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\"\xF4W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#\x12W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a#&W`\0\x80\xFD[\x815\x81\x81\x11\x15a#5W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a#GW`\0\x80\xFD[` \x83\x01\x96P\x94Pa#[`@\x89\x01a &V[\x93P``\x88\x015\x91P\x80\x82\x11\x15a#qW`\0\x80\xFD[Pa#~\x88\x82\x89\x01a!\xB2V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a#\xC4W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a#\x9FV[P\x94\x95\x94PPPPPV[`@\x81R`\0\x83Q`@\x80\x84\x01Ra#\xEA`\x80\x84\x01\x82a#\x8BV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01Ra$\x07\x82\x82a#\x8BV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a$KWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x82\x84\x03\x12\x15a$bW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\x8BW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a$\xC4Wa$\xC4a$\x9CV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a$\xDBW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0`\0\x19\x82\x14\x15a%\x06Wa%\x06a$\x9CV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a%\x1FW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15a%9Wa%9a$\x9CV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a%PW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a%{W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x19\nW`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a%\xB2Wa%\xB2a$\x9CV[\x03\x93\x92PPPV[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R`\0`\x04\x82\x01\x83Q` \x80\x86\x01`\0[\x83\x81\x10\x15a%\xF5W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a%\xD9V[P\x92\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a&\x1CWa&\x1Ca$\x9CV[P\x02\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a&9Wa&9a$\x9CV[`\x01\x01\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \x9E \xC2\xE4?\x1C\xE2\xC8\xE2\xF9\xF75\xE2`<\xC57\xFC%\xA8\x82\xFE\xCE\xFA\x18?Q\xBEj\xCF\xDB\xBFdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSSIGNATURECHECKER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BLSSignatureChecker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSSignatureChecker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSSignatureChecker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSSignatureChecker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSSignatureChecker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSSignatureChecker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSSignatureChecker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLSSIGNATURECHECKER_ABI.clone(),
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
                BLSSIGNATURECHECKER_ABI.clone(),
                BLSSIGNATURECHECKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `blsApkRegistry` (0x5df45946) function
        pub fn bls_apk_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([93, 244, 89, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x6efb4636) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
            reference_block_number: u32,
            params: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (QuorumStakeTotals, [u8; 32]),
        > {
            self.0
                .method_hash(
                    [110, 251, 70, 54],
                    (msg_hash, quorum_numbers, reference_block_number, params),
                )
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
        ///Calls the contract's `setStaleStakesForbidden` (0x416c7e5e) function
        pub fn set_stale_stakes_forbidden(
            &self,
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 108, 126, 94], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeRegistry` (0x68304835) function
        pub fn stake_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([104, 48, 72, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staleStakesForbidden` (0xb98d0908) function
        pub fn stale_stakes_forbidden(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([185, 141, 9, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trySignatureAndApkVerification` (0x171f1d5b) function
        pub fn try_signature_and_apk_verification(
            &self,
            msg_hash: [u8; 32],
            apk: G1Point,
            apk_g2: G2Point,
            sigma: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, bool)> {
            self.0
                .method_hash([23, 31, 29, 91], (msg_hash, apk, apk_g2, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `StaleStakesForbiddenUpdate` event
        pub fn stale_stakes_forbidden_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StaleStakesForbiddenUpdateFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StaleStakesForbiddenUpdateFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BLSSignatureChecker<M> {
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
        name = "StaleStakesForbiddenUpdate",
        abi = "StaleStakesForbiddenUpdate(bool)"
    )]
    pub struct StaleStakesForbiddenUpdateFilter {
        pub value: bool,
    }
    ///Container type for all input parameters for the `blsApkRegistry` function with signature `blsApkRegistry()` and selector `0x5df45946`
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
    #[ethcall(name = "blsApkRegistry", abi = "blsApkRegistry()")]
    pub struct BlsApkRegistryCall;
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`
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
        name = "checkSignatures",
        abi = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub reference_block_number: u32,
        pub params: NonSignerStakesAndSignature,
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
    ///Container type for all input parameters for the `setStaleStakesForbidden` function with signature `setStaleStakesForbidden(bool)` and selector `0x416c7e5e`
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
    #[ethcall(name = "setStaleStakesForbidden", abi = "setStaleStakesForbidden(bool)")]
    pub struct SetStaleStakesForbiddenCall {
        pub value: bool,
    }
    ///Container type for all input parameters for the `stakeRegistry` function with signature `stakeRegistry()` and selector `0x68304835`
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
    #[ethcall(name = "stakeRegistry", abi = "stakeRegistry()")]
    pub struct StakeRegistryCall;
    ///Container type for all input parameters for the `staleStakesForbidden` function with signature `staleStakesForbidden()` and selector `0xb98d0908`
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
    #[ethcall(name = "staleStakesForbidden", abi = "staleStakesForbidden()")]
    pub struct StaleStakesForbiddenCall;
    ///Container type for all input parameters for the `trySignatureAndApkVerification` function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`
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
        name = "trySignatureAndApkVerification",
        abi = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))"
    )]
    pub struct TrySignatureAndApkVerificationCall {
        pub msg_hash: [u8; 32],
        pub apk: G1Point,
        pub apk_g2: G2Point,
        pub sigma: G1Point,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BLSSignatureCheckerCalls {
        BlsApkRegistry(BlsApkRegistryCall),
        CheckSignatures(CheckSignaturesCall),
        Delegation(DelegationCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        SetStaleStakesForbidden(SetStaleStakesForbiddenCall),
        StakeRegistry(StakeRegistryCall),
        StaleStakesForbidden(StaleStakesForbiddenCall),
        TrySignatureAndApkVerification(TrySignatureAndApkVerificationCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSSignatureCheckerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BlsApkRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlsApkRegistry(decoded));
            }
            if let Ok(decoded) = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) = <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) = <SetStaleStakesForbiddenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStaleStakesForbidden(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) = <StaleStakesForbiddenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StaleStakesForbidden(decoded));
            }
            if let Ok(decoded) = <TrySignatureAndApkVerificationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrySignatureAndApkVerification(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSSignatureCheckerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BlsApkRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStaleStakesForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StaleStakesForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrySignatureAndApkVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BLSSignatureCheckerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BlsApkRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStaleStakesForbidden(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaleStakesForbidden(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TrySignatureAndApkVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BlsApkRegistryCall> for BLSSignatureCheckerCalls {
        fn from(value: BlsApkRegistryCall) -> Self {
            Self::BlsApkRegistry(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for BLSSignatureCheckerCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for BLSSignatureCheckerCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for BLSSignatureCheckerCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<SetStaleStakesForbiddenCall>
    for BLSSignatureCheckerCalls {
        fn from(value: SetStaleStakesForbiddenCall) -> Self {
            Self::SetStaleStakesForbidden(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for BLSSignatureCheckerCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<StaleStakesForbiddenCall> for BLSSignatureCheckerCalls {
        fn from(value: StaleStakesForbiddenCall) -> Self {
            Self::StaleStakesForbidden(value)
        }
    }
    impl ::core::convert::From<TrySignatureAndApkVerificationCall>
    for BLSSignatureCheckerCalls {
        fn from(value: TrySignatureAndApkVerificationCall) -> Self {
            Self::TrySignatureAndApkVerification(value)
        }
    }
    ///Container type for all return fields from the `blsApkRegistry` function with signature `blsApkRegistry()` and selector `0x5df45946`
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
    pub struct BlsApkRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`
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
    pub struct CheckSignaturesReturn(pub QuorumStakeTotals, pub [u8; 32]);
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
    ///Container type for all return fields from the `stakeRegistry` function with signature `stakeRegistry()` and selector `0x68304835`
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
    pub struct StakeRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `staleStakesForbidden` function with signature `staleStakesForbidden()` and selector `0xb98d0908`
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
    pub struct StaleStakesForbiddenReturn(pub bool);
    ///Container type for all return fields from the `trySignatureAndApkVerification` function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`
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
    pub struct TrySignatureAndApkVerificationReturn {
        pub pairing_successful: bool,
        pub siganature_is_valid: bool,
    }
}
