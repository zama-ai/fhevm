# Chains

A chain definition tells the SDK which FHEVM contracts and Relayer to talk to. It
is the first argument to every client factory. Definitions come from
`@fhevm/sdk/chains`.

```ts
import { mainnet, sepolia, defineFhevmChain } from '@fhevm/sdk/chains';
```

## Built-in chains

| Export    | Chain            | `id`       | Status     | Relayer                            |
| --------- | ---------------- | ---------- | ---------- | ---------------------------------- |
| `mainnet` | Ethereum mainnet | `1`        | Production | `https://relayer.mainnet.zama.org` |
| `sepolia` | Ethereum Sepolia | `11155111` | Testnet    | `https://relayer.testnet.zama.org` |

Use them directly:

```ts
const client = createFhevmClient({ chain: sepolia, provider });
```

Each definition bundles the host-chain contract addresses, the Relayer URL, and
the gateway coordinates the SDK needs. You never assemble these by hand for a
supported chain.

## The `FhevmChain` shape

A chain definition is a plain, deeply-frozen object:

```ts
type FhevmChain = {
  readonly id: number; // host chain id
  readonly fhevm: {
    readonly contracts: {
      readonly acl: ChainContract;
      readonly inputVerifier: ChainContract;
      readonly kmsVerifier: ChainContract;
      readonly protocolConfig: ChainContract | undefined;
    };
    readonly relayerUrl: string;
    readonly gateway: {
      readonly id: number; // gateway chain id
      readonly contracts: {
        readonly decryption: ChainContract;
        readonly inputVerification: ChainContract;
      };
    };
  };
};

type ChainContract = {
  readonly address: `0x${string}`;
  readonly blockCreated?: number | undefined;
};
```

| Field                                   | Purpose                                                      |
| --------------------------------------- | ----------------------------------------------------------- |
| `id`                                    | The host chain's EVM chain id.                              |
| `fhevm.contracts.acl`                   | Access Control List — tracks who may decrypt what.          |
| `fhevm.contracts.inputVerifier`         | Verifies encrypted inputs and their proofs.                 |
| `fhevm.contracts.kmsVerifier`           | Holds the authorized KMS signer set and quorum threshold.   |
| `fhevm.contracts.protocolConfig`        | Optional aggregate config contract; may be `undefined`.     |
| `fhevm.relayerUrl`                       | The Relayer the SDK sends proof and decryption requests to. |
| `fhevm.gateway.id`                       | The gateway rollup's chain id.                              |
| `fhevm.gateway.contracts.decryption`     | Gateway decryption contract.                                 |
| `fhevm.gateway.contracts.inputVerification` | Gateway input-verification contract.                     |

### Built-in addresses

{% tabs %}
{% tab title="mainnet (id 1)" %}

| Contract                 | Address                                        |
| ------------------------ | ---------------------------------------------- |
| ACL                      | `0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6`   |
| Input Verifier           | `0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2`   |
| KMS Verifier             | `0x77627828a55156b04Ac0DC0eb30467f1a552BB03`   |
| Gateway decryption       | `0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24`   |
| Gateway input verification | `0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287` |

Gateway chain id: `261131`.

{% endtab %}
{% tab title="sepolia (id 11155111)" %}

| Contract                 | Address                                        |
| ------------------------ | ---------------------------------------------- |
| ACL                      | `0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D`   |
| Input Verifier           | `0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0`   |
| KMS Verifier             | `0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A`   |
| Gateway decryption       | `0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478`   |
| Gateway input verification | `0x483b9dE06E4E4C7D35CCf5837A1668487406D955` |

Gateway chain id: `10901`.

{% endtab %}
{% endtabs %}

## Defining a custom chain

For a local devnet or a host chain the SDK doesn't ship, build a definition with
`defineFhevmChain`. It validates the shape, deep-freezes the object, and preserves
its exact type:

```ts
import { defineFhevmChain } from '@fhevm/sdk/chains';

export const myDevnet = defineFhevmChain({
  id: 12345,
  fhevm: {
    contracts: {
      acl: { address: '0x…' },
      inputVerifier: { address: '0x…' },
      kmsVerifier: { address: '0x…' },
      protocolConfig: { address: '0x…' },
    },
    relayerUrl: 'http://localhost:9000',
    gateway: {
      id: 54321,
      contracts: {
        decryption: { address: '0x…' },
        inputVerification: { address: '0x…' },
      },
    },
  },
});

const client = createFhevmClient({ chain: myDevnet, provider });
```

Set `protocolConfig` to `undefined` if your deployment doesn't have one — the
built-in `mainnet` and `sepolia` definitions currently do.

## Related

- [Clients](clients.md) — passing the chain to a factory.
- [Runtime configuration](runtime-configuration.md) — everything not chain-specific.
- [API reference](api-reference.md) — the `FhevmChain` type and `defineFhevmChain` signature.
```

