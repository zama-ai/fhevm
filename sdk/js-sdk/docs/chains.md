# Chains

## Supported Chains

The SDK ships with two predefined chain definitions:

### Ethereum Mainnet

```ts
import { mainnet } from "@fhevm/sdk/chains";
```

| Property | Value |
|----------|-------|
| Chain ID | `1` |
| ACL Contract | `0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6` |
| Input Verifier | `0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2` |
| KMS Verifier | `0x77627828a55156b04Ac0DC0eb30467f1a552BB03` |
| Relayer URL | `https://relayer.mainnet.zama.org` |
| Gateway ID | `261131` |
| Gateway Decryption | `0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24` |
| Gateway Input Verification | `0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287` |

### Sepolia Testnet

```ts
import { sepolia } from "@fhevm/sdk/chains";
```

| Property | Value |
|----------|-------|
| Chain ID | `11155111` |
| ACL Contract | `0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D` |
| Input Verifier | `0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0` |
| KMS Verifier | `0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A` |
| Relayer URL | `https://relayer.testnet.zama.org` |
| Gateway ID | `10901` |
| Gateway Decryption | `0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478` |
| Gateway Input Verification | `0x483b9dE06E4E4C7D35CCf5837A1668487406D955` |

## Chain Definition Structure

```ts
type FhevmChain = {
  readonly id: number;
  readonly fhevm: {
    readonly contracts: {
      readonly acl: { readonly address: ChecksummedAddress };
      readonly inputVerifier: { readonly address: ChecksummedAddress };
      readonly kmsVerifier: { readonly address: ChecksummedAddress };
    };
    readonly relayerUrl: string;
    readonly gateway: {
      readonly id: number;
      readonly contracts: {
        readonly decryption: { readonly address: ChecksummedAddress };
        readonly inputVerification: { readonly address: ChecksummedAddress };
      };
    };
  };
};
```

### Contract Roles

| Contract | Purpose |
|----------|---------|
| **ACL** | Access Control List — manages who can decrypt which handles |
| **Input Verifier** | Verifies ZK proofs and coprocessor signatures for encrypted inputs |
| **KMS Verifier** | Verifies KMS signatures for decryption responses |
| **Gateway Decryption** | Gateway contract for routing decryption requests |
| **Gateway Input Verification** | Gateway contract for routing input verification |

## Resolving Chain Configuration at Runtime

You can dynamically resolve a chain's FHEVM configuration by reading from the host contracts:

```ts
import { resolveFhevmConfig, readFhevmExecutorContractData } from "@fhevm/sdk/ethers";

// Read executor contract data
const executorData = await readFhevmExecutorContractData(client, {
  address: "0xFhevmExecutorAddress...",
});

// Read full config from contracts
const config = await resolveFhevmConfig(client, {
  contracts: {
    acl: { address: "0x..." },
    fhevmExecutor: { address: "0x..." },
    hcuLimit: { address: "0x..." },
    inputVerifier: { address: "0x..." },
    kmsVerifier: { address: "0x..." },
  },
  gateway: { ... },
});
```

## Host Contract Data Types

### FhevmExecutorContractData

```ts
{
  aclAddress: ChecksummedAddress;
  hcuLimitAddress: ChecksummedAddress;
  inputVerifierAddress: ChecksummedAddress;
  handleVersion: Uint8Number;
}
```

### InputVerifierContractData

```ts
{
  address: ChecksummedAddress;
  eip712Domain: { name, version, chainId, verifyingContract };
  coprocessorSigners: readonly ChecksummedAddress[];
  coprocessorSignerThreshold: Uint8Number;
}
```

### KmsVerifierContractData

```ts
{
  address: ChecksummedAddress;
  eip712Domain: KmsEIP712Domain;
  gatewayChainId: Uint64BigInt;
  kmsSigners: readonly ChecksummedAddress[];
  kmsSignerThreshold: Uint8Number;
  verifyingContractAddressDecryption: ChecksummedAddress;
  has(signer: string): boolean;  // Check if address is a KMS signer
}
```
