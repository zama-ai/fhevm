# Best Practices - Smart Contracts

Current best practices for fhevm Solidity development with sources.

---

## Custom Errors (Solidity 0.8.4+)

Use custom errors instead of require strings:

```solidity
// Define at contract level
error InsufficientBalance(uint256 available, uint256 required);
error UnauthorizedCaller(address caller);
error ZeroAddress();

// Use in functions
function transfer(address to, uint256 amount) external {
    if (to == address(0)) revert ZeroAddress();
    if (balances[msg.sender] < amount) {
        revert InsufficientBalance(balances[msg.sender], amount);
    }
}
```

**Benefits**: ~50 gas savings per error, better ABI encoding.

**Source**: [Solidity 0.8.4 Release](https://blog.soliditylang.org/2021/04/21/custom-errors/)

---

## Access Control

### Use Ownable2Step Over Ownable

```solidity
import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract MyContract is Ownable2Step {
    constructor() Ownable(msg.sender) {}
}
```

**Why**: Prevents accidental ownership loss via typo.

**Source**: [OpenZeppelin Access Control](https://docs.openzeppelin.com/contracts/5.x/access-control)

### Role-Based Access for Complex Permissions

```solidity
import "@openzeppelin/contracts/access/AccessControl.sol";

contract MyContract is AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        // ...
    }
}
```

**Source**: [OpenZeppelin AccessControl](https://docs.openzeppelin.com/contracts/5.x/access-control#role-based-access-control)

---

## Upgrade Patterns

### UUPS Proxy (Preferred)

```solidity
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

contract MyUpgradeable is UUPSUpgradeable, OwnableUpgradeable {
    function initialize(address owner) public initializer {
        __Ownable_init(owner);
        __UUPSUpgradeable_init();
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner
    {}
}
```

**Why UUPS over Transparent**: Lower deployment cost, upgrade logic in implementation.

**Source**: [OpenZeppelin UUPS](https://docs.openzeppelin.com/contracts/5.x/api/proxy#UUPSUpgradeable)

### Storage Gaps

```solidity
contract MyContractV1 {
    uint256 public value;
    uint256[49] private __gap;  // 50 - 1 used = 49 remaining
}

contract MyContractV2 is MyContractV1 {
    uint256 public newValue;
    uint256[48] private __gap;  // Reduced by 1
}
```

**Source**: [OpenZeppelin Upgrades Storage](https://docs.openzeppelin.com/upgrades-plugins/1.x/writing-upgradeable#storage-gaps)

---

## FHE-Specific Patterns

### Always Set ACL After Storing

```solidity
function deposit(externalEuint64 amount, bytes calldata proof) external {
    euint64 validAmount = FHE.fromExternal(amount, proof);
    euint64 newBalance = FHE.add(balances[msg.sender], validAmount);

    balances[msg.sender] = newBalance;

    // ALWAYS do this after storing
    FHE.allowThis(newBalance);
    FHE.allow(newBalance, msg.sender);
}
```

### Use FHE.select for Conditional Logic

```solidity
function safeTransfer(
    address to,
    euint64 amount,
    ebool canTransfer
) internal {
    // Compute actual transfer amount (0 if canTransfer is false)
    euint64 actualAmount = FHE.select(canTransfer, amount, FHE.asEuint64(0));

    // Update balances (always executes, but with 0 if not allowed)
    balances[msg.sender] = FHE.sub(balances[msg.sender], actualAmount);
    balances[to] = FHE.add(balances[to], actualAmount);

    // Set permissions
    FHE.allowThis(balances[msg.sender]);
    FHE.allow(balances[msg.sender], msg.sender);
    FHE.allowThis(balances[to]);
    FHE.allow(balances[to], to);
}
```

### Check Initialization

```solidity
function getBalance(address user) public returns (euint64) {
    euint64 balance = balances[user];

    // Return encrypted zero if not initialized
    if (!FHE.isInitialized(balance)) {
        return FHE.asEuint64(0);
    }

    return balance;
}
```

---

## Testing

### Dual Testing Strategy

Use both Hardhat and Foundry:

**Hardhat** (TypeScript): Integration tests, fhevmjs testing

```typescript
it("should transfer encrypted tokens", async function () {
  const input = fhevm.createEncryptedInput(contract.address, alice.address);
  input.add64(100n);
  const encrypted = await input.encrypt();

  await contract.transfer(bob.address, encrypted.handles[0], encrypted.inputProof);
});
```

**Foundry** (Solidity): Unit tests, fuzzing, gas optimization

```solidity
function testFuzz_Transfer(uint64 amount) public {
    vm.assume(amount <= initialBalance);
    // ...
}
```

**Source**: [Foundry Book](https://book.getfoundry.sh/), [Hardhat Docs](https://hardhat.org/docs)

---

## Gas Optimization

### Pack Storage Variables

```solidity
// BAD: Uses 3 storage slots
contract Unoptimized {
    uint128 a;  // Slot 0
    uint256 b;  // Slot 1
    uint128 c;  // Slot 2
}

// GOOD: Uses 2 storage slots
contract Optimized {
    uint128 a;  // Slot 0 (half)
    uint128 c;  // Slot 0 (half)
    uint256 b;  // Slot 1
}
```

### Use immutable for Constructor-Set Values

```solidity
contract MyContract {
    address public immutable admin;  // Set once in constructor

    constructor(address _admin) {
        admin = _admin;
    }
}
```

---

## Events

### Emit Events for State Changes

```solidity
event Transfer(address indexed from, address indexed to);
event Approval(address indexed owner, address indexed spender);

function transfer(address to, euint64 amount) external {
    // ... transfer logic ...

    // Note: Don't emit encrypted values!
    emit Transfer(msg.sender, to);
}
```

### Index Important Fields

```solidity
// Up to 3 indexed parameters
event Transfer(
    address indexed from,    // Indexed - searchable
    address indexed to,      // Indexed - searchable
    uint256 timestamp        // Not indexed - cheaper
);
```

**Source**: [Solidity Events](https://docs.soliditylang.org/en/latest/contracts.html#events)
