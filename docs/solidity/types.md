# Types

The library provides a type system that is checked both at compile time and at run time.
The structure and operations related to these types are described in this sections.

We currently support encrypted integers of bit length up to 32 bits.

<!-- Support for up to 256 bits is on our roadmap.  -->

Our library provides the following types :

- `ebool`
- `euint8`
- `euint16`
- `euint32`

These encrypted integers behave as much as possible as Solidity's integer types. However, behaviour such as "revert on overflow" is not supported as this would leak some information of the encrypted integers. Therefore, arithmetic on `euint` types is [unchecked](https://docs.soliditylang.org/en/latest/control-structures.html#checked-or-unchecked-arithmetic), i.e. there is wrap-around on overflow.

In the back-end, encrypted integers are TFHE ciphertexts.
The library abstracts away the ciphertexts and presents pointers to ciphertexts, or ciphertext handles, to the smart contract developer.
The `euint` types are _wrappers_ over these handles.

## Verification

When a user sends an encrypted integer, this ciphertext must be checked to prevent arbitrary data from being sent. So, to check user input received as `bytes`, there are several methods.

- `TFHE.asEbool()` will verify and returns a `ebool`
- `TFHE.asEuint8()` will verify and returns a `euint8`
- `TFHE.asEuint16()` will verify and returns a `euint16`
- `TFHE.asEuint32()` will verify and returns a `euint32`

### Example

```solidity
function mint(bytes calldata encryptedAmount) public onlyContractOwner {
  euint32 amount = TFHE.asEuint32(encryptedAmount);
  balances[contractOwner] = balances[contractOwner] + amount;
  totalSupply = totalSupply + amount;
}
```
