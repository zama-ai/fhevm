# Estimate gas

FHE operations are typically more computationally expensive than classical operations due to their inherent complexity. As a reference, here is an approximation of the gas cost associated with each operation.

| function name       | ebool/euint8 | euint16   | euint32   |
| ------------------- | ------------ | --------- | --------- |
| `TFHE.add`          | 86,000       | 112,000   | 134,000   |
| `TFHE.sub`          | 86,000       | 111,000   | 134,000   |
| `TFHE.mul`          | 154,000      | 205,000   | 276,000   |
| `TFHE.div`          | 1,393,000    | 3,557,000 | 9,266,000 |
| `TFHE.rem`          | 1,393,000    | 3,557,000 | 9,266,000 |
| `TFHE.and`          | 22,000       | 23,000    | 25,000    |
| `TFHE.or`           | 22,000       | 23,000    | 25,000    |
| `TFHE.xor`          | 22,000       | 23,000    | 25,000    |
| `TFHE.shr`          | 108,000      | 131,000   | 164,000   |
| `TFHE.shl`          | 108,000      | 131,000   | 164,000   |
| `TFHE.eq`           | 64,000       | 86,000    | 113,000   |
| `TFHE.ne`           | 64,000       | 86,000    | 113,000   |
| `TFHE.ge`           | 64,000       | 86,000    | 113,000   |
| `TFHE.gt`           | 64,000       | 86,000    | 113,000   |
| `TFHE.le`           | 64,000       | 86,000    | 113,000   |
| `TFHE.lt`           | 64,000       | 86,000    | 113,000   |
| `TFHE.min`          | 111,000      | 138,000   | 155,000   |
| `TFHE.max`          | 111,000      | 138,000   | 155,000   |
| `TFHE.neg`          | 85,000       | 111,000   | 133,000   |
| `TFHE.not`          | 85,000       | 111,000   | 133,000   |
| `TFHE.cmux`         | 382,000      | 483,000   | 598,000   |
| `TFHE.decrypt()`    | 2000         | 2000      | 2000      |
| `TFHE.randEuintX()` | 22,000       | 23,000    | 24,000    |

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
