# Public Decryption

Public decryption can be done with either the relayer HTTP endpoint or calling the on-chain decryption oracle.

## HTTP Public Decrypt

```ts
    // A list of ciphertexts handles to decrypt
    const handles = ["0x830a61b343d2f3de67ec59cb18961fd086085c1c73ff0000000000aa36a70000"];

    // The list of decrypted values
    const values = instance.publicDecrypt(
      handles
    );
```

## On-chain Public Decrypt
