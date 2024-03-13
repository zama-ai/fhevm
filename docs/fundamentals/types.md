# Use encrypted types

The `TFHE` library provides encrypted integer types and a type system that is checked both at compile time and at run time.

Encrypted integers behave as much as possible as Solidity's integer types. Currently, however, behaviour such as "revert on overflow" is not supported as this would leak some information about the encrypted value. Therefore, arithmetic on `e(u)int` types is [unchecked](https://docs.soliditylang.org/en/latest/control-structures.html#checked-or-unchecked-arithmetic), i.e. there is wrap-around on overflow.

Encrypted integers with overflow checking are coming soon to the `TFHE` library. They will allow reversal in case of an overflow, but will leak some information about the operands.

In terms of implementation in the `fhEVM`, encrypted integers take the form of FHE ciphertexts.
The `TFHE` library abstracts away that and, instead, exposes ciphertext handles to smart contract developers.
The `e(u)int` types are **wrappers** over these handles.

The following encrypted data types are defined:

| type       | supported       |
| ---------- | --------------- |
| `ebool`    | yes             |
| `euint4`   | yes             |
| `euint8`   | yes             |
| `euint16`  | yes             |
| `euint32`  | yes             |
| `euint64`  | yes             |
| `eaddress` | yes             |
| `eint8`    | no, coming soon |
| `eint16`   | no, coming soon |
| `eint32`   | no, coming soon |
| `eint64`   | no, coming soon |

Higher-precision integers are supported in the `TFHE-rs` library and can be added as needed to `fhEVM`.

## Verification

When users send serialized ciphertexts as `bytes` to the blockchain, they first need to be converted to the respective encrypted integer type. Conversion verifies if the ciphertext is well-formed and includes proof verification. These steps prevent usage of arbitrary inputs.
For example, following functions are provided for `ebool`, `euint8`, `euint16` and `euint32`:

- `TFHE.asEbool(bytes ciphertext)` verifies the provided ciphertext and returns an `ebool`
- `TFHE.asEuint4(bytes ciphertext)` verifies the provided ciphertext and returns an `euint4`
- `TFHE.asEuint8(bytes ciphertext)` verifies the provided ciphertext and returns an `euint8`
- `TFHE.asEuint16(bytes ciphertext)` verifies the provided ciphertext and returns an `euint16`
- `TFHE.asEuint32(bytes ciphertext)` verifies the provided ciphertext and returns an `euint32`
- `TFHE.asEuint64(bytes ciphertext)` verifies the provided ciphertext and returns an `euint64`
- `TFHE.asEaddress(bytes ciphertext)` verifies the provided ciphertext and returns an `eaddress`
- ... more functions for the respective encrypted integer types

### Example

```solidity
function mint(bytes calldata encryptedAmount) public onlyContractOwner {
  euint64 amount = TFHE.asEuint64(encryptedAmount);
  balances[contractOwner] = balances[contractOwner] + amount;
  totalSupply = totalSupply + amount;
}
```

## Contract state variables with encrypted types

If you require a state variable that utilizes these encrypted types, you cannot assign the value with `immutable` or `constant` keyword. If you're using these types, the compiler attempts to ascertain the value of `TFHE.asEuintXX(yy)` during compilation, which is not feasible because `asEuintXX()` invokes a precompiled contract. To address this challenge, you must not declare your encrypted state variables as `immutable` or `constant`. Still, you can use the following methods to set your variables:

```solidity
euint32 private totalSupply = TFHE.asEuint(0);
```

```solidity
euint32 private totalSupply;
constructor() {
  totalSupply = TFHE.asEuint32(0);
}
```
