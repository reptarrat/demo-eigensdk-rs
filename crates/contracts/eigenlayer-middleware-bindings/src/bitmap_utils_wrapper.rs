pub use bitmap_utils_wrapper::*;
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
pub mod bitmap_utils_wrapper {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bitmapToBytesArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bitmapToBytesArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bitmap"),
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
                                    name: ::std::borrow::ToOwned::to_owned("bytesArray"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("countNumOnes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("countNumOnes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isArrayStrictlyAscendingOrdered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isArrayStrictlyAscendingOrdered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bytesArray"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("isEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isEmpty"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bitmap"),
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
                    ::std::borrow::ToOwned::to_owned("isSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bitmap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "numberToCheckForInclusion",
                                    ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSubsetOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSubsetOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                    ::std::borrow::ToOwned::to_owned("minus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("noBitsInCommon"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("noBitsInCommon"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                    ::std::borrow::ToOwned::to_owned("orderedBytesArrayToBitmap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "orderedBytesArrayToBitmap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orderedBytesArray"),
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
                    ::std::borrow::ToOwned::to_owned("plus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("plus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bitmap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bit"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static BITMAPUTILSWRAPPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\xD4\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80cls\xBD\x87\x11a\0qW\x80cls\xBD\x87\x14a\x010W\x80cv7\x0F\x1F\x14a\x01CW\x80c\xA8\xE3\xEA\xBB\x14a\x01iW\x80c\xDDTq\x85\x14a\x01|W\x80c\xF4\xF3\xBD\xC1\x14a\x01\x9CW\x80c\xF9\x0C\xFE\xEF\x14a\x01\xAFW`\0\x80\xFD[\x80c\x1F\xF4\xAD\xBA\x14a\0\xAEW\x80c \xE8\x84\x03\x14a\0\xD6W\x80cN\xE2\x90\x90\x14a\0\xF7W\x80cb\xE2\xEF3\x14a\x01\nW\x80cf\t\x8DO\x14a\x01\x1DW[`\0\x80\xFD[a\0\xC1a\0\xBC6`\x04a\x05\xD0V[a\x01\xC2V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xE9a\0\xE46`\x04a\x06\x06V[a\x01\xDAV[`@Q\x90\x81R` \x01a\0\xCDV[a\0\xE9a\x01\x056`\x04a\x05\xD0V[a\x02\x1BV[a\0\xC1a\x01\x186`\x04a\x06xV[a\x02+V[a\0\xE9a\x01+6`\x04a\x06xV[a\x026V[a\0\xC1a\x01>6`\x04a\x06\x06V[a\x02@V[a\x01Va\x01Q6`\x04a\x06\x9AV[a\x02LV[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xCDV[a\0\xC1a\x01w6`\x04a\x06xV[a\x02WV[a\x01\x8Fa\x01\x8A6`\x04a\x06\x9AV[a\x02cV[`@Qa\0\xCD\x91\x90a\x06\xB3V[a\0\xE9a\x01\xAA6`\x04a\x06xV[a\x02nV[a\0\xC1a\x01\xBD6`\x04a\x06\x9AV[a\x02yV[`\0`\x01`\xFF\x83\x16\x84\x90\x1C\x81\x16\x14[\x90P[\x92\x91PPV[`\0a\x01\xD1\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x02\x82\x92PPPV[`\0`\x01`\xFF\x83\x16\x1B\x83\x17a\x01\xD1V[`\0\x81\x83\x16\x15a\x01\xD1V[`\0\x81\x83\x17a\x01\xD1V[`\0a\x01\xD1\x83\x83a\x04\x14V[`\0a\x01\xD4\x82a\x04\xD8V[`\0\x81\x83\x16\x83\x14a\x01\xD1V[``a\x01\xD4\x82a\x05\x03V[`\0\x81\x19\x83\x16a\x01\xD1V[`\0\x81\x15a\x01\xD4V[`\0a\x01\0\x82Q\x11\x15a\x03\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[\x81Qa\x03\x1EWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a\x034Wa\x034a\x07\x08V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\x04\x0BW\x84\x81\x81Q\x81\x10a\x03bWa\x03ba\x07\x08V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\x03\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x03\x07V[\x91\x81\x17\x91a\x04\x04\x81a\x074V[\x90Pa\x03GV[P\x90\x93\x92PPPV[`\0a\x01\0\x82\x11\x15a\x04(WP`\0a\x01\xD4V[\x81a\x045WP`\x01a\x01\xD4V[`\0\x83\x83`\0\x81\x81\x10a\x04JWa\x04Ja\x07\x08V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x91P`\x01\x90P[\x83\x81\x10\x15a\x04\xCDW`\xF8\x82\x90\x1C\x85\x85\x83\x81\x81\x10a\x04~Wa\x04~a\x07\x08V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x11a\x04\x9EW`\0\x92PPPa\x01\xD4V[\x84\x84\x82\x81\x81\x10a\x04\xB0Wa\x04\xB0a\x07\x08V[\x90P\x015`\xF8\x1C`\xF8\x1B\x91P\x80a\x04\xC6\x90a\x074V[\x90Pa\x04_V[P`\x01\x94\x93PPPPV[`\0\x80[\x82\x15a\x01\xD4Wa\x04\xED`\x01\x84a\x07OV[\x90\x92\x16\x91\x80a\x04\xFB\x81a\x07fV[\x91PPa\x04\xDCV[```\0\x80a\x05\x11\x84a\x04\xD8V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05-Wa\x05-a\x07\x88V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05WW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x05oWPa\x01\0\x81\x10[\x15a\x05\xC6W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x05\xB6W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x05\x98Wa\x05\x98a\x07\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x05\xBF\x81a\x074V[\x90Pa\x05^V[P\x90\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\xE3W`\0\x80\xFD[\x825\x91P` \x83\x015`\xFF\x81\x16\x81\x14a\x05\xFBW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x06\x19W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x061W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06TW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x06fW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x8BW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x06\xACW`\0\x80\xFD[P5\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x06\xE0W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x06\xC4V[\x81\x81\x11\x15a\x06\xF2W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x07HWa\x07Ha\x07\x1EV[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x07aWa\x07aa\x07\x1EV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x07~Wa\x07~a\x07\x1EV[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 3\x93\xC4V\xF5\x85\xF36\xC5\xC6\xE5 f\xCF\xB1\xD7\x02\xAB9\xC0\x08o\xA2\x85\xBFrJ \x91\xDA\xB99dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BITMAPUTILSWRAPPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80cls\xBD\x87\x11a\0qW\x80cls\xBD\x87\x14a\x010W\x80cv7\x0F\x1F\x14a\x01CW\x80c\xA8\xE3\xEA\xBB\x14a\x01iW\x80c\xDDTq\x85\x14a\x01|W\x80c\xF4\xF3\xBD\xC1\x14a\x01\x9CW\x80c\xF9\x0C\xFE\xEF\x14a\x01\xAFW`\0\x80\xFD[\x80c\x1F\xF4\xAD\xBA\x14a\0\xAEW\x80c \xE8\x84\x03\x14a\0\xD6W\x80cN\xE2\x90\x90\x14a\0\xF7W\x80cb\xE2\xEF3\x14a\x01\nW\x80cf\t\x8DO\x14a\x01\x1DW[`\0\x80\xFD[a\0\xC1a\0\xBC6`\x04a\x05\xD0V[a\x01\xC2V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xE9a\0\xE46`\x04a\x06\x06V[a\x01\xDAV[`@Q\x90\x81R` \x01a\0\xCDV[a\0\xE9a\x01\x056`\x04a\x05\xD0V[a\x02\x1BV[a\0\xC1a\x01\x186`\x04a\x06xV[a\x02+V[a\0\xE9a\x01+6`\x04a\x06xV[a\x026V[a\0\xC1a\x01>6`\x04a\x06\x06V[a\x02@V[a\x01Va\x01Q6`\x04a\x06\x9AV[a\x02LV[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xCDV[a\0\xC1a\x01w6`\x04a\x06xV[a\x02WV[a\x01\x8Fa\x01\x8A6`\x04a\x06\x9AV[a\x02cV[`@Qa\0\xCD\x91\x90a\x06\xB3V[a\0\xE9a\x01\xAA6`\x04a\x06xV[a\x02nV[a\0\xC1a\x01\xBD6`\x04a\x06\x9AV[a\x02yV[`\0`\x01`\xFF\x83\x16\x84\x90\x1C\x81\x16\x14[\x90P[\x92\x91PPV[`\0a\x01\xD1\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x02\x82\x92PPPV[`\0`\x01`\xFF\x83\x16\x1B\x83\x17a\x01\xD1V[`\0\x81\x83\x16\x15a\x01\xD1V[`\0\x81\x83\x17a\x01\xD1V[`\0a\x01\xD1\x83\x83a\x04\x14V[`\0a\x01\xD4\x82a\x04\xD8V[`\0\x81\x83\x16\x83\x14a\x01\xD1V[``a\x01\xD4\x82a\x05\x03V[`\0\x81\x19\x83\x16a\x01\xD1V[`\0\x81\x15a\x01\xD4V[`\0a\x01\0\x82Q\x11\x15a\x03\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[\x81Qa\x03\x1EWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a\x034Wa\x034a\x07\x08V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\x04\x0BW\x84\x81\x81Q\x81\x10a\x03bWa\x03ba\x07\x08V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\x03\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x03\x07V[\x91\x81\x17\x91a\x04\x04\x81a\x074V[\x90Pa\x03GV[P\x90\x93\x92PPPV[`\0a\x01\0\x82\x11\x15a\x04(WP`\0a\x01\xD4V[\x81a\x045WP`\x01a\x01\xD4V[`\0\x83\x83`\0\x81\x81\x10a\x04JWa\x04Ja\x07\x08V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x91P`\x01\x90P[\x83\x81\x10\x15a\x04\xCDW`\xF8\x82\x90\x1C\x85\x85\x83\x81\x81\x10a\x04~Wa\x04~a\x07\x08V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x11a\x04\x9EW`\0\x92PPPa\x01\xD4V[\x84\x84\x82\x81\x81\x10a\x04\xB0Wa\x04\xB0a\x07\x08V[\x90P\x015`\xF8\x1C`\xF8\x1B\x91P\x80a\x04\xC6\x90a\x074V[\x90Pa\x04_V[P`\x01\x94\x93PPPPV[`\0\x80[\x82\x15a\x01\xD4Wa\x04\xED`\x01\x84a\x07OV[\x90\x92\x16\x91\x80a\x04\xFB\x81a\x07fV[\x91PPa\x04\xDCV[```\0\x80a\x05\x11\x84a\x04\xD8V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05-Wa\x05-a\x07\x88V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05WW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x05oWPa\x01\0\x81\x10[\x15a\x05\xC6W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x05\xB6W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x05\x98Wa\x05\x98a\x07\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x05\xBF\x81a\x074V[\x90Pa\x05^V[P\x90\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\xE3W`\0\x80\xFD[\x825\x91P` \x83\x015`\xFF\x81\x16\x81\x14a\x05\xFBW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x06\x19W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x061W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06TW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x06fW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x8BW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x06\xACW`\0\x80\xFD[P5\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x06\xE0W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x06\xC4V[\x81\x81\x11\x15a\x06\xF2W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x07HWa\x07Ha\x07\x1EV[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x07aWa\x07aa\x07\x1EV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x07~Wa\x07~a\x07\x1EV[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 3\x93\xC4V\xF5\x85\xF36\xC5\xC6\xE5 f\xCF\xB1\xD7\x02\xAB9\xC0\x08o\xA2\x85\xBFrJ \x91\xDA\xB99dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BITMAPUTILSWRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BitmapUtilsWrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BitmapUtilsWrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BitmapUtilsWrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BitmapUtilsWrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BitmapUtilsWrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BitmapUtilsWrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BitmapUtilsWrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BITMAPUTILSWRAPPER_ABI.clone(),
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
                BITMAPUTILSWRAPPER_ABI.clone(),
                BITMAPUTILSWRAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `bitmapToBytesArray` (0xdd547185) function
        pub fn bitmap_to_bytes_array(
            &self,
            bitmap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([221, 84, 113, 133], bitmap)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `countNumOnes` (0x76370f1f) function
        pub fn count_num_ones(
            &self,
            n: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([118, 55, 15, 31], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isArrayStrictlyAscendingOrdered` (0x6c73bd87) function
        pub fn is_array_strictly_ascending_ordered(
            &self,
            bytes_array: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 115, 189, 135], bytes_array)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isEmpty` (0xf90cfeef) function
        pub fn is_empty(
            &self,
            bitmap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 12, 254, 239], bitmap)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSet` (0x1ff4adba) function
        pub fn is_set(
            &self,
            bitmap: ::ethers::core::types::U256,
            number_to_check_for_inclusion: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [31, 244, 173, 186],
                    (bitmap, number_to_check_for_inclusion),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSubsetOf` (0xa8e3eabb) function
        pub fn is_subset_of(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([168, 227, 234, 187], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minus` (0xf4f3bdc1) function
        pub fn minus(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 243, 189, 193], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `noBitsInCommon` (0x62e2ef33) function
        pub fn no_bits_in_common(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([98, 226, 239, 51], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `orderedBytesArrayToBitmap` (0x20e88403) function
        pub fn ordered_bytes_array_to_bitmap(
            &self,
            ordered_bytes_array: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([32, 232, 132, 3], ordered_bytes_array)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `plus` (0x66098d4f) function
        pub fn plus(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 9, 141, 79], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBit` (0x4ee29090) function
        pub fn set_bit(
            &self,
            bitmap: ::ethers::core::types::U256,
            bit: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 226, 144, 144], (bitmap, bit))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BitmapUtilsWrapper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `bitmapToBytesArray` function with signature `bitmapToBytesArray(uint256)` and selector `0xdd547185`
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
    #[ethcall(name = "bitmapToBytesArray", abi = "bitmapToBytesArray(uint256)")]
    pub struct BitmapToBytesArrayCall {
        pub bitmap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `countNumOnes` function with signature `countNumOnes(uint256)` and selector `0x76370f1f`
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
    #[ethcall(name = "countNumOnes", abi = "countNumOnes(uint256)")]
    pub struct CountNumOnesCall {
        pub n: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isArrayStrictlyAscendingOrdered` function with signature `isArrayStrictlyAscendingOrdered(bytes)` and selector `0x6c73bd87`
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
        name = "isArrayStrictlyAscendingOrdered",
        abi = "isArrayStrictlyAscendingOrdered(bytes)"
    )]
    pub struct IsArrayStrictlyAscendingOrderedCall {
        pub bytes_array: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isEmpty` function with signature `isEmpty(uint256)` and selector `0xf90cfeef`
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
    #[ethcall(name = "isEmpty", abi = "isEmpty(uint256)")]
    pub struct IsEmptyCall {
        pub bitmap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isSet` function with signature `isSet(uint256,uint8)` and selector `0x1ff4adba`
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
    #[ethcall(name = "isSet", abi = "isSet(uint256,uint8)")]
    pub struct IsSetCall {
        pub bitmap: ::ethers::core::types::U256,
        pub number_to_check_for_inclusion: u8,
    }
    ///Container type for all input parameters for the `isSubsetOf` function with signature `isSubsetOf(uint256,uint256)` and selector `0xa8e3eabb`
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
    #[ethcall(name = "isSubsetOf", abi = "isSubsetOf(uint256,uint256)")]
    pub struct IsSubsetOfCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `minus` function with signature `minus(uint256,uint256)` and selector `0xf4f3bdc1`
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
    #[ethcall(name = "minus", abi = "minus(uint256,uint256)")]
    pub struct MinusCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `noBitsInCommon` function with signature `noBitsInCommon(uint256,uint256)` and selector `0x62e2ef33`
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
    #[ethcall(name = "noBitsInCommon", abi = "noBitsInCommon(uint256,uint256)")]
    pub struct NoBitsInCommonCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `orderedBytesArrayToBitmap` function with signature `orderedBytesArrayToBitmap(bytes)` and selector `0x20e88403`
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
        name = "orderedBytesArrayToBitmap",
        abi = "orderedBytesArrayToBitmap(bytes)"
    )]
    pub struct OrderedBytesArrayToBitmapCall {
        pub ordered_bytes_array: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `plus` function with signature `plus(uint256,uint256)` and selector `0x66098d4f`
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
    #[ethcall(name = "plus", abi = "plus(uint256,uint256)")]
    pub struct PlusCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setBit` function with signature `setBit(uint256,uint8)` and selector `0x4ee29090`
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
    #[ethcall(name = "setBit", abi = "setBit(uint256,uint8)")]
    pub struct SetBitCall {
        pub bitmap: ::ethers::core::types::U256,
        pub bit: u8,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BitmapUtilsWrapperCalls {
        BitmapToBytesArray(BitmapToBytesArrayCall),
        CountNumOnes(CountNumOnesCall),
        IsArrayStrictlyAscendingOrdered(IsArrayStrictlyAscendingOrderedCall),
        IsEmpty(IsEmptyCall),
        IsSet(IsSetCall),
        IsSubsetOf(IsSubsetOfCall),
        Minus(MinusCall),
        NoBitsInCommon(NoBitsInCommonCall),
        OrderedBytesArrayToBitmap(OrderedBytesArrayToBitmapCall),
        Plus(PlusCall),
        SetBit(SetBitCall),
    }
    impl ::ethers::core::abi::AbiDecode for BitmapUtilsWrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BitmapToBytesArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BitmapToBytesArray(decoded));
            }
            if let Ok(decoded) = <CountNumOnesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CountNumOnes(decoded));
            }
            if let Ok(decoded) = <IsArrayStrictlyAscendingOrderedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsArrayStrictlyAscendingOrdered(decoded));
            }
            if let Ok(decoded) = <IsEmptyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsEmpty(decoded));
            }
            if let Ok(decoded) = <IsSetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsSet(decoded));
            }
            if let Ok(decoded) = <IsSubsetOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsSubsetOf(decoded));
            }
            if let Ok(decoded) = <MinusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Minus(decoded));
            }
            if let Ok(decoded) = <NoBitsInCommonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoBitsInCommon(decoded));
            }
            if let Ok(decoded) = <OrderedBytesArrayToBitmapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OrderedBytesArrayToBitmap(decoded));
            }
            if let Ok(decoded) = <PlusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Plus(decoded));
            }
            if let Ok(decoded) = <SetBitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BitmapUtilsWrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BitmapToBytesArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CountNumOnes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsArrayStrictlyAscendingOrdered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsEmpty(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSubsetOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Minus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoBitsInCommon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderedBytesArrayToBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Plus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBit(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BitmapUtilsWrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BitmapToBytesArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CountNumOnes(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsArrayStrictlyAscendingOrdered(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSubsetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Minus(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoBitsInCommon(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderedBytesArrayToBitmap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Plus(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BitmapToBytesArrayCall> for BitmapUtilsWrapperCalls {
        fn from(value: BitmapToBytesArrayCall) -> Self {
            Self::BitmapToBytesArray(value)
        }
    }
    impl ::core::convert::From<CountNumOnesCall> for BitmapUtilsWrapperCalls {
        fn from(value: CountNumOnesCall) -> Self {
            Self::CountNumOnes(value)
        }
    }
    impl ::core::convert::From<IsArrayStrictlyAscendingOrderedCall>
    for BitmapUtilsWrapperCalls {
        fn from(value: IsArrayStrictlyAscendingOrderedCall) -> Self {
            Self::IsArrayStrictlyAscendingOrdered(value)
        }
    }
    impl ::core::convert::From<IsEmptyCall> for BitmapUtilsWrapperCalls {
        fn from(value: IsEmptyCall) -> Self {
            Self::IsEmpty(value)
        }
    }
    impl ::core::convert::From<IsSetCall> for BitmapUtilsWrapperCalls {
        fn from(value: IsSetCall) -> Self {
            Self::IsSet(value)
        }
    }
    impl ::core::convert::From<IsSubsetOfCall> for BitmapUtilsWrapperCalls {
        fn from(value: IsSubsetOfCall) -> Self {
            Self::IsSubsetOf(value)
        }
    }
    impl ::core::convert::From<MinusCall> for BitmapUtilsWrapperCalls {
        fn from(value: MinusCall) -> Self {
            Self::Minus(value)
        }
    }
    impl ::core::convert::From<NoBitsInCommonCall> for BitmapUtilsWrapperCalls {
        fn from(value: NoBitsInCommonCall) -> Self {
            Self::NoBitsInCommon(value)
        }
    }
    impl ::core::convert::From<OrderedBytesArrayToBitmapCall>
    for BitmapUtilsWrapperCalls {
        fn from(value: OrderedBytesArrayToBitmapCall) -> Self {
            Self::OrderedBytesArrayToBitmap(value)
        }
    }
    impl ::core::convert::From<PlusCall> for BitmapUtilsWrapperCalls {
        fn from(value: PlusCall) -> Self {
            Self::Plus(value)
        }
    }
    impl ::core::convert::From<SetBitCall> for BitmapUtilsWrapperCalls {
        fn from(value: SetBitCall) -> Self {
            Self::SetBit(value)
        }
    }
    ///Container type for all return fields from the `bitmapToBytesArray` function with signature `bitmapToBytesArray(uint256)` and selector `0xdd547185`
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
    pub struct BitmapToBytesArrayReturn {
        pub bytes_array: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `countNumOnes` function with signature `countNumOnes(uint256)` and selector `0x76370f1f`
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
    pub struct CountNumOnesReturn(pub u16);
    ///Container type for all return fields from the `isArrayStrictlyAscendingOrdered` function with signature `isArrayStrictlyAscendingOrdered(bytes)` and selector `0x6c73bd87`
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
    pub struct IsArrayStrictlyAscendingOrderedReturn(pub bool);
    ///Container type for all return fields from the `isEmpty` function with signature `isEmpty(uint256)` and selector `0xf90cfeef`
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
    pub struct IsEmptyReturn(pub bool);
    ///Container type for all return fields from the `isSet` function with signature `isSet(uint256,uint8)` and selector `0x1ff4adba`
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
    pub struct IsSetReturn(pub bool);
    ///Container type for all return fields from the `isSubsetOf` function with signature `isSubsetOf(uint256,uint256)` and selector `0xa8e3eabb`
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
    pub struct IsSubsetOfReturn(pub bool);
    ///Container type for all return fields from the `minus` function with signature `minus(uint256,uint256)` and selector `0xf4f3bdc1`
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
    pub struct MinusReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `noBitsInCommon` function with signature `noBitsInCommon(uint256,uint256)` and selector `0x62e2ef33`
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
    pub struct NoBitsInCommonReturn(pub bool);
    ///Container type for all return fields from the `orderedBytesArrayToBitmap` function with signature `orderedBytesArrayToBitmap(bytes)` and selector `0x20e88403`
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
    pub struct OrderedBytesArrayToBitmapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `plus` function with signature `plus(uint256,uint256)` and selector `0x66098d4f`
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
    pub struct PlusReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `setBit` function with signature `setBit(uint256,uint8)` and selector `0x4ee29090`
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
    pub struct SetBitReturn(pub ::ethers::core::types::U256);
}
