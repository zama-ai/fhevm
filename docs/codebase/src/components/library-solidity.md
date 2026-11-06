# Solidity Library ✅

**Location**: `/library-solidity/`
**Status**: Stable
**Purpose**: Developer-facing FHE primitives for writing confidential smart contracts

## Overview

The Solidity library provides the API that smart contract developers use to work with encrypted types. It abstracts away the complexity of FHE operations behind familiar Solidity syntax.

## Key Components

| File | Purpose |
|------|---------|
| `lib/FHE.sol` | Main developer API - import this to use FHE |
| `lib/Impl.sol` | Implementation details, delegates to precompiles |
| `lib/FheType.sol` | Encrypted type enum definitions |

## Encrypted Types

**Boolean:**
- `ebool` - Encrypted boolean

**Unsigned Integers:**
- `euint4`, `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256` - Up to `euint2048`

**Signed Integers:**
- `eint8`, `eint16`, `eint32`, `eint64`, `eint128`, `eint256`

**Special Types:**
- `eaddress` - Encrypted Ethereum address
- `AsciiString` - Encrypted ASCII string

## Example Usage

```solidity
import {FHE, euint64} from "fhevm/lib/FHE.sol";

contract ConfidentialToken {
    mapping(address => euint64) private balances;

    function transfer(address to, euint64 amount) external {
        balances[msg.sender] = FHE.sub(balances[msg.sender], amount);
        balances[to] = FHE.add(balances[to], amount);
    }
}
```

## Core Operations

All standard operations are supported on encrypted types:

**Arithmetic:** `add`, `sub`, `mul`, `div`, `rem`, `min`, `max`
**Comparison:** `eq`, `ne`, `lt`, `le`, `gt`, `ge`
**Bitwise:** `and`, `or`, `xor`, `not`, `shl`, `shr`
**Control Flow:** `select` (ternary: `condition ? a : b`)

## Type System Design

Each encrypted type:
- Is a distinct Solidity type (strong typing)
- Internally stores a `bytes32` handle
- Operations return new encrypted values
- Cannot be implicitly converted to plaintext

## Key Files

- `lib/FHE.sol` - Primary import for developers
- `examples/EncryptedERC20.sol` - Reference implementation
- `codegen/` - Code generation for operator overloads

## Encrypted Type System

The library supports 88 encrypted types organized into categories. Each encrypted type internally stores a `bytes32` handle that references the actual ciphertext managed by the coprocessor.

### Type Categories

#### Boolean
| Type | Bit Size | FheType Enum | Clear Equivalent |
|------|----------|--------------|------------------|
| `ebool` | 2-bit | `Bool` (0) | `bool` |

#### Standard Unsigned Integers
| Type | Bit Size | FheType Enum | Clear Equivalent |
|------|----------|--------------|------------------|
| `euint8` | 8-bit | `Uint8` (2) | `uint8` |
| `euint16` | 16-bit | `Uint16` (3) | `uint16` |
| `euint32` | 32-bit | `Uint32` (4) | `uint32` |
| `euint64` | 64-bit | `Uint64` (5) | `uint64` |
| `euint128` | 128-bit | `Uint128` (6) | `uint128` |
| `euint256` | 256-bit | `Uint256` (8) | `uint256` |

#### Extended Unsigned Integers
| Type | Bit Size | FheType Enum | Clear Equivalent |
|------|----------|--------------|------------------|
| `euint2` | 2-bit | `Uint2` (12) | `uint8` |
| `euint4` | 4-bit | `Uint4` (1) | `uint8` |
| `euint6` | 6-bit | `Uint6` (13) | `uint8` |
| `euint10` | 10-bit | `Uint10` (14) | `uint16` |
| `euint12` | 12-bit | `Uint12` (15) | `uint16` |
| `euint14` | 14-bit | `Uint14` (16) | `uint16` |
| `euint24` | 24-bit | `Uint24` (34) | `uint32` |
| `euint40`-`euint248` | 40-248 bit | Various | Various |
| `euint512` | 512-bit | `Uint512` (9) | `bytes memory` |
| `euint1024` | 1024-bit | `Uint1024` (10) | `bytes memory` |
| `euint2048` | 2048-bit | `Uint2048` (11) | `bytes memory` |

#### Standard Signed Integers
| Type | Bit Size | FheType Enum | Clear Equivalent |
|------|----------|--------------|------------------|
| `eint8` | 8-bit | `Int8` (20) | `int8` |
| `eint16` | 16-bit | `Int16` (24) | `int16` |
| `eint32` | 32-bit | `Int32` (25) | `int32` |
| `eint64` | 64-bit | `Int64` (26) | `int64` |
| `eint128` | 128-bit | `Int128` (27) | `int128` |
| `eint256` | 256-bit | `Int256` (29) | `int256` |

