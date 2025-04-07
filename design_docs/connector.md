# Quick Intro

The old KMS Connector architecture was designed with a simple Cosmos blockchain and KV storage interface in mind. However, integrating with Arbitrum L2 introduces new requirements and complexities that the old design cannot efficiently handle.

Visually Connector methods-oriented current (old) arch design may be presented like this, roughly:

```plantuml
@startuml kms-connector-old-design
skinparam PackageBorderColor white
skinparam PackageBackgroundColor lightgrey
skinparam PackageStyle rectangle
skinparam ClassStyle rectangle
skinparam classFontStyle bold
skinparam classAttributeFontStyle normal

package "Gateway L2 Interfaces" {
    interface "ICiphertextStorage" as IStorage {
        +{abstract} get_ciphertext(handle: Vec<u8>): Result<Vec<u8>>
    }

    interface "IBlockchain" as IBlockchain {
        +{abstract} send_result(result: KmsOperationResponse): Result<()>
        +{abstract} get_operation_value(event: &KmsEvent): Result<OperationValue>
        +{abstract} get_fhe_parameter(): Result<FheParameter>
        +{abstract} get_public_key(): PublicKey
    }
}

package "KMS Core" {
    class "KmsCore<S: Storage>" as KmsCore {
        -storage: S
        -channel: Channel
        -metrics: Arc<OpenTelemetryMetrics>
        +new(config: CoreConfig, storage: S, metrics): Result<Self>
        +run_operation(fhe_parameter: Option<FheParameter>): Result<...>
        +poll_for_result(input: GenericPollerInput): Result<...>
    }
}

package "KMS Connector" #PaleGoldenRod {
    package "Core" {
        class "KmsCoreConnector" as Connector {
            -blockchain: B
            -kms: K
            -metrics: O
            -sharding: ShardingConfig
            +new_with_config(config: ConnectorConfig): Result<Self>
            +listen_for_events(catch_up_blocks: Option<usize>)
            +get_op_and_fhe_parameter(event: &KmsEvent): Result<...>
        }
    }

    package "Gateway L2 Adapter" {
        class "KmsBlockchain" as KmsBlockchain {
            -config: BlockchainConfig
            -metrics: OpenTelemetryMetrics
            +new(config: BlockchainConfig, metrics): Result<Self>
            +send_result(result: KmsOperationResponse): Result<()>
            +get_operation_value(event: &KmsEvent): Result<OperationValue>
            +get_fhe_parameter(): Result<FheParameter>
            +get_public_key(): PublicKey
            __
            Responsibilities:
            * Listen for blockchain events
            * Send operation results
            * Query blockchain state
        }
    }

    package "Storage Adapter" {
        class "KVStore" as KVStore {
            -config: StoreConfig
            +new(config: StoreConfig): Self
            +get_ciphertext(handle: Vec<u8>): Result<Vec<u8>>
            __
            Implementation details:
            * HTTP-based storage access
            * Hex encoding for identifiers
            * Timeout handling
        }
    }

    package "Operations" {
        class "KmsOperations" as Ops {
            +ReencryptOperation
            +DecryptOperation
            +VerifyProvenCtOperation
            __
            Each operation:
            * Fetches required ciphertexts
            * Processes cryptographic operations
            * Returns operation results
        }
    }
}

' Implementations
KVStore ..|> IStorage
KmsBlockchain ..|> IBlockchain

' Core relationships
Connector --> KmsCore: uses for cryptographic operations
Connector --> KmsBlockchain: uses for blockchain interaction
Connector ..> KVStore: creates and configures
KmsCore --> IStorage: uses for data access
Ops --> KmsCore: executed through

note right of Connector
  Main workflow:
  1. Receives blockchain events
  2. Fetches operation details
  3. Executes via KMS Core
  4. Returns results to blockchain
end note

note right of KmsCore
  Operation flow:
  1. Receives operation request
  2. Fetches required data via Storage
  3. Processes cryptographic operation
  4. Returns operation result
end note

@enduml
```

