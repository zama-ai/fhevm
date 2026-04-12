# Problem

- We need an Object FheEncryptionKey
- each FheEncryptionKey object is unique accross the JS V8 runtime
- Multiple FhevmRuntime instances sharing the same relayerUrl get the same FheEncryptionKey object
- The 50MB cost is paid once per relayerUrl, regardless of how many runtimes or clients exist.
- each FheEncryptionKey object is immutable from the outside of the SDK
- embeds a publicKey data either in Bytes form or in Wasm form
- can be serialized from wasm form to a JS compatible format
- can be deserialized from its JS compatible format to bytes form
- if the WASM module is available in the runtime : deserialize in wasm form directly
- the FheEncryptionKey object is the only public-facing object
- Behind the scene, there is a cache global
- the cache key is relayerUrl
- FheEncryptionKey is unique per relayerUrl
- the FheEncryptionKey can secretly mutate into its wasm form at any moment
- The goal of the bytes to wasm xform is to ensure the SDK does not keep 2x50MB bytes of data
- The problem of the current cache approach is that it is not complete: there is no "magical" object that auto-mutates from bytes to wasm. Maybe the user-facing object should be a Wrapper to the cache entry ?
