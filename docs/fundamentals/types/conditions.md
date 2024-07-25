# Branching in FHE
This document explains how to perform branching with encrypted booleans in fhEVM, including methods for condition handling and error management.

When you perform [comparison operations](../../references/functions.md#comparison-operation-eq-ne-ge-gt-le-lt) in fhEVM, the result is an encrypted boolean `ebool`, which does not support typical boolean operations.

## Condition with encrypted boolean

fhEVM provides a method which acts as a ternary operator on encrypted integers. This method is called [select](../../references/functions.md#multiplexer-operator-select).

```solidity
function bid(einput encryptedValue, bytes calldata inputProof) public onlyBeforeEnd {
  euint64 bid = TFHE.asEuint64(encryptedValue, inputProof);
  ebool isAbove = TFHE.lt(highestBid, bid);

  // Replace highest bid
  highestBid = TFHE.select(isAbove, bid, highestBid);
  TFHE.allow(highestBid, address(this));
}
```

Keep in mind that each time we assign a value using `TFHE.select`, the value changes, even if the plaintext value remains the same.

## Error handling

If a condition is not satisfied, the transaction will not be reverted, which can be challenging to communicate issues to users. 

To address this, a recommended approach is to implement an error handler that stores the latest error information for all wallets:

```solidity
struct LastError {
  euint8 error;
  uint timestamp;
}

euint8 internal NO_ERROR;
euint8 internal NOT_ENOUGH_FUND;

constructor() {
  NO_ERROR = TFHE.asEuint8(0);
  NOT_ENOUGH_FUND = TFHE.asEuint8(1);
}

function setLastError(euint8 error, address addr) private {
  _lastErrors[addr] = LastError(error, block.timestamp);
  emit ErrorChanged(addr);
}

function _transfer(address from, address to, euint32 amount) internal {
  // Make sure the sender has enough tokens.
  ebool canTransfer = TFHE.le(amount, balances[from]);
  setLastError(TFHE.select(canTransfer, NO_ERROR, NOT_ENOUGH_FUND), msg.sender);

  // Add to the balance of `to` and subract from the balance of `from`.
  balances[to] = TFHE.add(balances[to], TFHE.select(canTransfer, amount, TFHE.asEuint32(0)));
  TFHE.allow(balances[to], address(this));
  TFHE.allow(balances[to], to);

  balances[from] = TFHE.sub(balances[from], TFHE.select(canTransfer, amount, TFHE.asEuint32(0)));
  TFHE.allow(balances[from], address(this));
  TFHE.allow(balances[from], from);
}
```
