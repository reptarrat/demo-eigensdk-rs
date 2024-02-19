pub use beacon_chain_oracle_mock::*;
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
pub mod beacon_chain_oracle_mock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("setBlockRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBlockRoot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockRoot"),
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
                    ::std::borrow::ToOwned::to_owned("timestampToBlockRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "timestampToBlockRoot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BEACONCHAINORACLEMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01\x1F\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`2W`\x005`\xE0\x1C\x80cd5\x99\xF2\x14`7W\x80c\xAC\xD4\x14\xA8\x14`pW[`\0\x80\xFD[`^`B6`\x04`\x9AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x98`{6`\x04`\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[\0[`\0` \x82\x84\x03\x12\x15`\xABW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15`\xC4W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14`\xDBW`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV\xFE\xA2dipfsX\"\x12 k\xCB\xA4\xBB\x18\xD0\x14\xAB\x8D\x95Q\xD3\xEB\xF6\x16\xD7d5\xE4\x89s8\x1Eqg(\xF4\xFE>j\xC6ddsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BEACONCHAINORACLEMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`2W`\x005`\xE0\x1C\x80cd5\x99\xF2\x14`7W\x80c\xAC\xD4\x14\xA8\x14`pW[`\0\x80\xFD[`^`B6`\x04`\x9AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x98`{6`\x04`\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[\0[`\0` \x82\x84\x03\x12\x15`\xABW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15`\xC4W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14`\xDBW`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV\xFE\xA2dipfsX\"\x12 k\xCB\xA4\xBB\x18\xD0\x14\xAB\x8D\x95Q\xD3\xEB\xF6\x16\xD7d5\xE4\x89s8\x1Eqg(\xF4\xFE>j\xC6ddsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BEACONCHAINORACLEMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BeaconChainOracleMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BeaconChainOracleMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BeaconChainOracleMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BeaconChainOracleMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BeaconChainOracleMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BeaconChainOracleMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BeaconChainOracleMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BEACONCHAINORACLEMOCK_ABI.clone(),
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
                BEACONCHAINORACLEMOCK_ABI.clone(),
                BEACONCHAINORACLEMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `setBlockRoot` (0xacd414a8) function
        pub fn set_block_root(
            &self,
            timestamp: u64,
            block_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 212, 20, 168], (timestamp, block_root))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timestampToBlockRoot` (0x643599f2) function
        pub fn timestamp_to_block_root(
            &self,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([100, 53, 153, 242], timestamp)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BeaconChainOracleMock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `setBlockRoot` function with signature `setBlockRoot(uint64,bytes32)` and selector `0xacd414a8`
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
    #[ethcall(name = "setBlockRoot", abi = "setBlockRoot(uint64,bytes32)")]
    pub struct SetBlockRootCall {
        pub timestamp: u64,
        pub block_root: [u8; 32],
    }
    ///Container type for all input parameters for the `timestampToBlockRoot` function with signature `timestampToBlockRoot(uint256)` and selector `0x643599f2`
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
    #[ethcall(name = "timestampToBlockRoot", abi = "timestampToBlockRoot(uint256)")]
    pub struct TimestampToBlockRootCall {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BeaconChainOracleMockCalls {
        SetBlockRoot(SetBlockRootCall),
        TimestampToBlockRoot(TimestampToBlockRootCall),
    }
    impl ::ethers::core::abi::AbiDecode for BeaconChainOracleMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SetBlockRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBlockRoot(decoded));
            }
            if let Ok(decoded) = <TimestampToBlockRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TimestampToBlockRoot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BeaconChainOracleMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SetBlockRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimestampToBlockRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BeaconChainOracleMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetBlockRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimestampToBlockRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SetBlockRootCall> for BeaconChainOracleMockCalls {
        fn from(value: SetBlockRootCall) -> Self {
            Self::SetBlockRoot(value)
        }
    }
    impl ::core::convert::From<TimestampToBlockRootCall> for BeaconChainOracleMockCalls {
        fn from(value: TimestampToBlockRootCall) -> Self {
            Self::TimestampToBlockRoot(value)
        }
    }
    ///Container type for all return fields from the `timestampToBlockRoot` function with signature `timestampToBlockRoot(uint256)` and selector `0x643599f2`
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
    pub struct TimestampToBlockRootReturn(pub [u8; 32]);
}
