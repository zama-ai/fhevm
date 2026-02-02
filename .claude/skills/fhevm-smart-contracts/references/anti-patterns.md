# Anti-Patterns - Smart Contracts

Patterns that trigger CHANGES_REQUESTED in Solidity code review.

---

## Error Handling Anti-Patterns

### 1. Using require with String Messages

```solidity
// BAD: String-based require (gas inefficient, outdated)
function withdraw(uint256 amount) external {
    require(balances[msg.sender] >= amount, "Insufficient balance");
    require(amount > 0, "Amount must be positive");
}

// GOOD: Custom errors (gas efficient, modern)
error InsufficientBalance(uint256 available, uint256 required);
error InvalidAmount();

function withdraw(uint256 amount) external {
    if (balances[msg.sender] < amount) {
        revert InsufficientBalance(balances[msg.sender], amount);
    }
    if (amount == 0) {
        revert InvalidAmount();
    }
}
```

### 2. Silent Failures

```solidity
// BAD: Silently returns false
function transfer(address to, uint256 amount) external returns (bool) {
    if (balances[msg.sender] < amount) {
        return false;  // Caller might not check!
    }
    // ...
}

// GOOD: Revert on failure
function transfer(address to, uint256 amount) external returns (bool) {
    if (balances[msg.sender] < amount) {
        revert InsufficientBalance(balances[msg.sender], amount);
    }
    // ...
    return true;
}
```

---

## FHE Anti-Patterns

### 1. Using if on Encrypted Values

```solidity
// BAD: Cannot branch on encrypted bool
function conditionalTransfer(ebool canTransfer, euint64 amount) internal {
    if (FHE.decrypt(canTransfer)) {  // NEVER decrypt for branching
        // transfer
    }
}

// GOOD: Use FHE.select
function conditionalTransfer(ebool canTransfer, euint64 amount) internal {
    euint64 transferAmount = FHE.select(canTransfer, amount, FHE.asEuint64(0));
    // Always transfer, but amount is 0 if canTransfer is false
}
```

### 2. Forgetting ACL Permissions

```solidity
// BAD: Value stored but no permissions set
function updateBalance(euint64 newBalance) internal {
    balances[msg.sender] = newBalance;
    // User can't access their own balance!
}

// GOOD: Always set permissions after storing
function updateBalance(euint64 newBalance) internal {
    balances[msg.sender] = newBalance;
    FHE.allowThis(newBalance);           // Contract can use it
    FHE.allow(newBalance, msg.sender);   // User can access
}
```

### 3. Marking FHE Functions as view

```solidity
// BAD: FHE operations cost gas, can't be view
function computeSum(euint64 a, euint64 b) public view returns (euint64) {
    return FHE.add(a, b);  // ERROR: state-mutating in view function
}

// GOOD: No view modifier for FHE operations
function computeSum(euint64 a, euint64 b) public returns (euint64) {
    return FHE.add(a, b);
}
```

### 4. Using Encrypted Divisors

```solidity
// BAD: div/rem only support plaintext divisors
function divide(euint64 a, euint64 b) public returns (euint64) {
    return FHE.div(a, b);  // ERROR: b must be plaintext
}

// GOOD: Plaintext divisor only
function divideByConstant(euint64 a, uint64 divisor) public returns (euint64) {
    return FHE.div(a, divisor);
}
```

### 5. Non-Power-of-2 Random Bounds

```solidity
// BAD: upperBound must be power of 2
function randomInRange() public returns (euint8) {
    return FHE.randEuint8(100);  // ERROR: 100 is not power of 2
}

// GOOD: Use power of 2
function randomInRange() public returns (euint8) {
    return FHE.randEuint8(128);  // 2^7 = 128
}
```

---

## Upgrade Anti-Patterns

### 1. Breaking Storage Layout

```solidity
// BAD: Inserting variable breaks storage layout
contract MyContractV2 is MyContractV1 {
    uint256 public newVariable;  // Inserted before existing variables
    // All subsequent storage slots shift!
}

// GOOD: Append to end or use storage gaps
contract MyContractV1 {
    uint256 public existingVar;
    uint256[49] private __gap;  // Reserve space for upgrades
}

contract MyContractV2 is MyContractV1 {
    // Use gap space
    uint256 public newVariable;
    uint256[48] private __gap;  // Reduce gap by 1
}
```

### 2. Missing Initializer Modifier

```solidity
// BAD: Can be called multiple times
function initialize(address admin) public {
    _admin = admin;
}

// GOOD: Use initializer modifier
function initialize(address admin) public initializer {
    _admin = admin;
}
```

### 3. Calling Parent Constructor in Upgradeable Contract

```solidity
// BAD: Constructor in upgradeable contract
contract MyUpgradeable is UUPSUpgradeable {
    constructor() {
        _disableInitializers();  // This is OK
    }

    constructor(address admin) {  // BAD: Sets state in constructor
        _admin = admin;
    }
}

// GOOD: Use initialize function
contract MyUpgradeable is UUPSUpgradeable {
    function initialize(address admin) public initializer {
        _admin = admin;
    }
}
```

---

## Access Control Anti-Patterns

### 1. Single-Step Ownership Transfer

```solidity
// BAD: Single-step transfer can lose ownership
function transferOwnership(address newOwner) external onlyOwner {
    owner = newOwner;  // If newOwner is wrong, ownership is lost
}

// GOOD: Use Ownable2Step
import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract MyContract is Ownable2Step {
    // Requires acceptOwnership() call from new owner
}
```

### 2. Missing Zero Address Check

```solidity
// BAD: Can set admin to zero address
function setAdmin(address newAdmin) external onlyOwner {
    admin = newAdmin;
}

// GOOD: Validate input
error ZeroAddress();

function setAdmin(address newAdmin) external onlyOwner {
    if (newAdmin == address(0)) revert ZeroAddress();
    admin = newAdmin;
}
```

---

## Testing Anti-Patterns

### 1. Missing Negative Tests

```solidity
// BAD: Only tests happy path
it("should transfer tokens", async function() {
    await contract.transfer(recipient, 100);
    expect(await contract.balanceOf(recipient)).to.equal(100);
});

// GOOD: Include negative tests
it("should revert on insufficient balance", async function() {
    await expect(contract.transfer(recipient, 1000000))
        .to.be.revertedWithCustomError(contract, "InsufficientBalance");
});
```

### 2. Not Testing ACL Permissions

```solidity
// BAD: Assumes ACL works
it("should update balance", async function() {
    await contract.deposit(100);
    // Doesn't verify user can access their balance
});

// GOOD: Verify ACL permissions
it("should allow user to access their balance", async function() {
    await contract.deposit(100);
    // Verify user has permission to decrypt
    const canAccess = await contract.canAccessBalance(user.address);
    expect(canAccess).to.be.true;
});
```
