# Estimate gas

FHE operations are typically more computationally expensive than classical operations due to their inherent complexity. As a reference, here is an approximation of the gas cost associated with each operation.

| function name       | ebool/euint8 | euint16 | euint32 |
| ------------------- | ------------ | ------- | ------- |
| `TFHE.add`          | 120,000      | 150,000 | 180,000 |
| `TFHE.add` (scalar) | 120,000      | 150,000 | 180,000 |
| `TFHE.sub`          | 120,000      | 150,000 | 180,000 |
| `TFHE.sub` (scalar) | 120,000      | 150,000 | 180,000 |
| `TFHE.mul`          | 200,000      | 260,000 | 380,000 |
| `TFHE.mul` (scalar) | 135,000      | 140,000 | 170,000 |
| `TFHE.div` (scalar) | 450,000      | 500,000 | 550,000 |
| `TFHE.rem` (scalar) | 450,000      | 500,000 | 550,000 |
| `TFHE.and`          | 30,000       | 33,000  | 36,000  |
| `TFHE.or`           | 30,000       | 33,000  | 36,000  |
| `TFHE.xor`          | 30,000       | 33,000  | 36,000  |
| `TFHE.shr`          | 150,000      | 180,000 | 210,000 |
| `TFHE.shr` (scalar) | 32,000       | 32,000  | 32,000  |
| `TFHE.shl`          | 150,000      | 180,000 | 210,000 |
| `TFHE.shl` (scalar) | 32,000       | 32,000  | 32,000  |
| `TFHE.eq`           | 56,000       | 67,000  | 89,000  |
| `TFHE.ne`           | 56,000       | 67,000  | 89,000  |
| `TFHE.ge`           | 56,000       | 67,000  | 89,000  |
| `TFHE.gt`           | 56,000       | 67,000  | 89,000  |
| `TFHE.le`           | 56,000       | 67,000  | 89,000  |
| `TFHE.lt`           | 56,000       | 67,000  | 89,000  |
| `TFHE.min`          | 220,000      | 280,000 | 340,000 |
| `TFHE.max`          | 220,000      | 280,000 | 340,000 |
| `TFHE.neg`          | 29,000       | 31,000  | 33,000  |
| `TFHE.not`          | 29,000       | 31,000  | 33,000  |
| `TFHE.cmux`         | 60,000       | 65,000  | 70,000  |
| `TFHE.decrypt()`    | 500,000      | 500,000 | 500,000 |
| `TFHE.randEuintX()` | 100,000      | 100,000 | 100,000 |

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
