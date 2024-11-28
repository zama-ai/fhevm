# Coprocessor Backend

A Coprocesor backend is needed to run alongside the geth node. The Coprocessor backend executes the actual FHE computation. Please look at [FHE Computation](../../../fundamentals/fhevm/coprocessor/fhe_computation.md) for more info.

The coprocessor backend is implemented in the [Coprocessor](../../../../fhevm-engine/coprocessor/README.md) directory of `fhevm-engine`.

It consists of the following components:
 * **server** that handles:
    * gRPC requests for computation from geth
    * input insertion requests to the DB from the Gateway
    * FHE ciphertext read requests from the Gateway
 * **PostgreSQL DB** for storing computation requests and FHE ciphertexts
 * **worker** that reads comoutation requests from the DB, does the FHE computation and inserts result FHE ciphertexts into the DB

The server and the worker can be run as separate processes or as a single process. In both cases they communicate with one another through the DB.

The Coprocessor backend supports **multi-tenancy** in the sense that it can perform FHE computation for separate host blockchains, under different FHE keys.

You can use pre-generated Docker images for the Coprocessor backend node or build them yourself as described in the [README](../../../../fhevm-engine/coprocessor/README.md).

Please note that a [Coprocessor geth full node](geth.md) is needed in order to execute blocks on the host blockchain and trigger FHE computation.
