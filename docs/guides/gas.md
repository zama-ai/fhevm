# Gas estimation

FHE operations are typically more computationally expensive than classical operations due to their inherent complexity. As a reference, here is an approximation of the gas cost associated with each operation.

## ebool

| Function name    | Gas     |
| ---------------- | ------- |
| `and`/`or`/`xor` | 26,000  |
| `not`            | 30,000  |
| `decrypt`        | 500,000 |

## euint4

| function name        | Gas     |
| -------------------- | ------- |
| `add`/`sub`          | 65,000  |
| `add`/`sub` (scalar) | 65,000  |
| `mul`                | 150,000 |
| `mul` (scalar)       | 88,000  |
| `div` (scalar)       | 139,000 |
| `rem` (scalar)       | 286,000 |
| `and`/`or`/`xor`     | 32,000  |
| `shr`/`shl`          | 116,000 |
| `shr`/`shl` (scalar) | 35,000  |
| `eq`/`ne`            | 51,000  |
| `ge`/`gt`/`le`/`lt`  | 70,000  |
| `min`/`max`          | 121,000 |
| `min`/`max` (scalar) | 121,000 |
| `neg`                | 60,000  |
| `not`                | 33,000  |
| `select`             | 45,000  |
| `decrypt`            | 500,000 |
| `randEuintX()`       | 100,000 |

## euint8

| Function name        | Gas     |
| -------------------- | ------- |
| `add`/`sub`          | 94,000  |
| `add`/`sub` (scalar) | 94,000  |
| `mul`                | 197,000 |
| `mul` (scalar)       | 159,000 |
| `div` (scalar)       | 238,000 |
| `rem` (scalar)       | 460,000 |
| `and`/`or`/`xor`     | 34,000  |
| `shr`/`shl`          | 133,000 |
| `shr`/`shl` (scalar) | 35,000  |
| `eq`/`ne`            | 53,000  |
| `ge`/`gt`/`le`/`lt`  | 82,000  |
| `min`/`max`          | 128,000 |
| `min`/`max` (scalar) | 128,000 |
| `neg`                | 95,000  |
| `not`                | 34,000  |
| `select`             | 47,000  |
| `decrypt`            | 500,000 |
| `randEuintX()`       | 100,000 |

## euint16

| function name        | euint16 |
| -------------------- | ------- |
| `add`/`sub`          | 133,000 |
| `add`/`sub` (scalar) | 133,000 |
| `mul`                | 262,000 |
| `mul` (scalar)       | 208,000 |
| `div` (scalar)       | 314,000 |
| `rem` (scalar)       | 622,000 |
| `and`/`or`/`xor`     | 34,000  |
| `shr`/`shl`          | 153,000 |
| `shr`/`shl` (scalar) | 35,000  |
| `eq`/`ne`            | 54,000  |
| `ge`/`gt`/`le`/`lt`  | 105,000 |
| `min`/`max`          | 153,000 |
| `min`/`max` (scalar) | 150,000 |
| `neg`                | 131,000 |
| `not`                | 35,000  |
| `select`             | 47,000  |
| `decrypt`            | 500,000 |
| `randEuintX()`       | 100,000 |

## euint32

| Function name        | Gas fee |
| -------------------- | ------- |
| `add`/`sub`          | 162,000 |
| `add`/`sub` (scalar) | 162,000 |
| `mul`                | 359,000 |
| `mul` (scalar)       | 264,000 |
| `div` (scalar)       | 398,000 |
| `rem` (scalar)       | 805,000 |
| `and`/`or`/`xor`     | 35,000  |
| `shr`/`shl`          | 227,000 |
| `shr`/`shl` (scalar) | 35,000  |
| `eq`/`ne`            | 82,000  |
| `ge`/`gt`/`le`/`lt`  | 128,000 |
| `min`/`max`          | 183,000 |
| `min`/`max` (scalar) | 164,000 |
| `neg`                | 160,000 |
| `not`                | 36,000  |
| `select`             | 50,000  |
| `decrypt`            | 500,000 |
| `randEuintX()`       | 100,000 |

## euint64

| Function name        | Gas fee   |
| -------------------- | --------- |
| `add`/`sub`          | 188,000   |
| `add`/`sub` (scalar) | 188,000   |
| `mul`                | 641,000   |
| `mul` (scalar)       | 356,000   |
| `div` (scalar)       | 584,000   |
| `rem` (scalar)       | 1,095,000 |
| `and`/`or`/`xor`     | 38,000    |
| `shr`/`shl`          | 210,000   |
| `shr`/`shl` (scalar) | 38,000    |
| `eq`/`ne`            | 86,000    |
| `ge`/`gt`/`le`/`lt`  | 156,000   |
| `min`/`max`          | 210,000   |
| `min`/`max` (scalar) | 192,000   |
| `neg`                | 199,000   |
| `not`                | 37,000    |
| `select`             | 53,000    |
| `decrypt`            | 500,000   |

## eaddress

| Function name | Gas fee |
| ------------- | ------- |
| `eq`/`ne`     | 90,000  |

## Estimate gas

When you call estimate gas method, we can’t determine accurately the gas usage if your function uses `TFHE.decrypt`. During gas estimation, all `TFHE.decrypt()` will return `1`.

### What does it mean?

- `require(TFHE.decrypt(ebool));` will be ok but `require(!TFHE.decrypt(ebool));` will fail during estimation (revert transaction)
- A loop, where you expect a decrypt to be false to break, will never end in gas estimate method (and fails), since the decrypt will always return `1` (true)
- On the other hand, if your loop should last 2 or 3 cycles, until the value is 1, the estimation will be below.
- If you have branches (if/else) based on a decryption, the estimation will use the branch running when the decryption is `1`

While it’s challenging to accurately estimate gas consumption when using `TFHE.decrypt`, we strongly encourage you to take this into consideration.

### What can I do?

A possible solution is to overestimate your gas estimation. You can take this function (with ethers.js) as an example where we multiply the gas limit by `1.2`.

```typescript
export const createTransaction = async <A extends [...{ [I in keyof A]-?: A[I] | Typed }]>(
  method: TypedContractMethod<A>,
  ...params: A
) => {
  const gasLimit = await method.estimateGas(...params);
  const updatedParams: ContractMethodArgs<A> = [
    ...params,
    { gasLimit: Math.min(Math.round(+gasLimit.toString() * 1.2), 10000000) },
  ];
  return method(...updatedParams);
};
```

## Gas limit

The current devnet has a gas limit of **10,000,000**. If you send a transaction exceeding this limit, it won't be executed. Consequently, your wallet won't be able to emit a new transaction. To address this, emit a new transaction with the same nonce but the correct gas limit.
In Metamask, you can enforce the use of a specific nonce by enabling the feature in 'Advanced Settings'.
