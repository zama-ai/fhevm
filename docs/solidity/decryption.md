# Decryption and control structures

## Decryptions

We allow explicit decryption requests for any encrypted type.
The values are decrypted through the distributed decryption protocol and are stored on-chain.

### Examples

```solidity
function decryptAmount(euint8 amount) public view returns (uint8) {
    return TFHE.decrypt(amount);
}

function revertIfConditionIsFalse(ebool condition) public {
    bool plaintextCondition = TFHE.decrypt(condition);
    require(plaintextCondition, "Condition was not met");
    // ... continue execution if `condition` is true
}
```

## Booleans

The result of [comparison operations](functions.md#comparison-operation-eq-ne-ge-gt-le-lt) is of type `ebool`. Typical boolean operations are not currently supported for this type.

The purpose of the `ebool` type is two-fold:

1. control bit for the [`cmux`](functions.md#multiplexer-operator-cmux) operator;
2. input for optimistic encrypted require (`optReq`) statements.

## Optimistic encrypted require statements

The decryption statements described above may lead to important delays during the transaction execution as several of them may need to be processed in a single transaction.
Given that those decryptions might be used for control flow by using the Solidity `require` function, we introduce optimistic require statements (`optReq`).
These require statements take as input a value to type `ebool` and are accumulated throughout the execution of the transaction. 
The accumulated boolean value is decrypted via the threshold decryption protocol either when an explicit decryption is executed, or at the very end of a transaction execution. 
If the decryption returns `false`, the transaction is reverted. Otherwise, state changes are persisted as usual.
Optimistic requires may be more efficient, but this efficiency comes at the price of paying the full transaction gas cost if one of the boolean predicates is false.
