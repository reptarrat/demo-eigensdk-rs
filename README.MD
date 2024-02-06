# Eigen Layer AVS Rust SDK

This repo outlines a proposal for an AVS rust sdk. This sdk could be leveraged for building arbitrary AVS's. This may include completely custom AVS or AVS's that use pre-existing frameworks like Substrate.

## Proposal Overview 

The initial proposal is to provide a rust analogue for the [eigensdk-go, maintained by EigenLabs](https://github.com/Layr-Labs/eigensdk-go). The proposed rust sdk could be built under Eigen Labs or another entity.

Specifically, we propose building the following crates to reach feature parity with eigensdk-go:

* Node API - HTTP REST api for common AVS endpoints (todo links to specs)
* Node metrics - Prometheus metrics common for AVS nodes. We propose implementing a generic metric interface and providing a compatible ready to use prometheus registry that developers can opt into. (todo links to specs)
* Chain I/O - a series of clients to interact directly with the EigenLayer core and middleware smart contracts.
* Contract - bindings for EigenLayer core and middleware smart contracts.
* Crypto - convience wrappers with secret persistence formats for cryptographic primitives.
  * bls
  * bn254
  * ecdsa
* Services
  * Operator Public Keys - index [NewPubkeyRegistration events](https://github.com/Layr-Labs/eigenlayer-middleware/blob/9aa6eb543fe38db6e41516f89f15b654ad4d6bf4/src/interfaces/IBLSApkRegistry.sol#L38) and provide an endpoint to query for the G1 and G2 key by operator address.
  * AVS Registry - Index AVS registry and expose of series endpoints to query for operator metadata and AVS quorum info.
  * BLS Signature Aggregator - provide endpoints to initialize AVS tasks, gather signatures for them, and provide BLS aggregate for tasks that have a threshold of signatures.
* Signer - ECDSA signer for smart contract interaction. This will support signing with keys passed by value and password encrypted keys stored as JSON.
* Tracing - a reasonable default tokio-rs tracing subscriber for plugging into the `log` facade used in the rest of the crates.


## Implementation plan

Phase 1: groundwork:

* Basic github CI: test, fmt, clippy, release build
* Signer
* Logging facade and tracing
* Contract bindings
* Crypto
  * bls
  * bn254
  * ecdsa

Phase 2: protocol client:

* Chain I/O
  * Eth client convience wrapper
  * Transaction manager
* Integration tests (ideally these have a C)

Phase 3.0: AVS services

* AVS registry
* Operator Public Keys

The AVS services should be implemented in such a way that they are generic over a DB. As a first pass in Phase 3.0, the DBs will be instantiated as in memory stores, usable in local dev and testing. TBD what the data store will look like for each service (KV vs tables vs objects), but the generic interface should support one type of data store.

---

Everything beyond here is just a soft proposal and should be more precisely scoped after completing the prior phases.

Phase 3.1 (Optional): Data persistence for AVS services

Provide bindings to generic DB abstractions to persistent (FS based) data stores for AVS services. TBD if this should be excluded from scope.

Phase 4.0:

Task manager PoC

Phase X: non critical groundwork:

This is work that is not essential to complete before phase 2, but we will eventually likely want to include calls to the metrics collector within Chain I/0.

* AVS node api
* AVS node metrics
