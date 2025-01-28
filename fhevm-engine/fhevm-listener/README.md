# fhEVM-Listener

The fhevm-listener primary role is to observe the block chain execution and extend that execution off the chain.

## How

Our contracts actively emits events that forms the trace of a symbolic execution. These events can be observed via the blockchain node pubsub events feature.

## Command-line

If already compiled you can just call the binary directly:
```
../target/debug/listen -coprocessor-api-key 00000000000000000000000000000000
```

If you have no coprocessor-api-key, for local tests, you can do
```
psql
postgres=# insert into tenants values (13, '00000000000000000000000000000000', 0, 'contract verify', 'contract acl', '0'::bytea, '0'::bytea, '0'::bytea);
```

Otherwise you can compile + run with:

```
DATABASE_URL=postgresql://postgres:testmdp@0.0.0.0:5432 cargo run -- --coprocessor-api-key 00000000000000000000000000000000
```

DATABASE_URL need to specify an online database to compile SQL requests.

By default the listener propagate TFHE operation events to the database.
You can change the database url using --database-url, it defaults to a local test database url.
If you want to disable TFHE operation events propagation, you can provide an empty database-url.

## Events in FHEVM

### Blockchain Events
> Status: in progress
Blockchain events are used export the symbolic execution of TFHE operations from a blockchain node configured to accept pubsub requests.
A listener subscribe to the blockchain node and converts the events to a TFHE workload in a database.

There are 3 types of events related to:
- TFHE operations
- ACL, can be used to preprocess ciphertext for certain use case
- Public and User Decryption

### Database Events
> Status: proposal
Database events are used to hint the scheduler to dispath workload and to notice workload completion.

> https://stackoverflow.com/questions/56747634/how-do-i-use-the-postgres-crate-to-receive-table-modification-events-from-postgr

### Decryption Events
> Status: in progress

### Overview FHEVM
> **_NOTE:_**  Listener and scheduler could be in the same service.**
```mermaid
sequenceDiagram
    participant BC App Node
    participant Listener
    participant Scheduler
    participant DB
    participant Coprocessor

    Listener-->>BC App Node: Subscribe Contract Events
    Scheduler-->>DB: Subscribe Computations Insertions/Status<br/>(proposal)

    loop Block Execution - Symbolic Operations
        Note over BC App Node: Solidity traces a Symbolic Sequence
        Note over BC App Node: TFHEExecutor contract
        Note over BC App Node: ACL contract
    end

    Note over BC App Node: End of Block Execution (MAYBE)

    BC App Node-)Listener: TFHE Operations Events
    BC App Node-)Listener: ACL Events

    Listener->>DB: Insert TFHE Operations
    DB-)Scheduler: Notice TFHE Operations Insertions<br/>(proposal)
    Scheduler-)Coprocessor: THFE Operation Workload
    BC App Node-)Listener: Decryption Events

    loop FHE Computation
        Coprocessor -->> DB: Read Operands Ciphertexts
        Note over Coprocessor: TFHE Computation
        Coprocessor -->> DB: Write Result Ciphertext
        Coprocessor-->>DB: Mark TFHE Operation as Done
    end
    DB-)Scheduler: Notice TFHE Operations Status<br/>(proposal)
```

### Overview Relayer (maybe incorrect to be refined)

```mermaid
sequenceDiagram
    participant Relayer
    participant Listener
    participant Scheduler
    participant DB
    participant Coprocessor

    Note over Listener: THEFE Operations Events
    Note over Listener: Decryption Events

    Listener->>DB: Insert TFHE Operations
    Listener->>Relayer: Decryption Workload
    DB-)Scheduler: Notice TFHE Operations Insertions<br/>(proposal)
    Scheduler-)Coprocessor: THEFE Operation Workload

    loop FHE Computation
        Coprocessor -->> DB: Read Operands Ciphertexts
        Note over Coprocessor: TFHE Computation
        Coprocessor -->> DB: Write Result Ciphertexts
        Coprocessor-->>DB: TFHE Operation Done
    end
    DB-)Scheduler: Notice TFHE Operations Status<br/>(proposal)
    Scheduler-)Relayer: Notice Ciphertext ready for decryption
```