Extended signed integers (`eint2`-`eint2048`) follow the same pattern.

#### Special Types
| Type | Bit Size | FheType Enum | Clear Equivalent |
|------|----------|--------------|------------------|
| `eaddress` | 160-bit | `Uint160` (7) | `address` |
| `AsciiString` | variable | `AsciiString` (30) | `string` |

#### External Input Types
For accepting encrypted inputs from users with proof verification:
- `externalEbool`, `externalEuint8`, `externalEuint16`, `externalEuint32`
- `externalEuint64`, `externalEuint128`, `externalEaddress`, `externalEuint256`

### Type Conversion Rules

**Converting plaintext to encrypted:**
```solidity
ebool b = FHE.asEbool(true);
euint64 amount = FHE.asEuint64(1000);
eaddress addr = FHE.asEaddress(msg.sender);
```

**Casting between encrypted types (widening):**
```solidity
euint8 small = FHE.asEuint8(42);
euint64 large = FHE.asEuint64(small);  // Safe: widening conversion
```

**Casting between encrypted types (narrowing):**
```solidity
euint64 large = FHE.asEuint64(1000);
euint8 small = FHE.asEuint8(large);    // Truncates to lower 8 bits
```

**Converting external inputs:**
```solidity
function deposit(externalEuint64 encAmount, bytes calldata proof) external {
    euint64 amount = FHE.fromExternal(encAmount, proof);
    // Now 'amount' is verified and usable
}
```

### Memory and Gas Implications

- **Internal representation**: All encrypted types store a `bytes32` handle
- **Gas costs**: Operations on larger types consume more gas
- **Storage**: Only the handle is stored on-chain; ciphertexts live in the coprocessor
- **Type efficiency**: Use the smallest type that fits your data (e.g., `euint8` for percentages)

### Type Safety Guarantees

1. **Strong typing**: Cannot implicitly convert between encrypted types
2. **Opaque handles**: Cannot directly access or manipulate the underlying ciphertext
3. **ACL enforcement**: Operations fail if caller lacks permission on the encrypted value
4. **Initialization checks**: Use `FHE.isInitialized(value)` to check if a value has been set

---

## Codegen System

The library uses a TypeScript-based code generation system to produce Solidity operator overloads for all type combinations.

### Directory Structure

```
codegen/
├── codegen.mjs              # CLI entry point (commander.js)
├── package.json             # npm configuration
├── src/
│   ├── main.ts              # Command handlers
│   ├── operators.ts         # 21 operator definitions
│   ├── fheTypeInfos.ts      # 84+ FHE type definitions
│   ├── templateFHEDotSol.ts # FHE.sol code generator
│   ├── templateImpDotSol.ts # Impl.sol code generator
│   ├── templateFheTypeDotSol.ts  # FheType.sol generator
│   ├── operatorsPrices.ts   # Gas cost definitions
│   └── templates/           # Solidity template files
│       ├── FHE.sol-template
│       ├── Impl.sol-template
│       └── FheType.sol-template
└── overloads/               # Generated test data (JSON)
    ├── library-solidity.json
    ├── host-contracts.json
    └── e2e.json
```

### How Operator Overloads Are Generated

**1. Template System**: Templates use `$${PLACEHOLDER}$$` markers that get replaced with generated code.

**2. Operator Definitions** (`operators.ts`): Each of the 21 operators specifies:
```typescript
{
  name: 'add',
  hasScalar: true,      // Supports plaintext operands
  hasEncrypted: true,   // Supports encrypted operands
  arguments: OperatorArguments.Binary,
  returnType: ReturnType.Euint,
  fheLibName: 'fheAdd'  // Coprocessor function name
}
```

**3. Type Definitions** (`fheTypeInfos.ts`): Each type specifies supported operators and bit lengths.

**4. Code Generation**: The generator produces all valid combinations:
- `FHE.add(euint8, euint8)` → `euint8`
- `FHE.add(euint8, uint8)` → `euint8` (scalar)
- `FHE.add(uint8, euint8)` → `euint8` (scalar, left)

### Supported Operators

| Category | Operators | Notes |
|----------|-----------|-------|
| Arithmetic | `add`, `sub`, `mul`, `div`, `rem` | `div`/`rem` only support scalar divisor |
| Bitwise | `and`, `or`, `xor`, `not` | Work on all integer types |
| Shift/Rotate | `shl`, `shr`, `rotl`, `rotr` | Shift amount can be scalar or encrypted |
| Comparison | `eq`, `ne`, `ge`, `gt`, `le`, `lt` | Return `ebool` |
| Min/Max | `min`, `max` | Return same type as inputs |
| Unary | `neg`, `not` | Single operand |