After analyzing the codebase, it is recommended to implement a brand new connector design rather than refactoring the old one for the following reasons:

1. **Structural / Functional Differences**:

    - **Old**: Simple layered architecture with basic interfaces and a lot of redundant components (KVStore, Cosmos components penetrating a lot of Connector structs, outdated io logic, etc)
    - **New**: Complex adapter-based design with multiple specialized components. Need to follow HTTPZ logic and naming conventions
    - **Verdict**: Too many fundamental structural changes needed

2. **Code Organization**:

    - **Old**: Monolithic components with tight coupling
    - **New**: Modular adapters with clear boundaries
    - **Verdict**: Complete reorganization required. It's very hard to squeeze new implementation into existing code structure

3. **Multi-Contract Management**:

    - **Old**: Single contract interface
    - **New**: Coordinated interaction with multiple specialized contracts each having its own interface:
        - Decryption Manager
        - HTTPZ Protocol

4. **Event-Driven Architecture Requirements**:

    - **New**: Asynchronous event handling with MPSC-based orchestration for multiple contract events. It's suggested to have an improved version of shared Orchestrator component as developed by GW team.

## **Recommended Solution**

1. Create new crate `kms-connector/`
2. Implement new architecture from scratch (see images bellow)
3. Keep both (Old/New) connectors code in the kms-core repo during transition
4. Deprecate Old connector once migration is complete

