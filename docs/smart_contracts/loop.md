# If sentences

This document explains how to handle loops when working with Fully Homomorphic Encryption (FHE), specifically when a loop break is based on an encrypted condition.

## Breaking a loop

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

## Suggested approach

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

## Best practices

### Obfuscate branching

The previous paragraph emphasized that branch logic should rely as much as possible on `TFHE.select` instead of decryptions. It hides effectively which branch has been executed.

However, this is sometimes not enough. Enhancing the privacy of smart contracts often requires revisiting your application's logic.

For example, if implementing a simple AMM for two encrypted ERC20 tokens based on a linear constant function, it is recommended to not only hide the amounts being swapped, but also the token which is swapped in a pair.

✅ Here is a very simplified example implementation, we suppose here that the rate between tokenA and tokenB is constant and equals to 1:

```solidity
// typically either encryptedAmountAIn or encryptedAmountBIn is an encrypted null value
// ideally, the user already owns some amounts of both tokens and has pre-approved the AMM on both tokens
function swapTokensForTokens(einput encryptedAmountAIn, einput encryptedAmountBIn, bytes calldata inputProof) external {
  euint32 encryptedAmountA = TFHE.asEuint32(encryptedAmountAIn, inputProof); // even if amount is null, do a transfer to obfuscate trade direction
  euint32 encryptedAmountB = TFHE.asEuint32(encryptedAmountBIn, inputProof); // even if amount is null, do a transfer to obfuscate trade direction

  // send tokens from user to AMM contract
  TFHE.allowTransient(encryptedAmountA, tokenA);
  IConfidentialERC20(tokenA).transferFrom(msg.sender, address(this), encryptedAmountA);

  TFHE.allowTransient(encryptedAmountB, tokenB);
  IConfidentialERC20(tokenB).transferFrom(msg.sender, address(this), encryptedAmountB);

  // send tokens from AMM contract to user
  // Price of tokenA in tokenB is constant and equal to 1, so we just swap the encrypted amounts here
  TFHE.allowTransient(encryptedAmountB, tokenA);
  IConfidentialERC20(tokenA).transfer(msg.sender, encryptedAmountB);

  TFHE.allowTransient(encryptedAmountA, tokenB);
  IConfidentialERC20(tokenB).transferFrom(msg.sender, address(this), encryptedAmountA);
}
```

Notice that to preserve confidentiality, we had to make two inputs transfers on both tokens from the user to the AMM contract, and similarly two output transfers from the AMM to the user, even if technically most of the times it will make sense that one of the user inputs `encryptedAmountAIn` or `encryptedAmountBIn` is actually an encrypted zero.

This is different from a classical non-confidential AMM with regular ERC20 tokens: in this case, the user would need to just do one input transfer to the AMM on the token being sold, and receive only one output transfer from the AMM on the token being bought.

### Avoid using encrypted indexes

Using encrypted indexes to pick an element from an array without revealing it is not very efficient, because you would still need to loop on all the indexes to preserve confidentiality.

However, there are plans to make this kind of operation much more efficient in the future, by adding specialized operators for arrays.

For instance, imagine you have an encrypted array called `encArray` and you want to update an encrypted value `x` to match an item from this list, `encArray[i]`, _without_ disclosing which item you're choosing.

❌ You must loop over all the indexes and check equality homomorphically, however this pattern is very expensive in gas and should be avoided whenever possible.

```solidity
euint32 x;
euint32[] encArray;

function setXwithEncryptedIndex(einput encryptedIndex, bytes calldata inputProof) public {
    euint32 index = TFHE.asEuint32(encryptedIndex, inputProof);
    for (uint32 i = 0; i < encArray.length; i++) {
        ebool isEqual = TFHE.eq(index, i);
        x = TFHE.select(isEqual, encArray[i], x);
    }
    TFHE.allowThis(x);
}
```