### Build Process

```bash
# Generate all library files (FHE.sol, Impl.sol, FheType.sol)
npm run codegen

# Regenerate test data with new random values
npm run codegen:overloads
```

### Extending the Library

**Adding a new operator:**
1. Add operator definition to `codegen/src/operators.ts`
2. Add template handler in `templateFHEDotSol.ts`
3. Run `npm run codegen` to regenerate

**Adding a new type:**
1. Add type definition to `codegen/src/fheTypeInfos.ts` with:
   - `type`: Name (e.g., `'Uint40'`)
   - `bitLength`: Size in bits
   - `supportedOperators`: List of allowed operators
   - `clearMatchingType`: Corresponding Solidity type
2. Run `npm run codegen` to regenerate

---

## Best Practices

### Contract Setup Pattern

Always initialize the coprocessor in your constructor:

```solidity
import {FHE} from "fhevm/lib/FHE.sol";
import {CoprocessorSetup} from "fhevm/examples/CoprocessorSetup.sol";

contract MyContract {
    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }
}
```

### Access Control Pattern

After any operation that creates or modifies an encrypted value, grant appropriate permissions:

```solidity
function updateBalance(address user, euint64 newBalance) internal {
    balances[user] = newBalance;

    // Allow this contract to use the value in future operations
    FHE.allowThis(newBalance);

    // Allow the user to access their own balance
    FHE.allow(newBalance, user);
}
```

For cross-contract calls, use transient permissions:

```solidity
function transferToOtherContract(address target, euint64 amount) external {
    FHE.allowTransient(amount, target);
    IOtherContract(target).receive(amount);
}
```

### Common Pitfalls

**1. Forgetting coprocessor setup:**
```solidity
// BAD: Operations will fail
contract Broken {
    function doSomething() external {
        euint64 a = FHE.asEuint64(100);  // Fails!
    }
}

// GOOD: Initialize in constructor
contract Working {
    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }
}
```

**2. Missing ACL permissions:**
```solidity
// BAD: Recipient can't use their balance
function transfer(address to, euint64 amount) external {
    balances[to] = FHE.add(balances[to], amount);
    // Missing: FHE.allow(balances[to], to);
}

// GOOD: Grant permissions after modification
function transfer(address to, euint64 amount) external {
    balances[to] = FHE.add(balances[to], amount);
    FHE.allowThis(balances[to]);
    FHE.allow(balances[to], to);
}
```

**3. Branching on encrypted conditions:**
```solidity
// BAD: Reveals whether condition is true!
if (FHE.decrypt(condition)) {
    doA();
} else {
    doB();
}

// GOOD: Use select to maintain confidentiality
euint64 result = FHE.select(condition, valueIfTrue, valueIfFalse);
```

**4. Using uninitialized values:**
```solidity
// BAD: Comparing uninitialized values gives undefined behavior
ebool winner = FHE.gt(bids[alice], bids[bob]);

// GOOD: Check initialization first
require(FHE.isInitialized(bids[alice]), "Alice hasn't bid");
require(FHE.isInitialized(bids[bob]), "Bob hasn't bid");
ebool winner = FHE.gt(bids[alice], bids[bob]);
```

### Performance Optimization

1. **Choose appropriate types**: Use `euint8` for small values, not `euint256`
2. **Batch operations**: Combine multiple operations where possible
3. **Minimize encrypted ops**: Do plaintext computation when privacy isn't needed
4. **Avoid redundant permissions**: Don't call `FHE.allow()` if permission already exists

### Testing Encrypted Contracts

1. **Use mock mode** for faster development iteration
2. **Test boundary conditions** (zero, max values, overflow scenarios)
3. **Verify ACL permissions** are correctly set in all code paths
4. **Test with multiple accounts** to ensure cross-account permissions work

---

## Examples Deep-Dive

### EncryptedERC20.sol Walkthrough

The `EncryptedERC20` contract demonstrates a complete confidential token implementation.

**State Variables:**
```solidity
mapping(address => euint64) internal balances;           // Encrypted balances
mapping(address => mapping(address => euint64)) internal allowances;  // Encrypted allowances
```

**Constructor - Coprocessor Setup:**
```solidity
constructor(string memory name_, string memory symbol_) Ownable(msg.sender) {
    FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    _name = name_;
    _symbol = symbol_;
}
```

**Minting - Adding Plaintext to Encrypted:**
```solidity
function mint(uint64 mintedAmount) public virtual onlyOwner {
    // Add plaintext amount to encrypted balance (auto-encrypts the scalar)
    balances[owner()] = FHE.add(balances[owner()], mintedAmount);

    // Grant permissions for the new balance
    FHE.allowThis(balances[owner()]);
    FHE.allow(balances[owner()], owner());

    _totalSupply = _totalSupply + mintedAmount;
}
```

