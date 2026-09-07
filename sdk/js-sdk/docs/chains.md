# Chains

A chain definition tells the SDK which FHEVM contracts and Relayer to talk to. It
is the first argument to every client factory. Definitions come from
`@fhevm/sdk/chains`.

```ts
import { mainnet, sepolia, polygon, polygonAmoy, defineFhevmChain } from '@fhevm/sdk/chains';
```

## Built-in chains

| Export        | Chain            | `id`       | Status     | Relayer                            |
| ------------- | ---------------- | ---------- | ---------- | ---------------------------------- |
| `mainnet`     | Ethereum mainnet | `1`        | Production | `https://relayer.mainnet.zama.org` |
| `sepolia`     | Ethereum Sepolia | `11155111` | Testnet    | `https://relayer.testnet.zama.org` |
| `polygon`     | Polygon mainnet  | `137`      | Production | `https://relayer.mainnet.zama.org` |
| `polygonAmoy` | Polygon Amoy     | `80002`    | Testnet    | `https://relayer.testnet.zama.org` |

`polygon` shares its Relayer and gateway coordinates with `mainnet`;
`polygonAmoy` shares its Relayer and gateway coordinates with `sepolia` —
Polygon and Ethereum are different host chains talking to the same
protocol-side infrastructure.

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
| Protocol Config          | `0xD8236B57394f90726b26aB25D38CeAC776E1a7C4`   |
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
| Protocol Config          | `0x51f9AFBc89Ea792e1a21a12AB802ab58D4dbee83`   |
| Gateway decryption       | `0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478`   |
| Gateway input verification | `0x483b9dE06E4E4C7D35CCf5837A1668487406D955` |

Gateway chain id: `10901`.

{% endtab %}
{% tab title="polygon (id 137)" %}

| Contract                 | Address                                        |
| ------------------------ | ---------------------------------------------- |
| ACL                      | `0x6737F17e31cf26a1b62fb0362acC5a16CB156F49`   |
| Input Verifier           | `0xf40BD204B035522EaAc8E5afAdc55113Acac96ca`   |
| KMS Verifier             | `0x14e609595474874Dd6b6128376E336EfADfdBE37`   |
| Protocol Config          | `0x17f62Ab3A1Ea519703cD597410147A30Fa1a7f1e`   |
| Gateway decryption       | `0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24`   |
| Gateway input verification | `0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287` |

Gateway chain id: `261131` (same gateway as `mainnet`).

{% endtab %}
{% tab title="polygonAmoy (id 80002)" %}

| Contract                 | Address                                        |
| ------------------------ | ---------------------------------------------- |
| ACL                      | `0xD99Cb9Fc3c42c87f2A4A12e8Fd60318d6bDdf985`   |
| Input Verifier           | `0x6e5A7D8b0c645467Cba7e62D6624917085118631`   |
| KMS Verifier             | `0xCD1D89E311bce4C8DEa9a0857a0c9A4E153D4041`   |
| Protocol Config          | `0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F`   |
| Gateway decryption       | `0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478`   |
| Gateway input verification | `0x483b9dE06E4E4C7D35CCf5837A1668487406D955` |

Gateway chain id: `10901` (same gateway as `sepolia`).

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

Set `protocolConfig` to `undefined` if your deployment doesn't have one. All
four built-in chains now define it.

## Related

- [Clients](clients.md) — passing the chain to a factory.
- [Runtime configuration](runtime-configuration.md) — everything not chain-specific.
- [API reference](api-reference.md) — the `FhevmChain` type and `defineFhevmChain` signature.
```

