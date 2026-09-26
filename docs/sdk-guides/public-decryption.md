# Public decryption

{% hint style="warning" %}
The Relayer SDK (`@zama-fhe/relayer-sdk`) has been replaced by [`@fhevm/sdk`](https://www.npmjs.com/package/@fhevm/sdk). This page is no longer maintained.
{% endhint %}

Public decryption now uses `client.decryptPublicValues(…)`. To get the KMS proof needed by `FHE.checkSignatures` on-chain, use `client.decryptPublicValuesWithSignatures(…)`.

➡️ See [**Public decryption**](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/decryption.md#public-decryption) in the `@fhevm/sdk` documentation.

Migrating existing code? See [Migrating from the Relayer SDK](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md).
