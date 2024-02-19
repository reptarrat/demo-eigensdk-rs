pub use i_stake_registry_events::*;
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
pub mod i_stake_registry_events {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
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
    pub static ISTAKEREGISTRYEVENTS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IStakeRegistryEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStakeRegistryEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStakeRegistryEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStakeRegistryEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStakeRegistryEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStakeRegistryEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStakeRegistryEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISTAKEREGISTRYEVENTS_ABI.clone(),
                    client,
                ),
            )
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
            IStakeRegistryEventsEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IStakeRegistryEvents<M> {
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
    pub enum IStakeRegistryEventsEvents {
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        OperatorStakeUpdateFilter(OperatorStakeUpdateFilter),
        QuorumCreatedFilter(QuorumCreatedFilter),
        StrategyAddedToQuorumFilter(StrategyAddedToQuorumFilter),
        StrategyMultiplierUpdatedFilter(StrategyMultiplierUpdatedFilter),
        StrategyRemovedFromQuorumFilter(StrategyRemovedFromQuorumFilter),
    }
    impl ::ethers::contract::EthLogDecode for IStakeRegistryEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(
                    IStakeRegistryEventsEvents::MinimumStakeForQuorumUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorStakeUpdateFilter::decode_log(log) {
                return Ok(
                    IStakeRegistryEventsEvents::OperatorStakeUpdateFilter(decoded),
                );
            }
            if let Ok(decoded) = QuorumCreatedFilter::decode_log(log) {
                return Ok(IStakeRegistryEventsEvents::QuorumCreatedFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToQuorumFilter::decode_log(log) {
                return Ok(
                    IStakeRegistryEventsEvents::StrategyAddedToQuorumFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyMultiplierUpdatedFilter::decode_log(log) {
                return Ok(
                    IStakeRegistryEventsEvents::StrategyMultiplierUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyRemovedFromQuorumFilter::decode_log(log) {
                return Ok(
                    IStakeRegistryEventsEvents::StrategyRemovedFromQuorumFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IStakeRegistryEventsEvents {
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
    for IStakeRegistryEventsEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorStakeUpdateFilter>
    for IStakeRegistryEventsEvents {
        fn from(value: OperatorStakeUpdateFilter) -> Self {
            Self::OperatorStakeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<QuorumCreatedFilter> for IStakeRegistryEventsEvents {
        fn from(value: QuorumCreatedFilter) -> Self {
            Self::QuorumCreatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToQuorumFilter>
    for IStakeRegistryEventsEvents {
        fn from(value: StrategyAddedToQuorumFilter) -> Self {
            Self::StrategyAddedToQuorumFilter(value)
        }
    }
    impl ::core::convert::From<StrategyMultiplierUpdatedFilter>
    for IStakeRegistryEventsEvents {
        fn from(value: StrategyMultiplierUpdatedFilter) -> Self {
            Self::StrategyMultiplierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromQuorumFilter>
    for IStakeRegistryEventsEvents {
        fn from(value: StrategyRemovedFromQuorumFilter) -> Self {
            Self::StrategyRemovedFromQuorumFilter(value)
        }
    }
}
