# Input

{% hint style="warning" %}
The Relayer SDK (`@zama-fhe/relayer-sdk`) has been replaced by [`@fhevm/sdk`](https://www.npmjs.com/package/@fhevm/sdk). This page is no longer maintained.
{% endhint %}

Encrypted inputs are now created with a single declarative call, `client.encryptValues({ contractAddress, userAddress, values: [{ type, value }] })`, instead of the `createEncryptedInput().add32(…).encrypt()` builder.

➡️ See [**Encryption**](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/encryption.md) in the `@fhevm/sdk` documentation.

Migrating existing code? See [Migrating from the Relayer SDK](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md).
