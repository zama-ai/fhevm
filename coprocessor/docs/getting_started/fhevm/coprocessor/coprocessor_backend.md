# Coprocessor Backend

A Coprocessor backend is needed to run alongside the geth node. The Coprocessor backend executes the actual FHE computation. Please look at [FHE Computation](../../../fundamentals/fhevm/coprocessor/fhe_computation.md) for more info.

The coprocessor backend is implemented in the [Coprocessor](../../../../fhevm-engine/coprocessor/README.md) directory of `fhevm-engine`.

It consists of the following components:
 * **listeners** that propagate events to the DB and Gateway,
 * **server** that handles:
    * input insertion requests to the DB from the Gateway
    * FHE ciphertext read requests from the Gateway
 * **PostgreSQL DB** for storing computation requests and FHE ciphertexts
 * **worker** that reads computation requests from the DB, does the FHE computation and inserts result FHE ciphertexts into the DB

The server and the worker can be run as separate processes or as a single process. In both cases they communicate with one another through the DB.

The Coprocessor backend supports **multi-tenancy** in the sense that it can perform FHE computation for separate host blockchains, under different FHE keys.

You can use pre-generated Docker images for the Coprocessor backend node or build them yourself as described in the [README](../../../../fhevm-engine/coprocessor/README.md).
