/* eslint-disable @typescript-eslint/naming-convention */

export type createFhevmClient = () => unknown;

/*

The pattern (for tree shackability):
const c = createFhevm(); // empty, zero code, zero CPU
c.extend(relayer) // imports relayer module, zero CPU
c.extend(encrypt) // imports encrypt module, zero CPU
c.extend(decrypt) // imports decrypt module, zero CPU

Tree shackability issue:
here is the problem:
having a full client implies importing all the code
having a partial client only imports what is necessary

If someone wants to decrypt any value:
- relayer + decrypt modules are required

If someone wants to encrypt any value:
- encrypt module is necessary

If someone wants to encrypt any value and have it ready for use in FHE.sol smart contract:
- relayer + encrypt module is necessary (encrypt: to produce the encrypted data localy, relayer: to submit the encrypted value and get the inputProof for use in FHE.sol)

Maybe, one good approach could be:
- export a single decrypt function (or a set of decrypt functions). Behind the scene it takes the global runtime (which is unique), 
  dynmically adds the necessary modules (here: decrypt module + relayer module)

- export a single encrypt function (or a set of encrypt functions). Behind the scene it takes the global runtime (which is unique), 
  dynmically adds the necessary modules (here: encrypt module + relayer module)
  
// Layer 1 — composable (not a client per-se, this is more a runtime)
const c = createEmptyFhevm({ ... })
  .extend(encryptModule) // add encrypt library
  .extend(decryptModule) // add decrypt library
  .extend(relayerModule) // add relayer library

Definitions:
- fhevmRuntime -> compasable, tree-shackable set of functions

Definitions:
- fhevmClient = 1 fhevmRuntime + 1 chain-provider/client ( 1 viem client+chain or 1 ethers provider+chain)
    an fhevmClient is composable in a sense that it forwards the composability to the underlying runtime
an fhevmClient inherits the features of the underlying runtime


// Layer 2 — pre-composed for common use cases
const c = createFhevmEncryptClient({ ... })   // extend(encrypt) + extend(relayer)
const c = createFhevmDecryptClient({ ... })   // extend(decrypt) + extend(relayer) + extend(tkmsKey)
const c = createFhevmClient({ ... })     // everything

An

Option A: use 'Client' suffix : ok for Viem because there is a notion of Client, not ok for ethers, because the term is provider

Best naming
ethers/
  clients/
    createFhevmEncryptClient.ts //  — encrypt + relayer + globalFhePublicKey
    createFhevmDecryptClient.ts // — decrypt + relayer + tkmsKey
    createFhevmClient.ts // - everything


Option B: no 'Client' suffix

ethers/
  clients/
    createFhevmEncrypt.ts //  — encrypt + relayer
    createFhevmDecrypt.ts // — decrypt + relayer + tkmsKey
    createFhevm.ts // - everything


Best naming
ethers/
  clients/
    createFhevmEncryptClient.ts //  — encrypt + relayer + globalFhePublicKey
    createFhevmDecryptClient.ts // — decrypt + relayer + tkmsKey
    createFhevmClient.ts // - everything

extend() functions + modules/action groups are kept internal    

Now, there is an exception:
- FHE Global public key download, serialization/deserialization need chain in the sense that a Global FHE Public Key is attached to a chain
*/
