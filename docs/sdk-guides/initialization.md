# Initialization

{% hint style="warning" %}
The Relayer SDK (`@zama-fhe/relayer-sdk`) has been replaced by [`@fhevm/sdk`](https://www.npmjs.com/package/@fhevm/sdk). This page is no longer maintained.
{% endhint %}

Setup now uses `setFhevmRuntimeConfig()` and `createFhevmClient({ chain, provider })` from `@fhevm/sdk/ethers` or `@fhevm/sdk/viem`, with built-in chain definitions such as `sepolia` and `mainnet` from `@fhevm/sdk/chains`. See also [Clients](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/clients.md) and [Chains](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/chains.md).

➡️ See [**Getting started**](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/getting-started.md) in the `@fhevm/sdk` documentation.

Migrating existing code? See [Migrating from the Relayer SDK](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md).
