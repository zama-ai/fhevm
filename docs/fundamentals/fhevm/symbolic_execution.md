# Symbolic Execution

Symbolic execution is a method of constructing a computational graph of FHE operations without actually doing the FHE computation. It works by utilizing what we call a ciphertext **handle**. The handle could be thought of as an unique "pointer" to a given FHE ciphertext.and is implemented as a 32-byte value that is a result of applying a hash function to either an FHE ciphertext or other handles. Symbolic execution also checks constraints on input handles (e.g. the access control list, whether types match, etc.).

Symbolic execution onchain is implemented via the [HTTPZExecutor](../../../contracts/contracts/HTTPZExecutor.sol) contract. One of its main responsibilites is to deterministically generate ciphertext handles. For this, we hash the FHE operation requested and the inputs to produce the result handle H:

```
H = keccak256(fheOperation, input1, input2, ..., inputN)
```

Inputs can either be other handles or plaintext values.

## FHE Computation Data Dependencies

Note that fhEVM-native and fhEVM-coprocessor send both input handles and result handles for FHE computation. It is able to do that, because result handles are computed symbolically in the HTTPZExecutor contract. That allows for parallel FHE computation by analysing which computations are independent.

The Executor or Coprocessor can detect a conflict if an output of computation A (or the output of another computation depending on the output of A) is also used as an input in a subsequent computation B. We call these computations `dependent` and we need to execute them in order.

On the other hand, if two computations have inputs that are not related to their outputs, we call them `independent` and can schedule them to run in parallel.
