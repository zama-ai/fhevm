# Public Decryption

Public decryption can be done with either the relayer HTTP endpoint or calling the on-chain decryption oracle.

## HTTP Public Decrypt

```ts
    // A list of ciphertexts handles to decrypt
    const handles = ["0x830a61b343d2f3de67ec59cb18961fd086085c1c73ff0000000000aa36a70000", "0x98ee526413903d4613feedb9c8fa44fe3f4ed0dd00ff0000000000aa36a70400", "0xb837a645c9672e7588d49c5c43f4759a63447ea581ff0000000000aa36a70700"];

    // The list of decrypted values
    const values = instance.publicDecrypt(
      handles
    );
```

## On-chain Public Decrypt

For more details please refer to the on [on-chain Oracle public decryption page](../solidity-guides/decryption/oracle.md).

