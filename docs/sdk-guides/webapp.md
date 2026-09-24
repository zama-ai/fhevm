# Web applications

{% hint style="warning" %}
The Relayer SDK (`@zama-fhe/relayer-sdk`) has been replaced by [`@fhevm/sdk`](https://www.npmjs.com/package/@fhevm/sdk). This page is no longer maintained.
{% endhint %}

`@fhevm/sdk` has a single build that runs in both the browser and Node.js: there are no `/web`, `/node` or `/bundle` entry points and no CDN bundle, and no `initSDK()` call. Install it from npm and import from `@fhevm/sdk/ethers` or `@fhevm/sdk/viem`.

➡️ See [**Runtime & rendering compatibility**](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/runtime-compatibility.md) in the `@fhevm/sdk` documentation.

Migrating existing code? See [Migrating from the Relayer SDK](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md).
