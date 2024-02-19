pub use operator_state_retriever::*;
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
pub mod operator_state_retriever {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getCheckSignaturesIndices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCheckSignaturesIndices",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerOperatorIds",
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
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
                                            "struct OperatorStateRetriever.CheckSignaturesIndices",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.Operator[][]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.Operator[][]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OPERATORSTATERETRIEVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x17\"\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c5c\xB0\xD1\x14a\0FW\x80cOs\x9Ft\x14a\0oW\x80c\xCE\xFD\xC1\xD4\x14a\0\x8FW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x0F\x7FV[a\0\xB0V[`@Qa\0f\x91\x90a\x10\xDAV[`@Q\x80\x91\x03\x90\xF3[a\0\x82a\0}6`\x04a\x11?V[a\x05FV[`@Qa\0f\x91\x90a\x12BV[a\0\xA2a\0\x9D6`\x04a\x12\xFDV[a\x0CpV[`@Qa\0f\x92\x91\x90a\x13?V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x16\x91\x90a\x13`V[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01|\x91\x90a\x13`V[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE2\x91\x90a\x13`V[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xFFWa\x01\xFFa\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x022W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\x1DW\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x05:W`\0\x88\x82\x81Q\x81\x10a\x02UWa\x02Ua\x13}V[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xDE\x91\x90\x81\x01\x90a\x13\xB6V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03DW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x03\x17W\x90P[P\x84\x84\x81Q\x81\x10a\x03WWa\x03Wa\x13}V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x05$W`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x03\x9AWa\x03\x9Aa\x13}V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xC0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x01\x91\x90a\x13`V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x04!Wa\x04!a\x13}V[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x04OWa\x04Oa\x13}V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCF\x91\x90a\x14KV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x04\xEDWa\x04\xEDa\x13}V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x05\x06Wa\x05\x06a\x13}V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x05\x1C\x90a\x14\x8AV[\x91PPa\x03eV[PPP\x80\x80a\x052\x90a\x14\x8AV[\x91PPa\x028V[P\x97\x96PPPPPPPV[a\x05q`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x13`V[\x90Pa\x06\x02`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x062\x90\x8B\x90\x89\x90\x89\x90`\x04\x01a\x14\xA5V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06w\x91\x90\x81\x01\x90a\x14\xEFV[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x06\xA9\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x15\xA6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\xEE\x91\x90\x81\x01\x90a\x14\xEFV[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x0BWa\x07\x0Ba\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07>W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07)W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x0B\x81W`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07lWa\x07la\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x07\xAFWa\x07\xAFa\x13}V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\n\x81W`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x07\xE8Wa\x07\xE8a\x13}V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x08\x06Wa\x08\x06a\x13}V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08C\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x84\x91\x90a\x15\xCFV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\t,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\tAWa\tAa\x13}V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\nnW\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\t\x83Wa\t\x83a\x13}V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\t\x9FWa\t\x9Fa\x13}V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x19\x91\x90a\x15\xF8V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\n2Wa\n2a\x13}V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\nKWa\nKa\x13}V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\nj\x81a\x14\x8AV[\x93PP[P\x80a\ny\x81a\x14\x8AV[\x91PPa\x07\xBDV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x9CWa\n\x9Ca\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xC5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0BFW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\n\xECWa\n\xECa\x13}V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x0B\x05Wa\x0B\x05a\x13}V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0B\x1FWa\x0B\x1Fa\x13}V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0B>\x81a\x14\x8AV[\x91PPa\n\xCBV[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x0BaWa\x0Baa\x13}V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x0By\x90a\x16\x15V[\x91PPa\x07GV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE6\x91\x90a\x13`V[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x0C\x19\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01a\x165V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C^\x91\x90\x81\x01\x90a\x14\xEFV[` \x83\x01RP\x98\x97PPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\x0C\xABWa\x0C\xABa\x13}V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a\x0C\xE7\x90\x88\x90\x86\x90`\x04\x01a\x16_V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r,\x91\x90\x81\x01\x90a\x14\xEFV[`\0\x81Q\x81\x10a\r>Wa\r>a\x13}V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a\x15\xCFV[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a\r\xE4\x82a\x0E\x02V[\x90P\x81a\r\xF2\x8A\x83\x8Aa\0\xB0V[\x95P\x95PPPPP\x93P\x93\x91PPV[```\0\x80a\x0E\x10\x84a\x0E\xCEV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E+Wa\x0E+a\x0F\x17V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0EUW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x0EmWPa\x01\0\x81\x10[\x15a\x0E\xC4W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x0E\xB4W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x0E\x96Wa\x0E\x96a\x13}V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x0E\xBD\x81a\x14\x8AV[\x90Pa\x0E\\V[P\x90\x94\x93PPPPV[`\0\x80[\x82\x15a\x0E\xF9Wa\x0E\xE3`\x01\x84a\x16\xB3V[\x90\x92\x16\x91\x80a\x0E\xF1\x81a\x16\xCAV[\x91PPa\x0E\xD2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x14W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0FUWa\x0FUa\x0F\x17V[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x14W`\0\x80\xFD[\x805a\x0Fz\x81a\x0F]V[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x94W`\0\x80\xFD[\x835a\x0F\x9F\x81a\x0E\xFFV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0F\xBCW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0F\xD0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\xE2Wa\x0F\xE2a\x0F\x17V[a\x0F\xF4`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x0F-V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x10\nW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x10-`@\x85\x01a\x0FoV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x10\xCCW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a\x10\xB7W\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01a\x10sV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a\x10UV[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x10\xED` \x83\x01\x84a\x106V[\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x11\x06W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x1DW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x118W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x11XW`\0\x80\xFD[\x865a\x11c\x81a\x0E\xFFV[\x95P` \x87\x015a\x11s\x81a\x0F]V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\x8FW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x11\xA3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\xB2W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x11\xC4W`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP``\x89\x015\x91P\x80\x82\x11\x15a\x11\xE2W`\0\x80\xFD[Pa\x11\xEF\x89\x82\x8A\x01a\x10\xF4V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x127W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x12\x15V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01Ra\x12^`\xA0\x85\x01\x82a\x12\x01V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01Ra\x12{\x83\x83a\x12\x01V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01Ra\x12\x98\x83\x83a\x12\x01V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15a\x12\xEFW\x84\x87\x83\x03\x01\x84Ra\x12\xDD\x82\x87Qa\x12\x01V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01a\x12\xC3V[P\x99\x98PPPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x12W`\0\x80\xFD[\x835a\x13\x1D\x81a\x0E\xFFV[\x92P` \x84\x015\x91P`@\x84\x015a\x134\x81a\x0F]V[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0a\x13X`@\x83\x01\x84a\x106V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x13rW`\0\x80\xFD[\x81Qa\x10\xED\x81a\x0E\xFFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x13\xACWa\x13\xACa\x0F\x17V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15a\x13\xC9W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xDFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x13\xF0W`\0\x80\xFD[\x80Qa\x14\x03a\x13\xFE\x82a\x13\x93V[a\x0F-V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x14\"W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x14@W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x14'V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x14]W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x10\xEDW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x14\x9EWa\x14\x9Ea\x14tV[P`\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x14\xD2W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x15\x02W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x18W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x15)W`\0\x80\xFD[\x80Qa\x157a\x13\xFE\x82a\x13\x93V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x15VW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x14@W\x83Qa\x15n\x81a\x0F]V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15[V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x15\xC6`@\x83\x01\x84\x86a\x15}V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x15\xE1W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x10\xEDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\nW`\0\x80\xFD[\x81Qa\x10\xED\x81a\x0F]V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x16,Wa\x16,a\x14tV[`\x01\x01\x92\x91PPV[`@\x81R`\0a\x16I`@\x83\x01\x85\x87a\x15}V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15a\x16\xA6W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x16\x8AV[P\x90\x97\x96PPPPPPPV[`\0\x82\x82\x10\x15a\x16\xC5Wa\x16\xC5a\x14tV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x16\xE2Wa\x16\xE2a\x14tV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 =\xA7\xD2\x02\xD9\xE7Du\x98\x8A\xE6\x99X\xA7y\xE5rw\xF7\xDF\xFD\x91\xAD\xC8|\x19\xDA`hS\xE9\xACdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static OPERATORSTATERETRIEVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c5c\xB0\xD1\x14a\0FW\x80cOs\x9Ft\x14a\0oW\x80c\xCE\xFD\xC1\xD4\x14a\0\x8FW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x0F\x7FV[a\0\xB0V[`@Qa\0f\x91\x90a\x10\xDAV[`@Q\x80\x91\x03\x90\xF3[a\0\x82a\0}6`\x04a\x11?V[a\x05FV[`@Qa\0f\x91\x90a\x12BV[a\0\xA2a\0\x9D6`\x04a\x12\xFDV[a\x0CpV[`@Qa\0f\x92\x91\x90a\x13?V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x16\x91\x90a\x13`V[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01|\x91\x90a\x13`V[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE2\x91\x90a\x13`V[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xFFWa\x01\xFFa\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x022W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\x1DW\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x05:W`\0\x88\x82\x81Q\x81\x10a\x02UWa\x02Ua\x13}V[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xDE\x91\x90\x81\x01\x90a\x13\xB6V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03DW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x03\x17W\x90P[P\x84\x84\x81Q\x81\x10a\x03WWa\x03Wa\x13}V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x05$W`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x03\x9AWa\x03\x9Aa\x13}V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xC0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x01\x91\x90a\x13`V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x04!Wa\x04!a\x13}V[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x04OWa\x04Oa\x13}V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCF\x91\x90a\x14KV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x04\xEDWa\x04\xEDa\x13}V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x05\x06Wa\x05\x06a\x13}V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x05\x1C\x90a\x14\x8AV[\x91PPa\x03eV[PPP\x80\x80a\x052\x90a\x14\x8AV[\x91PPa\x028V[P\x97\x96PPPPPPPV[a\x05q`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x13`V[\x90Pa\x06\x02`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x062\x90\x8B\x90\x89\x90\x89\x90`\x04\x01a\x14\xA5V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06w\x91\x90\x81\x01\x90a\x14\xEFV[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x06\xA9\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x15\xA6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\xEE\x91\x90\x81\x01\x90a\x14\xEFV[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x0BWa\x07\x0Ba\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07>W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07)W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x0B\x81W`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07lWa\x07la\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x07\xAFWa\x07\xAFa\x13}V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\n\x81W`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x07\xE8Wa\x07\xE8a\x13}V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x08\x06Wa\x08\x06a\x13}V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08C\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x84\x91\x90a\x15\xCFV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\t,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\tAWa\tAa\x13}V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\nnW\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\t\x83Wa\t\x83a\x13}V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\t\x9FWa\t\x9Fa\x13}V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x19\x91\x90a\x15\xF8V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\n2Wa\n2a\x13}V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\nKWa\nKa\x13}V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\nj\x81a\x14\x8AV[\x93PP[P\x80a\ny\x81a\x14\x8AV[\x91PPa\x07\xBDV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x9CWa\n\x9Ca\x0F\x17V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xC5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0BFW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\n\xECWa\n\xECa\x13}V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x0B\x05Wa\x0B\x05a\x13}V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0B\x1FWa\x0B\x1Fa\x13}V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0B>\x81a\x14\x8AV[\x91PPa\n\xCBV[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x0BaWa\x0Baa\x13}V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x0By\x90a\x16\x15V[\x91PPa\x07GV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE6\x91\x90a\x13`V[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x0C\x19\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01a\x165V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C^\x91\x90\x81\x01\x90a\x14\xEFV[` \x83\x01RP\x98\x97PPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\x0C\xABWa\x0C\xABa\x13}V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a\x0C\xE7\x90\x88\x90\x86\x90`\x04\x01a\x16_V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r,\x91\x90\x81\x01\x90a\x14\xEFV[`\0\x81Q\x81\x10a\r>Wa\r>a\x13}V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a\x15\xCFV[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a\r\xE4\x82a\x0E\x02V[\x90P\x81a\r\xF2\x8A\x83\x8Aa\0\xB0V[\x95P\x95PPPPP\x93P\x93\x91PPV[```\0\x80a\x0E\x10\x84a\x0E\xCEV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E+Wa\x0E+a\x0F\x17V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0EUW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x0EmWPa\x01\0\x81\x10[\x15a\x0E\xC4W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x0E\xB4W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x0E\x96Wa\x0E\x96a\x13}V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x0E\xBD\x81a\x14\x8AV[\x90Pa\x0E\\V[P\x90\x94\x93PPPPV[`\0\x80[\x82\x15a\x0E\xF9Wa\x0E\xE3`\x01\x84a\x16\xB3V[\x90\x92\x16\x91\x80a\x0E\xF1\x81a\x16\xCAV[\x91PPa\x0E\xD2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x14W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0FUWa\x0FUa\x0F\x17V[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x14W`\0\x80\xFD[\x805a\x0Fz\x81a\x0F]V[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x94W`\0\x80\xFD[\x835a\x0F\x9F\x81a\x0E\xFFV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0F\xBCW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0F\xD0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\xE2Wa\x0F\xE2a\x0F\x17V[a\x0F\xF4`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x0F-V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x10\nW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x10-`@\x85\x01a\x0FoV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x10\xCCW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a\x10\xB7W\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01a\x10sV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a\x10UV[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x10\xED` \x83\x01\x84a\x106V[\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x11\x06W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x1DW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x118W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x11XW`\0\x80\xFD[\x865a\x11c\x81a\x0E\xFFV[\x95P` \x87\x015a\x11s\x81a\x0F]V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\x8FW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x11\xA3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\xB2W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x11\xC4W`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP``\x89\x015\x91P\x80\x82\x11\x15a\x11\xE2W`\0\x80\xFD[Pa\x11\xEF\x89\x82\x8A\x01a\x10\xF4V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x127W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x12\x15V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01Ra\x12^`\xA0\x85\x01\x82a\x12\x01V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01Ra\x12{\x83\x83a\x12\x01V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01Ra\x12\x98\x83\x83a\x12\x01V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15a\x12\xEFW\x84\x87\x83\x03\x01\x84Ra\x12\xDD\x82\x87Qa\x12\x01V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01a\x12\xC3V[P\x99\x98PPPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x12W`\0\x80\xFD[\x835a\x13\x1D\x81a\x0E\xFFV[\x92P` \x84\x015\x91P`@\x84\x015a\x134\x81a\x0F]V[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0a\x13X`@\x83\x01\x84a\x106V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x13rW`\0\x80\xFD[\x81Qa\x10\xED\x81a\x0E\xFFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x13\xACWa\x13\xACa\x0F\x17V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15a\x13\xC9W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xDFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x13\xF0W`\0\x80\xFD[\x80Qa\x14\x03a\x13\xFE\x82a\x13\x93V[a\x0F-V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x14\"W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x14@W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x14'V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x14]W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x10\xEDW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x14\x9EWa\x14\x9Ea\x14tV[P`\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x14\xD2W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x15\x02W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x18W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x15)W`\0\x80\xFD[\x80Qa\x157a\x13\xFE\x82a\x13\x93V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x15VW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x14@W\x83Qa\x15n\x81a\x0F]V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15[V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x15\xC6`@\x83\x01\x84\x86a\x15}V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x15\xE1W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x10\xEDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\nW`\0\x80\xFD[\x81Qa\x10\xED\x81a\x0F]V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x16,Wa\x16,a\x14tV[`\x01\x01\x92\x91PPV[`@\x81R`\0a\x16I`@\x83\x01\x85\x87a\x15}V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15a\x16\xA6W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x16\x8AV[P\x90\x97\x96PPPPPPPV[`\0\x82\x82\x10\x15a\x16\xC5Wa\x16\xC5a\x14tV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x16\xE2Wa\x16\xE2a\x14tV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 =\xA7\xD2\x02\xD9\xE7Du\x98\x8A\xE6\x99X\xA7y\xE5rw\xF7\xDF\xFD\x91\xAD\xC8|\x19\xDA`hS\xE9\xACdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static OPERATORSTATERETRIEVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OperatorStateRetriever<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OperatorStateRetriever<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OperatorStateRetriever<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OperatorStateRetriever<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OperatorStateRetriever<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OperatorStateRetriever))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OperatorStateRetriever<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPERATORSTATERETRIEVER_ABI.clone(),
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
                OPERATORSTATERETRIEVER_ABI.clone(),
                OPERATORSTATERETRIEVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getCheckSignaturesIndices` (0x4f739f74) function
        pub fn get_check_signatures_indices(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            reference_block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
            non_signer_operator_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, CheckSignaturesIndices> {
            self.0
                .method_hash(
                    [79, 115, 159, 116],
                    (
                        registry_coordinator,
                        reference_block_number,
                        quorum_numbers,
                        non_signer_operator_ids,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0x3563b0d1) function
        pub fn get_operator_state(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<Operator>>,
        > {
            self.0
                .method_hash(
                    [53, 99, 176, 209],
                    (registry_coordinator, quorum_numbers, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0xcefdc1d4) function
        pub fn get_operator_state_with_registry_coordinator_and_operator_id(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<::std::vec::Vec<Operator>>),
        > {
            self.0
                .method_hash(
                    [206, 253, 193, 212],
                    (registry_coordinator, operator_id, block_number),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OperatorStateRetriever<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getCheckSignaturesIndices` function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`
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
        name = "getCheckSignaturesIndices",
        abi = "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])"
    )]
    pub struct GetCheckSignaturesIndicesCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub reference_block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub non_signer_operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `getOperatorState` function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`
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
    #[ethcall(name = "getOperatorState", abi = "getOperatorState(address,bytes,uint32)")]
    pub struct GetOperatorStateCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getOperatorState` function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`
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
        name = "getOperatorState",
        abi = "getOperatorState(address,bytes32,uint32)"
    )]
    pub struct GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub block_number: u32,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OperatorStateRetrieverCalls {
        GetCheckSignaturesIndices(GetCheckSignaturesIndicesCall),
        GetOperatorState(GetOperatorStateCall),
        GetOperatorStateWithRegistryCoordinatorAndOperatorId(
            GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ),
    }
    impl ::ethers::core::abi::AbiDecode for OperatorStateRetrieverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCheckSignaturesIndicesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCheckSignaturesIndices(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorState(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OperatorStateRetrieverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCheckSignaturesIndices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OperatorStateRetrieverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCheckSignaturesIndices(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetCheckSignaturesIndicesCall>
    for OperatorStateRetrieverCalls {
        fn from(value: GetCheckSignaturesIndicesCall) -> Self {
            Self::GetCheckSignaturesIndices(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateCall> for OperatorStateRetrieverCalls {
        fn from(value: GetOperatorStateCall) -> Self {
            Self::GetOperatorState(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall>
    for OperatorStateRetrieverCalls {
        fn from(
            value: GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ) -> Self {
            Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(value)
        }
    }
    ///Container type for all return fields from the `getCheckSignaturesIndices` function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`
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
    pub struct GetCheckSignaturesIndicesReturn(pub CheckSignaturesIndices);
    ///Container type for all return fields from the `getOperatorState` function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`
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
    pub struct GetOperatorStateReturn(pub ::std::vec::Vec<::std::vec::Vec<Operator>>);
    ///Container type for all return fields from the `getOperatorState` function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`
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
    pub struct GetOperatorStateWithRegistryCoordinatorAndOperatorIdReturn(
        pub ::ethers::core::types::U256,
        pub ::std::vec::Vec<::std::vec::Vec<Operator>>,
    );
    ///`CheckSignaturesIndices(uint32[],uint32[],uint32[],uint32[][])`
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
    pub struct CheckSignaturesIndices {
        pub non_signer_quorum_bitmap_indices: ::std::vec::Vec<u32>,
        pub quorum_apk_indices: ::std::vec::Vec<u32>,
        pub total_stake_indices: ::std::vec::Vec<u32>,
        pub non_signer_stake_indices: ::std::vec::Vec<::std::vec::Vec<u32>>,
    }
    ///`Operator(address,bytes32,uint96)`
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
    pub struct Operator {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub stake: u128,
    }
}
