# Common pitfalls and best practises

## Common pitfalls to avoid

### No constant nor immutable encrypted state variables

Never use encrypted types for constant or immutable state variables, even if they should actually stay constants, or else any transaction involving those will fail. This is because ciphertexts should always be stored in the privileged storage of the contract (see paragraph 4.4 of [whitepaper](../../fhevm-whitepaper.pdf)) while constant and immutable variables are just appended to the bytecode of the deployed contract at construction time.

❌ So, even if `a` and `b` should never change after construction, this code :

```solidity
contract C {
  euint32 internal constant a = TFHE.asEuint32(42);
  euint32 internal immutable b;

  constructor(uint32 _b) {
    b = TFHE.asEuint32(_b);
    TFHE.allow(b, address(this));
  }
}
```

✅ Should be replaced by this snippet:

```solidity
contract C {
  euint32 internal a = TFHE.asEuint32(42);
  euint32 internal b;

  constructor(uint32 _b) {
    b = TFHE.asEuint32(_b);
    TFHE.allow(b, address(this));
  }
}
```

## Best practises

### Obfuscate branching

The previous paragraph emphasized that branch logic should rely as much as possible on `TFHE.select` instead of decryptions. It hides effectively which branch has been executed.

However, this is sometimes not enough. Enhancing the privacy of smart contracts often requires revisiting your application's logic.

For example, if implementing a simple AMM for two encrypted ERC20 tokens based on a linear constant function, it is recommended to not only hide the amounts being swapped, but also the token which is swapped in a pair.

✅ Here is a very simplified example implementations, we suppose here that the the rate between tokenA and tokenB is constant and equals to 1:

```solidity
// typically either encryptedAmountAIn or encryptedAmountBIn is an encrypted null value
// ideally, the user already owns some amounts of both tokens and has pre-approved the AMM on both tokens
function swapTokensForTokens(einput encryptedAmountAIn, einput encryptedAmountBIn, bytes calldata inputProof) external {
  euint32 encryptedAmountA = TFHE.asEuint32(encryptedAmountAIn, inputProof); // even if amount is null, do a transfer to obfuscate trade direction
  euint32 encryptedAmountB = TFHE.asEuint32(encryptedAmountBIn, inputProof); // even if amount is null, do a transfer to obfuscate trade direction

  // send tokens from user to AMM contract
  TFHE.allowTransient(encryptedAmountA, tokenA);
  IEncryptedERC20(tokenA).transferFrom(msg.sender, address(this), encryptedAmountA);

  TFHE.allowTransient(encryptedAmountB, tokenB);
  IEncryptedERC20(tokenB).transferFrom(msg.sender, address(this), encryptedAmountB);

  // send tokens from AMM contract to user
  // Price of tokenA in tokenB is constant and equal to 1, so we just swap the encrypted amounts here
  TFHE.allowTransient(encryptedAmountB, tokenA);
  IEncryptedERC20(tokenA).transfer(msg.sender, encryptedAmountB);

  TFHE.allowTransient(encryptedAmountA, tokenB);
  IEncryptedERC20(tokenB).transferFrom(msg.sender, address(this), encryptedAmountA);
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
    TFHE.allow(x, address(this));
}
```

### Use scalar operands when possible to save gas

Some TFHE operators exist in two versions : one where all operands are ciphertexts handles, and another where one of the operands is an unencrypted scalar. Whenever possible, use the scalar operand version, as this will save a lot of gas. See the page on [Gas](gas.md) to discover which operators support scalar operands and compare the gas saved between both versions: all-encrypted operands vs scalar.

❌ For example, this snippet cost way more in gas:

```solidity
euint32 x;
...
x = TFHE.add(x,TFHE.asEuint(42));
```

✅ Than this one:

```solidity
euint32 x;
...
x = TFHE.add(x,42);
```

Despite both leading to the same encrypted result!

### Beware of overflows of TFHE arithmetic operators

TFHE arithmetic operators can overflow. Do not forget to take into account such a possibility when implementing fhEVM smart contracts.

❌ For example, if you wanted to create a mint function for an encrypted ERC20 tokens with an encrypted `totalSupply` state variable, this code is vulnerable to overflows:

```solidity
function mint(einput encryptedAmount, bytes calldata inputProof) public {
  euint32 mintedAmount = TFHE.asEuint32(encryptedAmount, inputProof);
  totalSupply = TFHE.add(totalSupply, mintedAmount);
  balances[msg.sender] = TFHE.add(balances[msg.sender], mintedAmount);
  TFHE.allow(balances[msg.sender], address(this));
  TFHE.allow(balances[msg.sender], msg.sender);
}
```

✅ But you can fix this issue by using `TFHE.select` to cancel the mint in case of an overflow:

```solidity
function mint(einput encryptedAmount, bytes calldata inputProof) public {
  euint32 mintedAmount = TFHE.asEuint32(encryptedAmount, inputProof);
  euint32 tempTotalSupply = TFHE.add(totalSupply, mintedAmount);
  ebool isOverflow = TFHE.lt(tempTotalSupply, totalSupply);
  totalSupply = TFHE.select(isOverflow, totalSupply, tempTotalSupply);
  euint32 tempBalanceOf = TFHE.add(balances[msg.sender], mintedAmount);
  balances[msg.sender] = TFHE.select(isOverflow, balances[msg.sender], tempBalanceOf);
  TFHE.allow(balances[msg.sender], address(this));
  TFHE.allow(balances[msg.sender], msg.sender);
}
```

Notice that we did not check separately the overflow on `balances[msg.sender]` but only on `totalSupply` variable, because `totalSupply` is the sum of the balances of all the users, so `balances[msg.sender]` could never overflow if `totalSupply` did not.