**Transfer with External Input:**
```solidity
function transfer(
    address to,
    externalEuint64 encryptedAmount,  // User-provided encrypted amount
    bytes calldata inputProof          // Proof of valid encryption
) public virtual returns (bool) {
    // Verify and convert external input to internal encrypted value
    transfer(to, FHE.fromExternal(encryptedAmount, inputProof));
    return true;
}
```

**Internal Transfer - Conditional Logic:**
```solidity
function _transfer(address from, address to, euint64 amount, ebool isTransferable) internal {
    // Select: if transferable, use amount; else use 0
    euint64 transferValue = FHE.select(isTransferable, amount, FHE.asEuint64(0));

    // Update recipient balance
    euint64 newBalanceTo = FHE.add(balances[to], transferValue);
    balances[to] = newBalanceTo;
    FHE.allowThis(newBalanceTo);
    FHE.allow(newBalanceTo, to);

    // Update sender balance
    euint64 newBalanceFrom = FHE.sub(balances[from], transferValue);
    balances[from] = newBalanceFrom;
    FHE.allowThis(newBalanceFrom);
    FHE.allow(newBalanceFrom, from);
}
```

**Allowance Update - Combining Conditions:**
```solidity
function _updateAllowance(address owner, address spender, euint64 amount) internal returns (ebool) {
    euint64 currentAllowance = _allowance(owner, spender);

    // Check 1: Is amount <= allowance?
    ebool allowedTransfer = FHE.le(amount, currentAllowance);

    // Check 2: Does owner have enough balance?
    ebool canTransfer = FHE.le(amount, balances[owner]);

    // Combine conditions with AND
    ebool isTransferable = FHE.and(canTransfer, allowedTransfer);

    // Conditionally update allowance: subtract if transferable, keep same if not
    _approve(owner, spender,
        FHE.select(isTransferable, FHE.sub(currentAllowance, amount), currentAllowance));

    return isTransferable;
}
```

### HeadsOrTails.sol - Random Number Generation

Demonstrates encrypted randomness and public decryption:

```solidity
function headsOrTails(address headsPlayer, address tailsPlayer) external {
    // Generate encrypted random boolean (true = heads, false = tails)
    ebool headsOrTailsResult = FHE.randEbool();

    // Store in game state
    games[gameId] = Game({
        headsPlayer: headsPlayer,
        tailsPlayer: tailsPlayer,
        encryptedHasHeadWon: headsOrTailsResult,
        winner: address(0)
    });

    // Make result publicly decryptable (instead of oracle workflow)
    FHE.makePubliclyDecryptable(headsOrTailsResult);
}
```

Verifying decryption with proof:

```solidity
function checkWinner(uint256 gameId, bytes memory clearGameResult, bytes memory decryptionProof) public {
    // Decode the decrypted result
    bool decodedClearGameResult = abi.decode(clearGameResult, (bool));
    address winner = decodedClearGameResult ? games[gameId].headsPlayer : games[gameId].tailsPlayer;

    // Verify the proof
    bytes32[] memory cts = new bytes32[](1);
    cts[0] = FHE.toBytes32(games[gameId].encryptedHasHeadWon);
    FHE.checkSignatures(cts, clearGameResult, decryptionProof);

    games[gameId].winner = winner;
}
```

### Key Patterns Summary

| Pattern | Example | Use Case |
|---------|---------|----------|
| Encrypted state | `mapping(address => euint64) balances` | Store confidential per-user data |
| External input | `FHE.fromExternal(encAmount, proof)` | Accept user-provided encrypted values |
| Conditional select | `FHE.select(cond, a, b)` | Branch without revealing condition |
| Combined conditions | `FHE.and(cond1, cond2)` | Multiple checks without branching |
| Permission grant | `FHE.allow(value, addr)` | Enable address to use encrypted value |
| Random generation | `FHE.randEuint64()` | On-chain encrypted randomness |
| Public decrypt | `FHE.makePubliclyDecryptable(v)` | Reveal result without oracle |

### Additional Example References

- **Rand.sol**: Bounded random number generation with `FHE.randEuint8(upperBound)`
- **MakePubliclyDecryptable.sol**: Public decryption workflow patterns
- **BlindAuction** (in docs/examples): Complete sealed-bid auction with encrypted bid comparison

---

**Related:**
- [Host Contracts](host-contracts.md) - Underlying symbolic execution engine
- [Key Concepts](../key-concepts.md) - Understanding handles and symbolic execution
- [Workflows: Input Verification](../workflows/input-verification.md) - How users submit encrypted inputs
