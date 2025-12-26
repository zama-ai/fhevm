# Confidential Wrapper

The **Confidential Wrapper** is a smart contract that wraps standard ERC-20 tokens into confidential ERC-7984 tokens. Built on Zama's FHEVM, it enables privacy-preserving token transfers where balances and transfer amounts remain encrypted.

## Terminology

- **Confidential Token**: The ERC-7984 confidential token wrapper.
- **Underlying Token**: The standard ERC-20 token wrapped by the confidential wrapper.
- **Wrapping**: Converting ERC-20 tokens into confidential tokens.
- **Unwrapping**: Converting confidential tokens back into ERC-20 tokens.
- **Rate**: The conversion ratio between underlying token units and confidential token units (due to decimal differences). 
- **Operator**: An address authorized to transfer confidential tokens on behalf of another address.
- **Owner**: The owner of the wrapper contract. In the FHEVM protocol, this is initially set to a DAO governance contract handled by Zama. Ownership will then be transferred to the underlying token's owner.
- **Registry**: The registry contract that maps ERC-20 tokens to their corresponding confidential wrappers. More information [here](../../confidential-token-wrappers-registry/docs/README.md).
- **ACL**: The ACL contract that manages the ACL permissions for encrypted amounts. More information in the [FHEVM library documentation](https://docs.zama.org/protocol/protocol/overview/library#access-control).
- **Input proof**: A proof that the encrypted amount is valid. More information in the [`relayer-sdk` documentation](https://docs.zama.org/protocol/relayer-sdk-guides/fhevm-relayer/input).
- **Public decryption**: A request to publicly decrypt an encrypted amount. More information in the [`relayer-sdk` documentation](https://docs.zama.org/protocol/relayer-sdk-guides/fhevm-relayer/decryption/public-decryption).

## Quick Start

> ⚠️ **Decimal conversion:** The wrapper enforces a maximum of **6 decimals** for the confidential token. When wrapping, amounts are rounded down and excess tokens are refunded.

> ⚠️ **Unsupported tokens:** Non-standard tokens such as fee-on-transfer or any deflationary-type tokens are NOT supported.

### Get the confidential wrapper address of an ERC-20 token

Zama provides a registry contract that maps ERC-20 tokens to their corresponding verified confidential wrappers. Make sure to check the registry contract to ensure the confidential wrapper is valid before wrapping. More information [here](../../confidential-token-wrappers-registry/docs/README.md).

### Wrap ERC-20 → Confidential Token

**Important:** Prior to wrapping, the confidential wrapper contract must be approved by the `msg.sender` on the underlying token.

```solidity
wrapper.wrap(to, amount);
```

The wrapper will mint the corresponding confidential token to the `to` address and refund the excess tokens to the `msg.sender` (due to decimal conversion). Considerations:
- `amount` must be a value using the same decimal precision as the underlying token.
- `to` must not be the zero address.

> ℹ️ **Low amount handling:** If the amount is less than the rate, the wrapping will succeed but the recipient will receive 0 confidential tokens and the excess tokens will be refunded to the `msg.sender`.


### Unwrap Confidential Token → ERC-20

Unwrapping is a **two-step asynchronous process**: an `unwrap` must be first made and then finalized with `finalizeUnwrap`. The `unwrap` function can be called with or without an input proof.

#### 1) Unwrap request

> ⚠️ **Unsupported `from`:** Accounts with a zero balance that have never held tokens cannot be the `from` address in unwrap requests.

##### With input proof

> ℹ️ **Input proof:** To unwrap any amount of confidential tokens, the `from` address must first create an encrypted input to generate an `encryptedAmount` (`externalEuint64`) along its `inputProof`. The amount to be encrypted must use the same decimal precision as the confidential wrapper. More information in the [`relayer-sdk` documentation](https://docs.zama.org/protocol/relayer-sdk-guides/fhevm-relayer/input).

```solidity
wrapper.unwrap(from, to, encryptedAmount, inputProof);
```

Alternatively, an unwrap request can be made without an input proof if the encrypted amount (`euint64`) is known to `from`. For example, this can be the confidential balance of `from`.

This requests an unwrap request of `encryptedAmount` confidential tokens from `from`. Considerations:
- `msg.sender` must be `from` or an approved operator for `from`.
- `from` mut not be the zero address.
- `encryptedAmount` will be burned in the request.
- **NO** transfer of underlying tokens is made in this request.


It emits an `UnwrapRequested` event: 
```solidity
event UnwrapRequested(address indexed receiver, euint64 amount);
```

###### Without input proof

Alternatively, an unwrap request can be made without an input proof if the encrypted amount (`euint64`) is known to `from`. For example, this can be the confidential balance of `from`.

```solidity
wrapper.unwrap(from, to, encryptedAmount);
```

On top of the above unwrap request considerations:
- `msg.sender` must be approved by ACL for the given `encryptedAmount` ⚠️ (see [ACL documentation](https://docs.zama.org/protocol/protocol/overview/library#access-control)).


#### 2) Finalize unwrap

> ℹ️ **Public decryption:** The encrypted burned amount `burntAmount` emitted by the `UnwrapRequested` event must be publicly decrypted to get the `cleartextAmount` along its `decryptionProof`. More information in the [`relayer-sdk` documentation](https://docs.zama.org/protocol/relayer-sdk-guides/fhevm-relayer/decryption/public-decryption).

```solidity
wrapper.finalizeUnwrap(burntAmount, cleartextAmount, decryptionProof);
```

This finalizes the unwrap request by sending the corresponding amount of underlying tokens to the `to` defined in the `unwrap` request.

### Transfer confidential tokens

> ℹ️ **Transfer with input proof:** Similarly to the unwrap process, transfers can be made with or without an input proof and the encrypted amount must be approved by the ACL for the `msg.sender`.

> ⚠️ **Unsupported `from`:** Accounts with a zero balance that have never held tokens cannot be the `from` address in confidential transfers.

#### Direct transfer

```solidity
token.confidentialTransfer(to, encryptedAmount, inputProof);

token.confidentialTransfer(to, encryptedAmount);
```

#### Operator-based transfer

```solidity
token.confidentialTransferFrom(from, to, encryptedAmount, inputProof);

token.confidentialTransferFrom(from, to, encryptedAmount);
```

Considerations:
- `msg.sender` must be `from` or an approved operator for `from`.

#### Transfer with callback

The callback can be used along an ERC-7984 receiver contract.

```solidity
token.confidentialTransferAndCall(to, encryptedAmount, inputProof, callbackData);

token.confidentialTransferAndCall(to, encryptedAmount, callbackData);
```

#### Operator-based transfer with callback

The callback can be used along an ERC-7984 receiver contract.

```solidity
token.confidentialTransferFromAndCall(from, to, encryptedAmount, inputProof, callbackData);

token.confidentialTransferFromAndCall(from, to, encryptedAmount, callbackData);
```

Considerations:
- `msg.sender` must be `from` or an approved operator for `from`.

### Check the conversion rate and decimals

```solidity
uint256 conversionRate = wrapper.rate();
uint8 wrapperDecimals = wrapper.decimals();
```

**Examples:**
| Underlying Decimals | Wrapper Decimals | Rate | Effect |
|---------------------|------------------|------|--------|
| 18 | 6 | 10^12 | 1 wrapped = 10^12 underlying |
| 6 | 6 | 1 | 1:1 mapping |
| 2 | 2 | 1 | 1:1 mapping |

### Check supplies

#### Non-confidential total supply

The wrapper exposes a non-confidential view of the total supply, computed from the underlying ERC20 balance held by the wrapper contract. This value may be higher than `confidentialTotalSupply()` if tokens are sent directly to the wrapper outside of the wrapping process.

> ℹ️ **Total Value Shielded (TVS):** This view function is useful for getting a good approximation of the wrapper's Total Value Shielded (TVS).

```solidity
uint256 nonConfidentialSupply = wrapper.totalSupply();
```

#### Encrypted (confidential) total supply

The actual supply tracked by the confidential token contract, represented as an encrypted value. To determine the cleartext value, you need to request decryption and appropriate ACL authorization.

```solidity
euint64 encryptedSupply = wrapper.confidentialTotalSupply();
```

#### Maximum total supply

The maximum number of wrapped tokens supported by the encrypted datatype (uint64 limit). If this maximum is exceeded, wrapping new tokens will revert.

```solidity
uint256 maxSupply = wrapper.maxTotalSupply();
```

---

## Integration Patterns

### Operator system

Delegate transfer capabilities with time-based expiration:

```solidity
// Grant operator permission until a specific timestamp
token.setOperator(operatorAddress, validUntilTimestamp);

// Check if an address is an authorized operator
bool isAuthorized = token.isOperator(holder, spender);
```

### Amount disclosure

Optionally reveal encrypted amounts publicly:

```solidity
// Request disclosure (initiates async decryption)
token.requestDiscloseEncryptedAmount(encryptedAmount);

// Complete disclosure with proof
token.discloseEncryptedAmount(encryptedAmount, cleartextAmount, decryptionProof);
```

### Check ACL permissions

Before using encrypted amounts in transactions, callers must be authorized:

```solidity
require(FHE.isAllowed(encryptedAmount, msg.sender), "Unauthorized");
```

Transfer functions with `euint64` (not `externalEuint64`) require the caller to already have ACL permission for that ciphertext. More information in the [FHEVM library documentation](https://docs.zama.org/protocol/protocol/overview/library#access-control).

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     ConfidentialWrapper                         │
│  (UUPS Upgradeable, Ownable2Step)                              │
├─────────────────────────────────────────────────────────────────┤
│                 ERC7984ERC20WrapperUpgradeable                  │
│  (Wrapping/Unwrapping Logic, ERC1363 Receiver)                 │
├─────────────────────────────────────────────────────────────────┤
│                    ERC7984Upgradeable                           │
│  (Confidential Token Standard - Encrypted Balances/Transfers)  │
├─────────────────────────────────────────────────────────────────┤
│               ZamaEthereumConfigUpgradeable                     │
│  (FHE Coprocessor Configuration)                               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Events

| Event | Description |
|-------|-------------|
| `ConfidentialTransfer(from, to, encryptedAmount)` | Emitted on every transfer (including mint/burn) |
| `OperatorSet(holder, operator, until)` | Emitted when operator permissions change |
| `UnwrapRequested(receiver, encryptedAmount)` | Emitted when unwrap is initiated |
| `UnwrapFinalized(receiver, encryptedAmount, cleartextAmount)` | Emitted when unwrap completes |
| `AmountDiscloseRequested(encryptedAmount, requester)` | Emitted when disclosure is requested |
| `AmountDisclosed(encryptedAmount, cleartextAmount)` | Emitted when amount is publicly disclosed |

---

## Errors

| Error | Cause |
|-------|-------|
| `ERC7984InvalidReceiver(address)` | Transfer to zero address |
| `ERC7984InvalidSender(address)` | Transfer from zero address |
| `ERC7984UnauthorizedSpender(holder, spender)` | Caller not authorized as operator |
| `ERC7984ZeroBalance(holder)` | Sender has never held tokens |
| `ERC7984UnauthorizedUseOfEncryptedAmount(amount, user)` | Caller lacks ACL permission for ciphertext |
| `ERC7984UnauthorizedCaller(caller)` | Invalid caller for operation |
| `InvalidUnwrapRequest(amount)` | Finalizing non-existent unwrap request |
| `ERC7984TotalSupplyOverflow()` | Minting would exceed uint64 max |

---

## Important Considerations

### Ciphertext uniqueness assumption

The unwrap mechanism stores requests in a mapping keyed by ciphertext and the current implementation assumes these ciphertexts are unique. This holds in this very specific case but be aware of this architectural decision as it is **NOT** true in the general case.

---

## Interface Support (ERC-165)

```solidity
wrapper.supportsInterface(type(IERC7984).interfaceId);
wrapper.supportsInterface(type(IERC7984ERC20Wrapper).interfaceId);
wrapper.supportsInterface(type(IERC165).interfaceId);
```

---

## Upgradeability

The contract uses **UUPS (Universal Upgradeable Proxy Standard)** with 2-step ownership transfer. Only the owner can upgrade the contract. Initially, the owner is set to a DAO governance contract handled by Zama. Ownership will then be transferred to the underlying token's owner.