This is a KMS Connector-focused extract from the general system diagram as presented here (but with corrections for L2 storage) [SEE IMAGE HERE](https://github.com/zama-ai/fhevm-relayer/blob/main/out/design-docs/full-architecture-highe-level-interactions/gateway-external-connections.svg)

```plantuml

@startuml kms-connector-general-design

skinparam PackageBorderColor white
skinparam PackageBackgroundColor lightgrey
skinparam PackageStyle rectangle
skinparam ComponentStyle rectangle

package "Gateway L2" {
    component "Decryption Manager" as gatewayL2_decryptionManager
    component "HTTPZ Protocol" as gatewayL2_httpz
    component "Access Control" as gatewayL2_accessControl
    component "RPC" as gatewayL2_Rpc
    () "WebSocket" as gatewayL2_ws
    () "HTTP" as gatewayL2_http
}

package "Kms Core" as kmsCore {
    component "Keygen" as kmsCore_keygen
    component "CRS Gen" as kmsCore_crsgen
    component "Decryption" as kmsCore_decryption
    component "API" as kmsCore_api
    () "gRPC" as kmsCore_grpc
}

package "KMS connector" #PaleGoldenRod {
    package "Gateway L2 Adapter" {
        component "Event Listener" as connector_eventListener
        component "Tx Sender" as connector_txSender
    }

    package "Kms Core Adapter" {
        component "Decrypt" as connector_decrypt
        component "Keygen" as connector_keygen
        component "CRS Gen" as connector_crsgen
    }
}

' Internal Gateway L2 connections
gatewayL2_Rpc --- gatewayL2_http
gatewayL2_Rpc --- gatewayL2_ws
gatewayL2_decryptionManager --- gatewayL2_Rpc
gatewayL2_httpz --- gatewayL2_Rpc
gatewayL2_accessControl --- gatewayL2_Rpc

' KMS Core internal connections
kmsCore_api -- kmsCore_grpc
kmsCore_keygen -- kmsCore_api
kmsCore_crsgen -- kmsCore_api
kmsCore_decryption -- kmsCore_api

' KMS Connector to Gateway L2 connections
connector_eventListener ..> gatewayL2_ws: Listen for events
connector_txSender --> gatewayL2_http: Send transactions

' KMS Connector to KMS Core connections
connector_decrypt --> kmsCore_grpc: Decrypt requests
connector_keygen --> kmsCore_grpc: Key generation
connector_crsgen --> kmsCore_grpc: CRS generation

note right of connector_eventListener
  Listens for:
  - Decryption requests
  - Key generation events
  - CRS generation events
end note

note right of connector_txSender
  Sends:
  - Operation results
  - Key generation results
  - CRS generation results
  - Error responses
end note

note right of gatewayL2_httpz
  Handles:
  - Key generation protocol
  - CRS generation protocol
  - Validation & verification
end note

@enduml

```

Detailed KMS Connector diagram reflecting current L2 smart-contract interfaces (pls note that `reencryption` is renamed to `userDecryption`) Also iHTTPZ SC interface is about to be finalized... so I put my vision to be clarified once finalization is done

```plantuml

@startuml kms-connector-updated-design

skinparam PackageBorderColor white
skinparam PackageBackgroundColor lightgrey
skinparam PackageStyle rectangle
skinparam ClassStyle rectangle
skinparam classFontStyle bold
skinparam classAttributeFontStyle normal
skinparam linetype ortho
skinparam nodesep 100
skinparam ranksep 100

' Top Layer - Core
package "KMS Connector Core" #PaleGoldenRod {
    class KmsCoreConnector {
        -arbitrumProvider: Provider
        -kmsClient: IKmsCore
        -metrics: MetricsCollector
        -decryptionAdapter: DecryptionAdapter
        -httpzAdapter: HTTPZAdapter
        -mpsc: Orchestrator
        +new(config: ConnectorConfig): Self
        +start(): void
        +stop(): void
        -handlePublicDecryption(event: Event): void
        -handleUserDecryption(event: Event): void
        -handleKeyGeneration(event: Event): void
        -handleCrsGeneration(event: Event): void
    }
}

' Second Layer - Operations
package "KMS Core Adapter" #PaleGoldenRod {
    class PublicDecryptionOperation {
        -accessControl: AccessControlReader
        -storage: CiphertextStorageAdapter
        +execute(request: Request): Result
    }
    
    class UserDecryptionOperation {
        -accessControl: AccessControlReader
        -storage: CiphertextStorageAdapter
        +execute(request: Request): Result
        -validateUserAccess(ctHandle: uint256,\n  user: address, signature: bytes): bool
    }

    class KeyGenerationOperation {
        -httpz: HTTPZAdapter
        +execute(request: KeyGenRequest): Result
        -signPublicKey(publicKey: bytes): bytes
        -preprocessKey(fheParams: FheParams): uint256
    }

    class CrsGenerationOperation {
        -httpz: HTTPZAdapter
        +execute(request: CrsGenRequest): Result
        -signCrs(crs: bytes): bytes
        -validateParameters(params: FheParameters): bool
    }
}

' Third Layer - Adapters
package "Gateway L2 Adapters" #PaleGoldenRod {
    package "Decryption" {
        class DecryptionAdapter {
            -provider: Provider
            -decryptionManager: Contract
            -accessControl: Contract
            +subscribeToDecryptionEvents(): void
            +sendDecryptionResult(requestId: uint256,\n  result: bytes, signature: bytes): void
            +verifyAccess(ctHandle: uint256, user: address): bool
        }
    }
    package "Key Management" {
        class HTTPZAdapter {
            -provider: Provider
            -httpz: Contract
            +subscribeToKeyGenEvents(): void
            +subscribeToCrsGenEvents(): void
            +subscribeToKskEvents(): void
            +handlePreprocessKeygenRequest(event: Event): void
            +handlePreprocessKskRequest(event: Event): void
            +handleKeygenRequest(event: Event): void
            +handleCrsgenRequest(event: Event): void
            +handleKskRequest(event: Event): void
            +submitPreprocessKeygenResponse(preKeyId: uint256): void
            +submitPreprocessKskResponse(preKskId: uint256): void
            +submitKeygenResponse(keyId: uint256): void
            +submitCrsgenResponse(crsId: uint256): void
            +submitKskResponse(kskId: uint256): void
            +getCurrentKeyId(): uint256
            +isCurrentKeyId(keyId: uint256): bool
            +getKmsIdentities(): bytes[]
            +getCoprocessorIdentities(): bytes[]
        }
    }
}

' Bottom Layer - Smart Contract Interfaces
package "Gateway L2 (Arbitrum) Contracts" {
    interface IDecryptionManager {
        +struct CiphertextContract
        +publicDecryptionRequest(uint256[] ciphertextHandles): void
        +publicDecryptionResponse(uint256 publicDecryptionId,\n  bytes decryptedResult, bytes signature): void
        +userDecryptionRequest(CiphertextContract[] contracts,\n  address user, bytes key, uint256 chainId,\n  address[] contracts, bytes sig): void
        +userDecryptionResponse(uint256 userDecryptionId,\n  bytes decryptedResult, bytes signature): void
    }

    interface IHTTPZ {
        +struct ProtocolMetadata
        +struct KmsNode
        +struct Coprocessor
        +struct Network
        +struct FheParams
        +initialize(metadata: ProtocolMetadata,\n  admins: address[]): void
        +preprocessKeygenRequest(): void
        +preprocessKskRequest(): void
        +keygenRequest(preKeyId: uint256): void
        +crsgenRequest(): void
        +kskRequest(preKskId: uint256,\n  sourceKeyId: uint256,\n  destKeyId: uint256): void
        +activateKeyRequest(keyId: uint256): void
        +preprocessKeygenResponse(preKeyId: uint256): void
        +preprocessKskResponse(preKskId: uint256): void
        +keygenResponse(keyId: uint256): void
        +crsgenResponse(crsId: uint256): void
        +kskResponse(kskId: uint256): void
        +activateKeyResponse(keyId: uint256): void
        +getCurrentKeyId(): uint256
        +isCurrentKeyId(keyId: uint256): bool
        +getKmsNodes(): KmsNode[]
        +getKmsIdentities(): bytes[]
        +getCoprocessors(): Coprocessor[]
        +getCoprocessorIdentities(): bytes[]
    }
}

' External Services
package "External Services" {
    interface IKmsCore {
        +decrypt(bytes ciphertext, bytes proof): Result
        +generateKey(KeyType keyType): Result
        +generateCrs(FheParameters params): Result
        +signPublicKey(bytes publicKey): Result
        +signCrs(bytes crs): Result
    }
}

' Relationships with annotations
KmsCoreConnector --> PublicDecryptionOperation : "executes"
KmsCoreConnector --> UserDecryptionOperation : "executes"
KmsCoreConnector --> KeyGenerationOperation : "executes"
KmsCoreConnector --> CrsGenerationOperation : "executes"
IKmsCore <--> KmsCoreConnector : "KMS crypto ops"

PublicDecryptionOperation --> DecryptionAdapter : "submits results"
UserDecryptionOperation --> DecryptionAdapter : "submits results"
KeyGenerationOperation --> HTTPZAdapter : "manages key operations"
CrsGenerationOperation --> HTTPZAdapter : "manages CRS operations"

DecryptionAdapter <--> IDecryptionManager : "listens for events\nsubmits results"
HTTPZAdapter <--> IHTTPZ : "manages key, CRS, KSK\nand preprocessing"

note left of DecryptionAdapter
  Handles decryption flow:
  - Event subscription
  - Access control
  - Result submission
end note

note right of HTTPZAdapter
  Handles key operations:
  - Key preprocessing
  - Key generation
  - CRS generation
  - KSK operations
  - Key activation
end note

@enduml

```

## Additional Functionality

1. Endpoints shall have ability to notify about request statuses as was developed here: <https://github.com/zama-ai/kms-core/pull/1868>

2. Polling for a new L2 events shall account to Arbitrum soft-finality rules where all operations but for KeyGen shall have almost instant finality (upon tx inclusion into L2 block), while KeyGen tx (due to its price) is suggested to have finality up to 7-8 days (end or arbitrage term).

```plantuml
@startuml Arbitrum with AnyTrust DA

skinparam sequenceMessageAlign center 

actor User
participant Sequencer
participant "Active Validator" as ActiveValidator
participant "Validator (Non-Active)" as Validator
participant "AnyTrust DAC Members" as AnyTrust
participant "Ethereum" as Ethereum

User -> Sequencer: Submit Transaction
Sequencer -> User: Return Receipt <<~250-500ms>>
note over User, Sequencer: **Soft Finality Stage 1**\n- Tx will be included in next L2 Block\n- Tx will be eventually finalized on L2\n- Tx will be eventually finalized on L1\nTime: ~250-500ms

Sequencer -> Validator: Broadcast L2 Block via Sequencer Feed <<~1s>>
Sequencer -> ActiveValidator: Broadcast L2 Block via Sequencer Feed <<~1s>>

Validator -> AnyTrust: Is data for L2 Block available ? <<~2-3s>>
alt
AnyTrust -> Validator: Yes <<~1s>>
Validator -> Validator: Confirm L2 Block
note over User, Ethereum: **Soft Finality Stage 2 (Confirmed)**\n- Txs in the L2 Block are finalized on L2\nTime: ~3-5s from submission
else 
AnyTrust -> Validator: No
Validator -> Validator: L2 chain is halted until Yes is received
end
opt Check for Tx confirmed
User -> Validator: Reads that Tx is confirmed <<~5s total from submission>>
end


ActiveValidator -> ActiveValidator: Collect L2 Blocks\nand form an RBlock <<~1 day>>
ActiveValidator -> Ethereum: Post RBlock to L1 <<~15s>>


note over ActiveValidator, Ethereum: **Settlement Stage**\n- RBlock is registered on L1\n- Fraud proof challenge window starts\nTime: ~1 day from submission

alt Fraud detected in Rblock
    Validator -> Ethereum: Submit fraud proof <<variable>>
    note over ActiveValidator, Ethereum: Interactive fraud proof resolution\n(Up to 7 days)
    Ethereum -> ActiveValidator: Slash bond for fraud
    Ethereum -> Ethereum: Fraudulent L2 Blocks are reverted
    Validator -> Ethereum: Reads the last unreverted L2 Block
    ActiveValidator -> Ethereum: Reads the last unreverted L2 Block
    Validator -> Validator: Resets the L2 chain to this L2 Block
    ActiveValidator -> ActiveValidator: Resets the L2 chain to this L2 Block
    User -> Validator: Reads that Tx was reverted
else Challenge period expired
    Ethereum -> Validator: Finalize RBlock <<after 7 days>>
end

note over User, Ethereum: **Hard finality**\n- Tx is finalized on L1\nTime: ~7-8 days total from submission

@enduml
```

### **Benefits of New Implementation**

- Components and their naming follow HTTPZ workflow doc
- Clean codebase without legacy code
- No risk of breaking existing functionality
- Better testing coverage from start
- Clear separation from old implementation
- Easier to maintain and extend

## Suggested Project Structure Within `kms-core` Repo

```rust
kms-connector/
├── Cargo.toml
├── README.md
└── src/
    ├── core/
    │   ├── mod.rs
    │   ├── connector.rs        // KmsCoreConnector implementation
    │   ├── config.rs           // Configuration structures
    │   └── orchestrator.rs     // MPSC-based event orchestration (might be a module jointly used with the GW)
    │
    ├── kms_core_adapter/
    │   ├── mod.rs
    │   ├── public_decryption.rs
    │   ├── user_decryption.rs
    │   ├── key_generation.rs
    │   └── crs_generation.rs
    │
    ├── gwl2_adapters/
    │   ├── mod.rs
    │   ├── decryption/
    │   │   ├── mod.rs
    │   │   └── adapter.rs      // DecryptionAdapter
    │   └── key_management/
    │       ├── mod.rs
    │       └── httpz.rs        // HTTPZAdapter (keys + CRS)
    │
    ├── gwl2_contracts/        // Alloy bindings
    │   ├── mod.rs
    │   ├── decryption.rs       // IDecryptionManager
    │   └── httpz.rs            // IHTTPZ
    │
    ├── provider.rs             // Arbitrum provider
    ├── error.rs
    ├── types.rs
    └── lib.rs
```
