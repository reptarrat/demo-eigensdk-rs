# EigenLayer AVS Rust SDK

This repo outlines a proposal for an AVS rust sdk. This sdk can be leveraged to build arbitrary AVSs. These can include AVSs that use pre-existing frameworks like Substrate or that are completely custom.

## Proposal Overview

The initial proposal is to provide a rust analogue for the [eigensdk-go, maintained by EigenLabs](https://github.com/Layr-Labs/eigensdk-go). The proposed rust sdk can be built under EigenLabs or another entity.

### Modules

Specifically, we propose building the following crates to reach feature parity with the eigensdk-go:


* `chainio` - a series of clients to interact directly with the EigenLayer core and middleware smart contracts.
* `contracts` - bindings for EigenLayer core and middleware smart contracts.
* crypto - convenience wrappers with secret persistence formats for cryptographic primitives.
  * `bls`
  * `bn254`
  * `ecdsa`
* services
  * `operator-pubkeys` - index [NewPubkeyRegistration events](https://github.com/Layr-Labs/eigenlayer-middleware/blob/9aa6eb543fe38db6e41516f89f15b654ad4d6bf4/src/interfaces/IBLSApkRegistry.sol#L38) and provide an endpoint to query for the G1 and G2 key by operator address.
  * `avs-registry` - index AVS registry and expose of series endpoints to query for operator metadata and AVS quorum info.
  * `bls-aggregator` - provide endpoints to initialize AVS tasks, gather task signatures, and provide BLS aggregations for tasks that require a threshold of signatures.
* `signer` - ECDSA signer for smart contract interactions. This will support signing with keys passed by value and password encrypted keys stored as JSON.
* `tracing` - a default tokio-rs tracing subscriber for plugging into the `log` facade used in the rest of the crates.
* `node-api` - HTTP REST API for common AVS endpoints ([specs](https://eigen.nethermind.io/docs/category/avs-node-api))
* `node-metrics` - Prometheus metrics common for AVS nodes. We propose implementing a generic metric interface and providing a compatible ready to use Prometheus registry that developers can opt into. ([specs](https://eigen.nethermind.io/docs/category/metrics))

### Dependencies
```
# crypto
# rust crypto ecdsa library
ecdsa = { version = "0.16", default-features = false }
# derivative of parity's bn library
bn254 = { version = "0.0.1", default-features = false }
# filecoin bls library
bls-signatures = { version = "0.15.0" }

# eth
# eth type primitives
alloy-primitives = { version = "0.6", default-features = false }
# json abi interfaces
alloy-json-abi = { version = "0.6", default-features = false }
# experimental eth json-rpc client. may need to use ethers-rs to support functionality that is not yet released by alloy
alloy-rpc-client = { git = "https://github.com/alloy-rs/alloy" }

# serde
# json interactions
serde = { version = "1.0", features = ["derive"], default-features = false }
serde_json = { version = "1.0", default-features = false }

# misc.
# http server/router
axum = { version = "0.7", default-features = false }
# facade for tracking and exporting metrics
metrics = { version = "0.22", default-features = false }
# prometheus plugin for the metrics facade
metrics-exporter-prometheus = { version = "0.13", default-features = false }
# logging facade
log = { version = "0.4", default-features = false }
# default plugin for log
tracing = { version = "0.1.0", default-features = false }
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "json"], default-features = false }
```


## Deliverables

### Phase 1: Launchpad

* Basic Github CI: test, fmt, clippy, release build
* Signer
* Logging facade and tracing
* Contract bindings
* Crypto
  * bls
  * bn254
  * ecdsa
* Unit tests

### Phase 2: Engines running

* Chain I/O
  * ETH client convenience wrapper
  * Transaction manager
* Unit tests
* Integration tests with Anvil chain

### Phase 3: Lift-off

Each of these services will be a running binary as well as an associated set of libraries.

* AVS services
  * AVS registry
  * BLS aggregator
  * Operator public keys

These services will be implemented such that they are generic over a database. As a first pass, the databases will be instantiated as in-memory stores, usable in local dev and testing.

### Phase 4: Follow-up

* AVS node API
* AVS node metrics

## Future considerations

### Data persistence for AVS services

Provide bindings to generic database abstractions to persistent (FS based) data stores for AVS services.

### Task management

Different AVSs may have different requirements for task queues or pools.

* Ephemeral pool that is gossiped with libp2p
* Task type classification (retry policies, DDoS protection)
* Multiple queue implementations as AVSs or modules used by AVSs

### Decentralized load balancing

### Hot and cold wallet support

### Operator rotation module





