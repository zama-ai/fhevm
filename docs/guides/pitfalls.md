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
  }
}
```

### Never use public encrypted state variables

Declaring an encrypted state variable as public exposes the variable to any external untrusted smart contract to access and potentially decrypt them, compromising their confidentiality.

❌ In summary, never write in production:

```solidity
contract C {
  euint32 public a;

  constructor(uint32 _a) {
    a = TFHE.asEuint32(_a);
  }
}
```

✅ Instead, you should declare the state variable as follow:

```solidity
contract C {
  euint32 internal a;

  constructor(uint32 _a) {
    a = TFHE.asEuint32(_a);
  }
}
```

In this last snippet, the `internal` keyword could have been omitted (state variables are internal by default) or alternatively have been replaced by `private`.

### Protect access of view functions using reencryptions

If a view function is using `TFHE.reencrypt` it is mandatory to protect its access to not leak confidentiality, for instance this is doable easily via the `onlySignedPublicKey` modifier imported from `"fhevm/abstracts/Reencrypt.sol"`. See the example from the [Decrypt page](../fundamentals/decrypt.md#handle-private-reencryption). Failing to address this allows anyone to reencrypt another person's ciphertext. This vulnerability comes from the ability to impersonate any `msg.sender` address during a static call to a view function, as it does not require a signature, unlike transactions.

## Best practises

### Avoid using TFHE.decrypt, use TFHE.cmux instead

Any use of decryption should be avoided as much as possible. Current version of `TFHE.decrypt` will soon be deprecated and get replaced by an asynchronous version, so please consider this operator as a very expensive one which should be used only if absolutely necessary.

Whenever your code contains a branch depending on the result of a decryption, we recommend to replace it by a `TFHE.cmux`.

❌ For instance, instead of:

```solidity
euint32 x;
ebool condition = TFHE.gt(x,5)
if(TFHE.decrypt(condition)){
    x = TFHE.asEuint(0);
} else {
    x = TFHE.asEuint(42);
}
```

✅ We recommend instead to use the following pattern:

```solidity
euint32 x;
ebool condition = TFHE.gt(x,5)
x = TFHE.cmux(condition, TFHE.asEuint(0), TFHE.asEuint(42));
```

### Obfuscate branching

The previous paragraph emphasized that branch logic should rely as much as possible on `TFHE.cmux` instead of decryptions. It hides effectively which branch has been executed.

However, this is sometimes not enough. Enhancing the privacy of smart contracts often requires revisiting your application's logic.

For example, if implementing a simple AMM for two encrypted ERC20 tokens based on a linear constant function, it is recommended to not only hide the amounts being swapped, but also the token which is swapped in a pair.

✅ Here is a very simplified example implementations, we suppose here that the the rate between tokenA and tokenB is constant and equals to 1:

```solidity
    // typically either encryptedAmountAIn or encryptedAmountBIn is an encrypted null value
    // ideally, the user already owns some amounts of both tokens and has pre-approved the AMM on both tokens
    function swapTokensForTokens(
        bytes calldata encryptedAmountAIn,
        bytes calldata encryptedAmountBIn,
    ) external {
        euint32 encryptedAmountA = TFHE.asEuint32(encryptedAmountAIn); // even if amount is null, do a transfer to obfuscate trade direction
        euint32 encryptedAmountB = TFHE.asEuint32(encryptedAmountBIn); // even if amount is null, do a transfer to obfuscate trade direction

        // send tokens from user to AMM contract
        IEncryptedERC20(tokenA).transferFrom(
            msg.sender, address(this), encryptedAmountA
        );
        IEncryptedERC20(tokenB).transferFrom(
            msg.sender, address(this), encryptedAmountB
        );

        // send tokens from AMM contract to user
        // Price of tokenA in tokenB is constant and equal to 1, so we just swap the encrypted amounts here
        IEncryptedERC20(tokenA).transfer(
            msg.sender, encryptedAmountB
        );
        IEncryptedERC20(tokenB).transferFrom(
            msg.sender, address(this), encryptedAmountA
        );
    }

```

Notice that to preserve confidentiality, we had to make two inputs transfers on both tokens from the user to the AMM contract, and similarly two output transfers from the AMM to the user, even if technically most of the times it will make sense that one of the user inputs `encryptedAmountAIn` or `encryptedAmountBIn` is actually an encrypted zero.

This is different from a classical non-confidential AMM with regular ERC20 tokens: in this case, the user would need to just do one input transfer to the AMM on the token being sold, and receive only one output transfer from the AMM on the token being bought.

### Avoid using while loops with an encrypted condition

❌ Avoid using this type of loop because it might require many decryption operations:

```solidity
ebool isTrue;
euint32 x;
// some code
while(TFHE.decrypt(isTrue)){
    x=TFHE.add(x, 1);
    // some other code
}
```

If your code logic requires looping on an encrypted boolean condition, we highly suggest to try to replace it by a finite loop with an appropriate constant maximum number of steps and use `TFHE.cmux` inside the loop.

✅ For example, the previous code could maybe be replaced by the following snippet:

```solidity
ebool isTrue;
euint32 x;
// some code
for (uint32 i = 0; i < 5; i++) {
    euint32 increment = TFHE.cmux(isTrue, 1, 0);
    x=TFHE.add(x, increment);
    // some other code
}
```

### Avoid using encrypted indexes

Using encrypted indexes to pick an element from an array without revealing it is not very efficient, because you would still need to loop on all the indexes to preserve confidentiality.

However, there are plans to make this kind of operation much more efficient in the future, by adding specialized operators for arrays.

For instance, imagine you have an encrypted array called `encArray` and you want to update an encrypted value `x` to match an item from this list, `encArray[i]`, _without_ disclosing which item you're choosing.

❌ You must loop over all the indexes and check equality homomorphically, however this pattern is very expensive in gas and should be avoided whenever possible.

```solidity
euint32 x;
euint32[] encArray;

function setXwithEncryptedIndex(bytes calldata encryptedIndex) public {
    euint32 index = TFHE.asEuint32(encryptedIndex);
    for (uint32 i = 0; i < encArray.length; i++) {
        ebool isEqual = TFHE.eq(index, i);
        x = TFHE.cmux(isEqual, encArray[i], x);
    }
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
function mint(bytes calldata encryptedAmount) public {
  euint32 mintedAmount = TFHE.asEuint32(encryptedAmount);
  totalSupply = TFHE.add(totalSupply, mintedAmount);
  balances[msg.sender] = TFHE.add(balances[msg.sender], mintedAmount);
}
```

✅ But you can fix this issue by using `TFHE.cmux` to cancel the mint in case of an overflow:

```solidity
function mint(bytes calldata encryptedAmount) public {
  euint32 mintedAmount = TFHE.asEuint32(encryptedAmount);
  euint32 tempTotalSupply = TFHE.add(totalSupply, mintedAmount);
  ebool isOverflow = TFHE.lt(tempTotalSupply, totalSupply);
  totalSupply = TFHE.cmux(isOverflow, totalSupply, tempTotalSupply);
  euint32 tempBalanceOf = TFHE.add(balances[msg.sender], mintedAmount);
  balances[msg.sender] = TFHE.cmux(isOverflow, balances[msg.sender], tempBalanceOf);
}
```

Notice that we did not check separately the overflow on `balances[msg.sender]` but only on `totalSupply` variable, because `totalSupply` is the sum of the balances of all the users, so `balances[msg.sender]` could never overflow if `totalSupply` did not.
