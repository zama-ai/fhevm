# How can I break a loop ?

❌ In FHE, it is not possible to break a loop based on an encrypted condition. For example, this would not work:

```solidity
euint8 maxValue = TFHE.asEuint(6); // Could be a value between 0 and 10
euint8 x = TFHE.asEuint(0);
// some code
while(TFHE.lt(x, maxValue)){
    x = TFHE.add(x, 2);
}
```

If your code logic requires looping on an encrypted boolean condition, we highly suggest to try to replace it by a finite loop with an appropriate constant maximum number of steps and use `TFHE.select` inside the loop.

✅ For example, the previous code could maybe be replaced by the following snippet:

```solidity
euint8 maxValue = TFHE.asEuint(6); // Could be a value between 0 and 10
euint8 x;
// some code
for (uint32 i = 0; i < 10; i++) {
    euint8 toAdd = TFHE.select(TFHE.lt(x, maxValue), 2, 0);
    x = TFHE.add(x, toAdd);
}
```

In this snippet, we perform 10 iterations, adding 4 to `x` in each iteration as long as the iteration count is less than `maxValue`. If the iteration count exceeds `maxValue`, we add 0 instead for the remaining iterations because we can't break the loop.
