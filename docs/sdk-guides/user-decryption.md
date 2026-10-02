# User decryption

{% hint style="warning" %}
The Relayer SDK (`@zama-fhe/relayer-sdk`) has been replaced by [`@fhevm/sdk`](https://www.npmjs.com/package/@fhevm/sdk). This page is no longer maintained.
{% endhint %}

User decryption now uses `client.generateTransportKeyPair()`, `client.signDecryptionPermit(…)` and `client.decryptValue(…)` / `client.decryptValues(…)`.

➡️ See [**Private decryption**](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/decryption.md#private-decryption) in the `@fhevm/sdk` documentation.

Migrating existing code? See [Migrating from the Relayer SDK](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md).
