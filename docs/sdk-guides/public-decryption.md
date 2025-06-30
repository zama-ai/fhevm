# Public Decryption

This document explains how to perform public decryption of FHEVM ciphertexts.
Public decryption is required when you want everyone to see the value in a ciphertext, for example the result of private auction.
Public decryption can be done with either the Relayer HTTP endpoint or calling the on-chain decryption oracle.

## HTTP Public Decrypt

Calling the public decryption endpoint of the Relayer can be done easily using the following code snippet.

```ts
// A list of ciphertexts handles to decrypt
const handles = [
  "0x830a61b343d2f3de67ec59cb18961fd086085c1c73ff0000000000aa36a70000",
  "0x98ee526413903d4613feedb9c8fa44fe3f4ed0dd00ff0000000000aa36a70400",
  "0xb837a645c9672e7588d49c5c43f4759a63447ea581ff0000000000aa36a70700",
];

// The list of decrypted values
// {
//  '0x830a61b343d2f3de67ec59cb18961fd086085c1c73ff0000000000aa36a70000': true,
//  '0x98ee526413903d4613feedb9c8fa44fe3f4ed0dd00ff0000000000aa36a70400': 242n,
//  '0xb837a645c9672e7588d49c5c43f4759a63447ea581ff0000000000aa36a70700': '0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8'
// }
const values = instance.publicDecrypt(handles);
```

## Onchain Public Decrypt

For more details please refer to the on [onchain Oracle public decryption page](https://docs.zama.ai/protocol/solidity-guides/smart-contract/oracle).
