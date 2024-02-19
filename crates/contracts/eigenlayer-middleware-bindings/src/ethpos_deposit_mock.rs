pub use ethpos_deposit_mock::*;
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
pub mod ethpos_deposit_mock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawal_credentials",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deposit_data_root"),
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
                    ::std::borrow::ToOwned::to_owned("get_deposit_count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_deposit_count"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("get_deposit_root"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_deposit_root"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DepositEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DepositEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawal_credentials",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static ETHPOSDEPOSITMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\n\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x004W`\x005`\xE0\x1C\x80c\"\x89Q\x18\x14a\09W\x80cb\x1F\xD10\x14a\0RW\x80c\xC5\xF2\x89/\x14a\0wW[`\0\x80\xFD[a\0Pa\0G6`\x04a\0\xDCV[PPPPPPPV[\0[4\x80\x15a\0^W`\0\x80\xFD[P```@Qa\0n\x91\x90a\x01\x7FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x83W`\0\x80\xFD[P`@Q`\0\x81R` \x01a\0nV[`\0\x80\x83`\x1F\x84\x01\x12a\0\xA5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xBDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\0\xD5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\0\xF7W`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\x0FW`\0\x80\xFD[a\x01\x1B\x8B\x83\x8C\x01a\0\x93V[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a\x014W`\0\x80\xFD[a\x01@\x8B\x83\x8C\x01a\0\x93V[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a\x01YW`\0\x80\xFD[Pa\x01f\x8A\x82\x8B\x01a\0\x93V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x01\xACW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\x90V[\x81\x81\x11\x15a\x01\xBEW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 D\x90\xF9:\x8C\x05\xCF\x82=\x95\x14O\xC8\xEA\x11\x94\xBB\xD4\xD1\xFEd9\xC7]\xFEr\x18\xD9lr\xB7\xB1dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static ETHPOSDEPOSITMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x004W`\x005`\xE0\x1C\x80c\"\x89Q\x18\x14a\09W\x80cb\x1F\xD10\x14a\0RW\x80c\xC5\xF2\x89/\x14a\0wW[`\0\x80\xFD[a\0Pa\0G6`\x04a\0\xDCV[PPPPPPPV[\0[4\x80\x15a\0^W`\0\x80\xFD[P```@Qa\0n\x91\x90a\x01\x7FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x83W`\0\x80\xFD[P`@Q`\0\x81R` \x01a\0nV[`\0\x80\x83`\x1F\x84\x01\x12a\0\xA5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xBDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\0\xD5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\0\xF7W`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\x0FW`\0\x80\xFD[a\x01\x1B\x8B\x83\x8C\x01a\0\x93V[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a\x014W`\0\x80\xFD[a\x01@\x8B\x83\x8C\x01a\0\x93V[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a\x01YW`\0\x80\xFD[Pa\x01f\x8A\x82\x8B\x01a\0\x93V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x01\xACW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\x90V[\x81\x81\x11\x15a\x01\xBEW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 D\x90\xF9:\x8C\x05\xCF\x82=\x95\x14O\xC8\xEA\x11\x94\xBB\xD4\xD1\xFEd9\xC7]\xFEr\x18\xD9lr\xB7\xB1dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static ETHPOSDEPOSITMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ETHPOSDepositMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ETHPOSDepositMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ETHPOSDepositMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ETHPOSDepositMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ETHPOSDepositMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ETHPOSDepositMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ETHPOSDepositMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ETHPOSDEPOSITMOCK_ABI.clone(),
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
                ETHPOSDEPOSITMOCK_ABI.clone(),
                ETHPOSDEPOSITMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deposit` (0x22895118) function
        pub fn deposit(
            &self,
            pubkey: ::ethers::core::types::Bytes,
            withdrawal_credentials: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
            deposit_data_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 137, 81, 24],
                    (pubkey, withdrawal_credentials, signature, deposit_data_root),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_deposit_count` (0x621fd130) function
        pub fn get_deposit_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([98, 31, 209, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_deposit_root` (0xc5f2892f) function
        pub fn get_deposit_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([197, 242, 137, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DepositEvent` event
        pub fn deposit_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEventFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEventFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ETHPOSDepositMock<M> {
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
        name = "DepositEvent",
        abi = "DepositEvent(bytes,bytes,bytes,bytes,bytes)"
    )]
    pub struct DepositEventFilter {
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub amount: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub index: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(bytes,bytes,bytes,bytes32)` and selector `0x22895118`
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
    #[ethcall(name = "deposit", abi = "deposit(bytes,bytes,bytes,bytes32)")]
    pub struct DepositCall {
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub deposit_data_root: [u8; 32],
    }
    ///Container type for all input parameters for the `get_deposit_count` function with signature `get_deposit_count()` and selector `0x621fd130`
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
    #[ethcall(name = "get_deposit_count", abi = "get_deposit_count()")]
    pub struct GetDepositCountCall;
    ///Container type for all input parameters for the `get_deposit_root` function with signature `get_deposit_root()` and selector `0xc5f2892f`
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
    #[ethcall(name = "get_deposit_root", abi = "get_deposit_root()")]
    pub struct GetDepositRootCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ETHPOSDepositMockCalls {
        Deposit(DepositCall),
        GetDepositCount(GetDepositCountCall),
        GetDepositRoot(GetDepositRootCall),
    }
    impl ::ethers::core::abi::AbiDecode for ETHPOSDepositMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <GetDepositCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDepositCount(decoded));
            }
            if let Ok(decoded) = <GetDepositRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDepositRoot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ETHPOSDepositMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDepositRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ETHPOSDepositMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositRoot(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for ETHPOSDepositMockCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetDepositCountCall> for ETHPOSDepositMockCalls {
        fn from(value: GetDepositCountCall) -> Self {
            Self::GetDepositCount(value)
        }
    }
    impl ::core::convert::From<GetDepositRootCall> for ETHPOSDepositMockCalls {
        fn from(value: GetDepositRootCall) -> Self {
            Self::GetDepositRoot(value)
        }
    }
    ///Container type for all return fields from the `get_deposit_count` function with signature `get_deposit_count()` and selector `0x621fd130`
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
    pub struct GetDepositCountReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `get_deposit_root` function with signature `get_deposit_root()` and selector `0xc5f2892f`
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
    pub struct GetDepositRootReturn(pub [u8; 32]);
}
