pub use i_delayed_withdrawal_router::*;
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
pub mod i_delayed_withdrawal_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("canClaimDelayedWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "canClaimDelayedWithdrawal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimDelayedWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "claimDelayedWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "maxNumberOfWithdrawalsToClaim",
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "claimDelayedWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "maxNumberOfWithdrawalsToClaim",
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
                    ::std::borrow::ToOwned::to_owned("createDelayedWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createDelayedWithdrawal",
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
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "getClaimableUserDelayedWithdrawals",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getClaimableUserDelayedWithdrawals",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(224usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelayedWithdrawalRouter.DelayedWithdrawal[]",
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
                    ::std::borrow::ToOwned::to_owned("getUserDelayedWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserDelayedWithdrawals",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(224usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelayedWithdrawalRouter.DelayedWithdrawal[]",
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
                    ::std::borrow::ToOwned::to_owned("setWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setWithdrawalDelayBlocks",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
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
                    ::std::borrow::ToOwned::to_owned("userDelayedWithdrawalByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userDelayedWithdrawalByIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(224usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelayedWithdrawalRouter.DelayedWithdrawal",
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
                    ::std::borrow::ToOwned::to_owned("userWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("userWithdrawals"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(224usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelayedWithdrawalRouter.UserDelayedWithdrawals",
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
                    ::std::borrow::ToOwned::to_owned("userWithdrawalsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userWithdrawalsLength",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("withdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawalDelayBlocks",
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DelayedWithdrawalCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelayedWithdrawalCreated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("podOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("DelayedWithdrawalsClaimed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelayedWithdrawalsClaimed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountClaimed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "delayedWithdrawalsCompleted",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("WithdrawalDelayBlocksSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawalDelayBlocksSet",
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDELAYEDWITHDRAWALROUTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IDelayedWithdrawalRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDelayedWithdrawalRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDelayedWithdrawalRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDelayedWithdrawalRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDelayedWithdrawalRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDelayedWithdrawalRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDelayedWithdrawalRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDELAYEDWITHDRAWALROUTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `canClaimDelayedWithdrawal` (0x75608896) function
        pub fn can_claim_delayed_withdrawal(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([117, 96, 136, 150], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimDelayedWithdrawals` (0xd44e1b76) function
        pub fn claim_delayed_withdrawals(
            &self,
            max_number_of_withdrawals_to_claim: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 78, 27, 118], max_number_of_withdrawals_to_claim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimDelayedWithdrawals` (0xe5db06c0) function
        pub fn claim_delayed_withdrawals_with_recipient(
            &self,
            recipient: ::ethers::core::types::Address,
            max_number_of_withdrawals_to_claim: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [229, 219, 6, 192],
                    (recipient, max_number_of_withdrawals_to_claim),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDelayedWithdrawal` (0xc0db354c) function
        pub fn create_delayed_withdrawal(
            &self,
            pod_owner: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 219, 53, 76], (pod_owner, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getClaimableUserDelayedWithdrawals` (0x1f39d87f) function
        pub fn get_claimable_user_delayed_withdrawals(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<DelayedWithdrawal>,
        > {
            self.0
                .method_hash([31, 57, 216, 127], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserDelayedWithdrawals` (0x3e1de008) function
        pub fn get_user_delayed_withdrawals(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<DelayedWithdrawal>,
        > {
            self.0
                .method_hash([62, 29, 224, 8], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawalDelayBlocks` (0x4d50f9a4) function
        pub fn set_withdrawal_delay_blocks(
            &self,
            new_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 80, 249, 164], new_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userDelayedWithdrawalByIndex` (0x85594e58) function
        pub fn user_delayed_withdrawal_by_index(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, DelayedWithdrawal> {
            self.0
                .method_hash([133, 89, 78, 88], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userWithdrawals` (0xecb7cb1b) function
        pub fn user_withdrawals(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, UserDelayedWithdrawals> {
            self.0
                .method_hash([236, 183, 203, 27], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userWithdrawalsLength` (0xe4f4f887) function
        pub fn user_withdrawals_length(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 244, 248, 135], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawalDelayBlocks` (0x50f73e7c) function
        pub fn withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 247, 62, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DelayedWithdrawalCreated` event
        pub fn delayed_withdrawal_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelayedWithdrawalCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DelayedWithdrawalsClaimed` event
        pub fn delayed_withdrawals_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelayedWithdrawalsClaimedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalDelayBlocksSet` event
        pub fn withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalDelayBlocksSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IDelayedWithdrawalRouterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDelayedWithdrawalRouter<M> {
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
        name = "DelayedWithdrawalCreated",
        abi = "DelayedWithdrawalCreated(address,address,uint256,uint256)"
    )]
    pub struct DelayedWithdrawalCreatedFilter {
        pub pod_owner: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
        name = "DelayedWithdrawalsClaimed",
        abi = "DelayedWithdrawalsClaimed(address,uint256,uint256)"
    )]
    pub struct DelayedWithdrawalsClaimedFilter {
        pub recipient: ::ethers::core::types::Address,
        pub amount_claimed: ::ethers::core::types::U256,
        pub delayed_withdrawals_completed: ::ethers::core::types::U256,
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
        name = "WithdrawalDelayBlocksSet",
        abi = "WithdrawalDelayBlocksSet(uint256,uint256)"
    )]
    pub struct WithdrawalDelayBlocksSetFilter {
        pub previous_value: ::ethers::core::types::U256,
        pub new_value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IDelayedWithdrawalRouterEvents {
        DelayedWithdrawalCreatedFilter(DelayedWithdrawalCreatedFilter),
        DelayedWithdrawalsClaimedFilter(DelayedWithdrawalsClaimedFilter),
        WithdrawalDelayBlocksSetFilter(WithdrawalDelayBlocksSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for IDelayedWithdrawalRouterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DelayedWithdrawalCreatedFilter::decode_log(log) {
                return Ok(
                    IDelayedWithdrawalRouterEvents::DelayedWithdrawalCreatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DelayedWithdrawalsClaimedFilter::decode_log(log) {
                return Ok(
                    IDelayedWithdrawalRouterEvents::DelayedWithdrawalsClaimedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = WithdrawalDelayBlocksSetFilter::decode_log(log) {
                return Ok(
                    IDelayedWithdrawalRouterEvents::WithdrawalDelayBlocksSetFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IDelayedWithdrawalRouterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelayedWithdrawalCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelayedWithdrawalsClaimedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DelayedWithdrawalCreatedFilter>
    for IDelayedWithdrawalRouterEvents {
        fn from(value: DelayedWithdrawalCreatedFilter) -> Self {
            Self::DelayedWithdrawalCreatedFilter(value)
        }
    }
    impl ::core::convert::From<DelayedWithdrawalsClaimedFilter>
    for IDelayedWithdrawalRouterEvents {
        fn from(value: DelayedWithdrawalsClaimedFilter) -> Self {
            Self::DelayedWithdrawalsClaimedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalDelayBlocksSetFilter>
    for IDelayedWithdrawalRouterEvents {
        fn from(value: WithdrawalDelayBlocksSetFilter) -> Self {
            Self::WithdrawalDelayBlocksSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `canClaimDelayedWithdrawal` function with signature `canClaimDelayedWithdrawal(address,uint256)` and selector `0x75608896`
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
        name = "canClaimDelayedWithdrawal",
        abi = "canClaimDelayedWithdrawal(address,uint256)"
    )]
    pub struct CanClaimDelayedWithdrawalCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claimDelayedWithdrawals` function with signature `claimDelayedWithdrawals(uint256)` and selector `0xd44e1b76`
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
        name = "claimDelayedWithdrawals",
        abi = "claimDelayedWithdrawals(uint256)"
    )]
    pub struct ClaimDelayedWithdrawalsCall {
        pub max_number_of_withdrawals_to_claim: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claimDelayedWithdrawals` function with signature `claimDelayedWithdrawals(address,uint256)` and selector `0xe5db06c0`
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
        name = "claimDelayedWithdrawals",
        abi = "claimDelayedWithdrawals(address,uint256)"
    )]
    pub struct ClaimDelayedWithdrawalsWithRecipientCall {
        pub recipient: ::ethers::core::types::Address,
        pub max_number_of_withdrawals_to_claim: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createDelayedWithdrawal` function with signature `createDelayedWithdrawal(address,address)` and selector `0xc0db354c`
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
        name = "createDelayedWithdrawal",
        abi = "createDelayedWithdrawal(address,address)"
    )]
    pub struct CreateDelayedWithdrawalCall {
        pub pod_owner: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getClaimableUserDelayedWithdrawals` function with signature `getClaimableUserDelayedWithdrawals(address)` and selector `0x1f39d87f`
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
        name = "getClaimableUserDelayedWithdrawals",
        abi = "getClaimableUserDelayedWithdrawals(address)"
    )]
    pub struct GetClaimableUserDelayedWithdrawalsCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUserDelayedWithdrawals` function with signature `getUserDelayedWithdrawals(address)` and selector `0x3e1de008`
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
        name = "getUserDelayedWithdrawals",
        abi = "getUserDelayedWithdrawals(address)"
    )]
    pub struct GetUserDelayedWithdrawalsCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setWithdrawalDelayBlocks` function with signature `setWithdrawalDelayBlocks(uint256)` and selector `0x4d50f9a4`
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
        name = "setWithdrawalDelayBlocks",
        abi = "setWithdrawalDelayBlocks(uint256)"
    )]
    pub struct SetWithdrawalDelayBlocksCall {
        pub new_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `userDelayedWithdrawalByIndex` function with signature `userDelayedWithdrawalByIndex(address,uint256)` and selector `0x85594e58`
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
        name = "userDelayedWithdrawalByIndex",
        abi = "userDelayedWithdrawalByIndex(address,uint256)"
    )]
    pub struct UserDelayedWithdrawalByIndexCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `userWithdrawals` function with signature `userWithdrawals(address)` and selector `0xecb7cb1b`
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
    #[ethcall(name = "userWithdrawals", abi = "userWithdrawals(address)")]
    pub struct UserWithdrawalsCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userWithdrawalsLength` function with signature `userWithdrawalsLength(address)` and selector `0xe4f4f887`
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
    #[ethcall(name = "userWithdrawalsLength", abi = "userWithdrawalsLength(address)")]
    pub struct UserWithdrawalsLengthCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawalDelayBlocks` function with signature `withdrawalDelayBlocks()` and selector `0x50f73e7c`
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
    #[ethcall(name = "withdrawalDelayBlocks", abi = "withdrawalDelayBlocks()")]
    pub struct WithdrawalDelayBlocksCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IDelayedWithdrawalRouterCalls {
        CanClaimDelayedWithdrawal(CanClaimDelayedWithdrawalCall),
        ClaimDelayedWithdrawals(ClaimDelayedWithdrawalsCall),
        ClaimDelayedWithdrawalsWithRecipient(ClaimDelayedWithdrawalsWithRecipientCall),
        CreateDelayedWithdrawal(CreateDelayedWithdrawalCall),
        GetClaimableUserDelayedWithdrawals(GetClaimableUserDelayedWithdrawalsCall),
        GetUserDelayedWithdrawals(GetUserDelayedWithdrawalsCall),
        SetWithdrawalDelayBlocks(SetWithdrawalDelayBlocksCall),
        UserDelayedWithdrawalByIndex(UserDelayedWithdrawalByIndexCall),
        UserWithdrawals(UserWithdrawalsCall),
        UserWithdrawalsLength(UserWithdrawalsLengthCall),
        WithdrawalDelayBlocks(WithdrawalDelayBlocksCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDelayedWithdrawalRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CanClaimDelayedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanClaimDelayedWithdrawal(decoded));
            }
            if let Ok(decoded) = <ClaimDelayedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimDelayedWithdrawals(decoded));
            }
            if let Ok(decoded) = <ClaimDelayedWithdrawalsWithRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimDelayedWithdrawalsWithRecipient(decoded));
            }
            if let Ok(decoded) = <CreateDelayedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateDelayedWithdrawal(decoded));
            }
            if let Ok(decoded) = <GetClaimableUserDelayedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetClaimableUserDelayedWithdrawals(decoded));
            }
            if let Ok(decoded) = <GetUserDelayedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUserDelayedWithdrawals(decoded));
            }
            if let Ok(decoded) = <SetWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) = <UserDelayedWithdrawalByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserDelayedWithdrawalByIndex(decoded));
            }
            if let Ok(decoded) = <UserWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserWithdrawals(decoded));
            }
            if let Ok(decoded) = <UserWithdrawalsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserWithdrawalsLength(decoded));
            }
            if let Ok(decoded) = <WithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawalDelayBlocks(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDelayedWithdrawalRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CanClaimDelayedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimDelayedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimDelayedWithdrawalsWithRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateDelayedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetClaimableUserDelayedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserDelayedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserDelayedWithdrawalByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserWithdrawalsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IDelayedWithdrawalRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CanClaimDelayedWithdrawal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimDelayedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimDelayedWithdrawalsWithRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateDelayedWithdrawal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetClaimableUserDelayedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserDelayedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserDelayedWithdrawalByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserWithdrawalsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CanClaimDelayedWithdrawalCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: CanClaimDelayedWithdrawalCall) -> Self {
            Self::CanClaimDelayedWithdrawal(value)
        }
    }
    impl ::core::convert::From<ClaimDelayedWithdrawalsCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: ClaimDelayedWithdrawalsCall) -> Self {
            Self::ClaimDelayedWithdrawals(value)
        }
    }
    impl ::core::convert::From<ClaimDelayedWithdrawalsWithRecipientCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: ClaimDelayedWithdrawalsWithRecipientCall) -> Self {
            Self::ClaimDelayedWithdrawalsWithRecipient(value)
        }
    }
    impl ::core::convert::From<CreateDelayedWithdrawalCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: CreateDelayedWithdrawalCall) -> Self {
            Self::CreateDelayedWithdrawal(value)
        }
    }
    impl ::core::convert::From<GetClaimableUserDelayedWithdrawalsCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: GetClaimableUserDelayedWithdrawalsCall) -> Self {
            Self::GetClaimableUserDelayedWithdrawals(value)
        }
    }
    impl ::core::convert::From<GetUserDelayedWithdrawalsCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: GetUserDelayedWithdrawalsCall) -> Self {
            Self::GetUserDelayedWithdrawals(value)
        }
    }
    impl ::core::convert::From<SetWithdrawalDelayBlocksCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: SetWithdrawalDelayBlocksCall) -> Self {
            Self::SetWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<UserDelayedWithdrawalByIndexCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: UserDelayedWithdrawalByIndexCall) -> Self {
            Self::UserDelayedWithdrawalByIndex(value)
        }
    }
    impl ::core::convert::From<UserWithdrawalsCall> for IDelayedWithdrawalRouterCalls {
        fn from(value: UserWithdrawalsCall) -> Self {
            Self::UserWithdrawals(value)
        }
    }
    impl ::core::convert::From<UserWithdrawalsLengthCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: UserWithdrawalsLengthCall) -> Self {
            Self::UserWithdrawalsLength(value)
        }
    }
    impl ::core::convert::From<WithdrawalDelayBlocksCall>
    for IDelayedWithdrawalRouterCalls {
        fn from(value: WithdrawalDelayBlocksCall) -> Self {
            Self::WithdrawalDelayBlocks(value)
        }
    }
    ///Container type for all return fields from the `canClaimDelayedWithdrawal` function with signature `canClaimDelayedWithdrawal(address,uint256)` and selector `0x75608896`
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
    pub struct CanClaimDelayedWithdrawalReturn(pub bool);
    ///Container type for all return fields from the `getClaimableUserDelayedWithdrawals` function with signature `getClaimableUserDelayedWithdrawals(address)` and selector `0x1f39d87f`
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
    pub struct GetClaimableUserDelayedWithdrawalsReturn(
        pub ::std::vec::Vec<DelayedWithdrawal>,
    );
    ///Container type for all return fields from the `getUserDelayedWithdrawals` function with signature `getUserDelayedWithdrawals(address)` and selector `0x3e1de008`
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
    pub struct GetUserDelayedWithdrawalsReturn(pub ::std::vec::Vec<DelayedWithdrawal>);
    ///Container type for all return fields from the `userDelayedWithdrawalByIndex` function with signature `userDelayedWithdrawalByIndex(address,uint256)` and selector `0x85594e58`
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
    pub struct UserDelayedWithdrawalByIndexReturn(pub DelayedWithdrawal);
    ///Container type for all return fields from the `userWithdrawals` function with signature `userWithdrawals(address)` and selector `0xecb7cb1b`
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
    pub struct UserWithdrawalsReturn(pub UserDelayedWithdrawals);
    ///Container type for all return fields from the `userWithdrawalsLength` function with signature `userWithdrawalsLength(address)` and selector `0xe4f4f887`
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
    pub struct UserWithdrawalsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdrawalDelayBlocks` function with signature `withdrawalDelayBlocks()` and selector `0x50f73e7c`
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
    pub struct WithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
}
