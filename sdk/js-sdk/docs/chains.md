# Chains

Every FHEVM client is bound to a **chain** — a configuration object that tells the SDK which smart contracts to use and where the Relayer lives. The SDK ships with definitions for the currently supported chains, and you can define your own.

## Built-in chains

### Ethereum mainnet

The production FHEVM deployment on Ethereum.

```ts
import { mainnet } from "@fhevm/sdk/chains";
```

| Property | Value |
| --- | --- |
| Chain ID | `1` |
| ACL | `0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6` |
| Input Verifier | `0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2` |
| KMS Verifier | `0x77627828a55156b04Ac0DC0eb30467f1a552BB03` |
| Relayer URL | `https://relayer.mainnet.zama.org` |
| Gateway ID | `261131` |
| Gateway Decryption | `0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24` |
| Gateway Input Verification | `0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287` |

### Sepolia testnet

The test deployment. Use this for development and testing.

```ts
import { sepolia } from "@fhevm/sdk/chains";
```

| Property | Value |
| --- | --- |
| Chain ID | `11155111` |
| ACL | `0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D` |
| Input Verifier | `0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0` |
| KMS Verifier | `0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A` |
| Relayer URL | `https://relayer.testnet.zama.org` |
| Gateway ID | `10901` |
| Gateway Decryption | `0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478` |
| Gateway Input Verification | `0x483b9dE06E4E4C7D35CCf5837A1668487406D955` |

---

## What's in a chain definition

A chain definition tells the SDK everything it needs to talk to the FHEVM infrastructure on a specific network:

```ts
type FhevmChain = {
  readonly id: number;                   // The EVM chain ID (e.g., 1 for mainnet)
  readonly fhevm: {
    readonly contracts: {
      readonly acl: { address: string };           // Who can decrypt what
      readonly inputVerifier: { address: string }; // Verifies encrypted input proofs
      readonly kmsVerifier: { address: string };   // Verifies KMS signatures
    };
    readonly relayerUrl: string;                   // The Relayer HTTP endpoint
    readonly gateway: {
      readonly id: number;                         // The Gateway chain ID
      readonly contracts: {
        readonly decryption: { address: string };         // Routes decryption requests
        readonly inputVerification: { address: string };  // Routes input verification
      };
    };
  };
};
```

**What are these contracts?**

| Contract | What it does |
| --- | --- |
| **ACL** | The Access Control List — tracks which users and contracts can decrypt which handles |
| **Input Verifier** | Verifies that encrypted inputs were created correctly (checks ZK proofs and coprocessor signatures) |
| **KMS Verifier** | Verifies that decryption responses are authentic (checks KMS signatures) |
| **Gateway Decryption** | Routes decryption requests between the chain and the KMS |
| **Gateway Input Verification** | Routes input verification between the chain and the coprocessor |

---

## Custom chains

If you're running your own FHEVM deployment (e.g., a private testnet), you can define a custom chain:

```ts
import { defineFhevmChain } from "@fhevm/sdk/chains";

const myChain = defineFhevmChain({
  id: 8453,
  fhevm: {
    contracts: {
      acl: { address: "0x..." },
      inputVerifier: { address: "0x..." },
      kmsVerifier: { address: "0x..." },
    },
    relayerUrl: "https://relayer.mychain.example.com",
    gateway: {
      id: 99999,
      contracts: {
        decryption: { address: "0x..." },
        inputVerification: { address: "0x..." },
      },
    },
  },
});
```

`defineFhevmChain()` deep-freezes the chain object so it can't be accidentally mutated at runtime.

---

## Reading chain configuration at runtime

The SDK also provides functions to read FHEVM contract state directly from the chain. This is useful for debugging or building tooling — most applications don't need these.

```ts
import {
  readFhevmExecutorContractData,
  readInputVerifierContractData,
  readKmsVerifierContractData,
  resolveFhevmConfig,
} from "@fhevm/sdk/viem"; // or "@fhevm/sdk/ethers"
```

These functions return structured data about the on-chain contracts:

**`FhevmExecutorContractData`** — ACL address, handle version, input verifier address

**`InputVerifierContractData`** — coprocessor signers, signature threshold, EIP-712 domain

**`KmsVerifierContractData`** — KMS signers, signature threshold, Gateway chain ID
