# Confidential Token Wrappers Registry

The **Confidential Token Wrappers Registry** is an on-chain directory that maps ERC-20 tokens to their corresponding ERC-7984 confidential token wrappers. Use it to discover, validate, and integrate confidential wrappers within the FHEVM ecosystem.

## Terminology

- **Token**: An ERC-20 token.
- **Confidential Wrapper**: An ERC-7984 confidential token wrapper. Also called "confidential token".
- **Underlying Token**: The ERC-20 token that the confidential wrapper is associated with.
- **TokenWrapperPair**: A pair of a token and its confidential wrapper.
- **Valid**: A valid confidential wrapper has been verified by the registry owner and can be used to wrap and unwrap tokens from the underlying token.
- **Invalid**: An invalid confidential wrapper has been revoked by the registry owner and should not be used to wrap and unwrap tokens from the underlying token.
- **Owner**: The owner of the registry. In the FHEVM protocol, this is a DAO governance contract handled by Zama.

## Quick Start

> A token can only be associated with one confidential wrapper. A confidential wrapper can only be associated with one token.

> ⚠️ **Always check validity:** A non-zero wrapper address may be revoked by the owner. Always verify the `isValid` flag associated with the (token, wrapper) pair before use.

### Find the confidential wrapper of a token

```solidity
(bool isValid, address confidentialToken) = registry.getConfidentialTokenAddress(erc20TokenAddress);
```

If the token has been registered with a confidential wrapper: 
- `isValid` will be `true`
- `confidentialToken` will be the address of the confidential wrapper.

If the token has never been registered with a confidential wrapper: 
- `isValid` will be `false`
- `confidentialToken` will be `address(0)`.

If the confidential wrapper has been revoked:
- `isValid` will be `false`
- `confidentialToken` will be the address of the (revoked) confidential wrapper.

### Find the underlying token of a confidential wrapper

```solidity
(bool isValid, address token) = registry.getTokenAddress(confidentialWrapperAddress);
```

If the confidential wrapper has been registered with a token:
- `isValid` will be `true`
- `token` will be the address of its underlying token.

If the confidential wrapper has never been registered with a token:
- `isValid` will be `false`
- `token` will be `address(0)`.

If the confidential wrapper has been revoked:
- `isValid` will be `false`
- `token` will be the address of its underlying token.

### Check if a confidential wrapper is valid

```solidity
bool isValid = registry.isConfidentialTokenValid(confidentialWrapperAddress);
```

If the confidential wrapper has been revoked, 
- `isValid` will be `false`.

If the confidential wrapper has not been revoked (it is still valid), 
- `isValid` will be `true`.


---

## Integration patterns 

Token and confidential wrapper pairs are stored in the registry as `TokenWrapperPair` structs. Each struct contains:
- `tokenAddress`: the address of the underlying token.
- `confidentialTokenAddress`: the address of the confidential wrapper.
- `isValid`: `true` if the confidential wrapper is valid, `false` if it has been revoked.

### Get all valid confidential (token, wrapper) pairs
```solidity
TokenWrapperPair[] memory tokenConfidentialTokenPairs = registry.getTokenConfidentialTokenPairs();
```

It returns all confidential wrappers (including revoked ones).

### Get the total number of confidential (token, wrapper) pairs

```solidity
uint256 totalTokenConfidentialTokenPairs = registry.getTokenConfidentialTokenPairsLength();
```

### Get the index of a token

```solidity
uint256 tokenIndex = registry.getTokenIndex(tokenAddress);
```

`tokenAddress` must be a registered token. Otherwise, it will revert with `TokenNotRegistered`.

### Get a valid confidential (token, wrapper) pair by index

```solidity
TokenWrapperPair memory tokenConfidentialTokenPair = registry.getTokenConfidentialTokenPair(index);
```

It returns a single confidential (token, wrapper) pair (including revoked ones).

### Get a slice of confidential (token, wrapper) pairs

```solidity
TokenWrapperPair[] memory tokenConfidentialTokenPairsSlice = registry.getTokenConfidentialTokenPairsSlice(fromIndex, toIndex);
```

It returns a slice of confidential (token, wrapper) pairs (including revoked ones). `fromIndex` is included and `toIndex` is excluded.

---

## Data Structures

### TokenWrapperPair

```solidity
struct TokenWrapperPair {
    address tokenAddress;              // The ERC-20 token
    address confidentialTokenAddress;  // The ERC-7984 wrapper
    bool isValid;                      // false if revoked
}
```

---

## Events

| Event | Description |
|-------|-------------|
| `ConfidentialTokenRegistered(tokenAddress, confidentialTokenAddress)` | Emitted when a new pair is registered |
| `ConfidentialTokenRevoked(tokenAddress, confidentialTokenAddress)` | Emitted when a wrapper is revoked |

---

## Errors

| Error | Cause |
|-------|-------|
| `TokenZeroAddress()` | Attempted to register with zero token address |
| `ConfidentialTokenZeroAddress()` | Attempted to register/revoke with zero wrapper address |
| `NotERC7984(confidentialTokenAddress)` | Wrapper doesn't support ERC-7984 interface |
| `ConfidentialTokenDoesNotSupportERC165(confidentialTokenAddress)` | Wrapper doesn't implement ERC-165 |
| `ConfidentialTokenAlreadyAssociatedWithToken(tokenAddress, existingConfidentialTokenAddress)` | Wrapper already registered to another token |
| `TokenAlreadyAssociatedWithConfidentialToken(tokenAddress, existingConfidentialTokenAddress)` | Token already has a registered wrapper |
| `RevokedConfidentialToken(confidentialTokenAddress)` | Attempting to revoke an already-revoked wrapper |
| `NoTokenAssociatedWithConfidentialToken(confidentialTokenAddress)` | Attempting to revoke unregistered wrapper |
| `FromIndexGreaterOrEqualToIndex(fromIndex, toIndex)` | Invalid slice range |
| `TokenNotRegistered(tokenAddress)` | Token has not been registered |

---

## Owner Administration

> All administrative actions are restricted to the registry owner.

### Register a confidential token

```solidity
registry.registerConfidentialToken(
    erc20TokenAddress,
    confidentialWrapperAddress
);
```

**Validation performed:**
- Neither address can be zero
- Confidential token must implement ERC-165 (`supportsInterface` function) and support the ERC-7984 interface (`0x4958f2a4`)
- Token must not already have an associated wrapper
- Confidential token must not already be associated with another token

### Revoke a confidential token

```solidity
registry.revokeConfidentialToken(confidentialWrapperAddress);
```

**Important:** Revocation does NOT delete the mapping—it only sets `isValid = false`. This preserves historical records and prevents re-registration of malicious addresses.

Revoking is currently permanent. When a wrapper is revoked:
- `isValid` is set to `false`
- The mapping entries remain in storage
- The token cannot be registered with a new wrapper

---

## Upgradeability

The contract uses **UUPS (Universal Upgradeable Proxy Standard)** with 2-step ownership transfer. Only the owner can upgrade the contract.
