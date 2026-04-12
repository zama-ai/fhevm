- Runtime
  - Modules
  - <FunctionName>Parameters
  - <FunctionName>ReturnType
    - <functionName>(runtime, parameters: <FunctionName>Parameters): <FunctionName>ReturnType
    - <functionName>(runtime, parameters: <FunctionName>Parameters): Promise<<FunctionName>ReturnType>
- Client
  - <FunctionName>Parameters
  - <FunctionName>ReturnType
  - Function signature:
    - <functionName>(client, parameters: <FunctionName>Parameters): <FunctionName>ReturnType
    - <functionName>(client, parameters: <FunctionName>Parameters): Promise<<FunctionName>ReturnType>

# SDK Design Principles

- Order-independant API
- Composable Extensions
- Composable runtime modules
- No required init step (explicit or lazy)
- Initialization is Lazy, Idempotent, Shared
- Initialization supports multiple concurrent calls
- Configuration is chainable and order-independent
- SDK lifecycle must be predictable
- SDK must not cause ordering footguns
- Zero config must work
- Lazy init or Explicity init must be supported
- For encryption: the publicKey is provided or resolvable
- Must offer flexible chaining
- Perform runtime check
- Validation is performed at runtime
- Avoid over-constraining Typescript
- Use share mutable context when needed
- Resolve config at call time
- Must be treeshackable
- Throw clear error messages when something is missing or bad config
- Extensions must not capture config at creation time
- Configuration should be resolved at call time, not at creation time
- SDK must be flexible, treeshackable, and composable, idempotent and safe (no silent misuse)
- Creation must be pure
- extensions must be reusable
- Supports Lazy+Async flows
- SDK should be scalable to more configs
- Pattern: Explicit, composable, chainable, no constructor explosion, works with partial clients
- The clients should carry a shared internal context, and extensions read from it
- SDK design: initialization + dependency orchestration problem: Make the default path Lazy and automatic, but allow explicit control for power users
- API Shape: 1. The "it just works" default (Everything is memoized), 2. Optional explicit configutation, 3. Power-user explicit init (optional but important) (this is useful for preloading, avoiding latency spikes, SSR/controlled environments)
- Key design decisions: no async at construction, first usage triggers everything, Clean UX/DX, idempotent init (very important): init() can be called manually or internally, always returns the same promise
- extend() is always allowed
- init() is always allowed, always idempotent
- Init functions must be globally unique (by reference) and internally idempotent
- SDK constraints: encryptModule and decryptModule are 2 independent modules that are interacting with 2 different WASM modules (1 for each). It is required to avoid loading a given WASM module if not necessary (therefore the SDK uses a extension principle).
- When a partial client is created, the sdk should allow the possibility to extend it to a full client

# SDK Architecture

## runtime

- a composable runtime (FhevmRuntime)
- multiple runtimes could potentially cohexist
- a runtime is composed of a list of modules.
- each module can be dynamically added
- some modules can be unique in the Javascript runtime (for example WASM modules)
- Two different runtimes could potentially share the same modules
- Example of runtimes: a production runtime and a mock runtime. These are 2 different runtimes that could potentially coexist.
- A runtime as a list of preloaded modules and a list of optional modules.
- The design is guided by the need of composability and tree-shackability
- Runtime creation must be pure
- Runtime extension must be pure
- Each module may potentially need a CPU intensive initialization step.
- Init is always: idempotent, lazy or manual.

## clients

- Fhevm Clients
- each fhevm client has a runtime
- a runtime can be shared by multiple clients
- a client is essentially: a runtime + a set of additional parameters to enable specific features
- a client is extensible
- Given clientA and clientB it should always be possible to extend clientA and/or clientB so that clientA == clientB
- Average SDK user manipulates clients
- Runtime should remain an internal component
- call fhevmClient.extend(...) to extend a client with new functions
- call fhevmClient.withXXX(...) to set a client config parameter
- a fhevmClient must always be initialized after creation or extension (this will initialize the new runtime modules if needed)
- fhevmClient initialization is optional. If not called, the initialization will automatically be performed at runtime: "at first call"

```ts
// full client (chain, provider, encrypt module, decrypt module)
const fhevmFull = createFhevmClient({ chain, provider });

// partial decrypt client (chain, provider, decrypt module, no encrypt module)
const fhevmDecrypt = createFhevmDecryptClient({ chain, provider });

// partial encrypt client (chain, provider, encrypt module, no decrypt module)
const fhevmEncrypt = createFhevmEncryptClient({ chain, provider });
// create with optional publicKeyBytes,
const fhevmEncrypt = createFhevmEncryptClient({
  chain,
  provider,
  publicKeyBytes,
});

// Problem: how to get the publicKeyBytes ?
// publicKeyBytes can be fetched independently

// Convert partial client to full client
const fhevmFull = fhevmEncrypt.extend(decryptActions);

// decryptActions is a function that takes the client as argument and returns an object that consists in a group of functions
// with client captured
// Function groups usually depends on modules that must be extented to the client runtime to run properly
// For example the decryptActions needs the decryptModule.
// after client extend, the client must be initialized again (because the underlying new decryptModule needs to be initialized)

// decryptActions should automatically extend the module
// The encrypt module requires the global FHE public key

fhevmEncrypt = fhevmEncrypt.withPublicKey(publicKeyBytes);
await fhevmEncrypt.fetchPublicKey();

// returns a promise (eq to { return init(); })
await fhevmEncrypt.ready;

// manual init call (fetch key if needed)
await fhevmEncrypt.init();

// any call to withPublicKey or fetchPublicKey should throw an error after init()
// if init is not called the first call to encrypt will call init()
```
