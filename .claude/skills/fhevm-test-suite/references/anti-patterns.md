# Anti-Patterns - Test Suite

Patterns that trigger CHANGES_REQUESTED in test code review.

---

## Flakiness Anti-Patterns

### 1. Timing-Dependent Assertions

```typescript
// BAD: Race condition
it("should emit event", async function () {
  contract.transfer(bob.address, amount);
  await sleep(100); // Hope the event is emitted
  expect(events).to.include("Transfer");
});

// GOOD: Wait for specific condition
it("should emit event", async function () {
  await expect(contract.transfer(bob.address, amount))
    .to.emit(contract, "Transfer")
    .withArgs(alice.address, bob.address);
});
```

### 2. Non-Deterministic Random

```rust
// BAD: Different results each run
#[test]
fn test_random_transfer() {
    let amount = rand::random::<u64>();
    // Results vary each run
}

// GOOD: Seeded random
#[test]
fn test_random_transfer() {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let amount = rng.gen::<u64>();
    // Reproducible
}
```

### 3. External Service Dependencies

```typescript
// BAD: Depends on external service
it("should fetch price", async function () {
  const price = await fetch("https://api.external.com/price");
  expect(price).to.be.greaterThan(0);
});

// GOOD: Mock external services
it("should fetch price", async function () {
  mockServer.onGet("/price").reply(200, { price: 100 });
  const price = await contract.getPrice();
  expect(price).to.equal(100);
});
```

### 4. Order-Dependent Tests

```typescript
// BAD: Tests depend on execution order
describe("User tests", function () {
  it("should create user", async function () {
    user = await createUser(); // Sets global
  });

  it("should update user", async function () {
    await updateUser(user); // Depends on previous test
  });
});

// GOOD: Independent tests
describe("User tests", function () {
  let user: User;

  beforeEach(async function () {
    user = await createUser();
  });

  it("should update user", async function () {
    await updateUser(user);
  });
});
```

---

## Coverage Anti-Patterns

### 1. Happy Path Only

```typescript
// BAD: Only tests success
describe("transfer", function () {
  it("should transfer tokens", async function () {
    await contract.transfer(bob.address, 100);
    expect(await contract.balanceOf(bob.address)).to.equal(100);
  });
});

// GOOD: Include error cases
describe("transfer", function () {
  it("should transfer tokens with sufficient balance", async function () {
    await contract.transfer(bob.address, 100);
    expect(await contract.balanceOf(bob.address)).to.equal(100);
  });

  it("should revert with zero address recipient", async function () {
    await expect(contract.transfer(ethers.ZeroAddress, 100)).to.be.revertedWithCustomError(
      contract,
      "ZeroAddress"
    );
  });

  it("should return zero transfer with insufficient balance", async function () {
    // Test FHE.select behavior
  });
});
```

### 2. Missing Edge Cases

```rust
// BAD: Only tests middle values
#[test]
fn test_division() {
    assert_eq!(divide(100, 10), 10);
}

// GOOD: Include edge cases
#[test]
fn test_division_normal() {
    assert_eq!(divide(100, 10), 10);
}

#[test]
fn test_division_by_one() {
    assert_eq!(divide(100, 1), 100);
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_division_by_zero() {
    divide(100, 0);
}

#[test]
fn test_division_with_remainder() {
    assert_eq!(divide(100, 3), 33);
}
```

---

## Cleanup Anti-Patterns

### 1. Missing Teardown

```typescript
// BAD: State leaks between tests
describe("Database tests", function () {
  it("should insert record", async function () {
    await db.insert({ id: 1, value: "test" });
    // Never cleaned up
  });
});

// GOOD: Proper cleanup
describe("Database tests", function () {
  afterEach(async function () {
    await db.clear();
  });

  it("should insert record", async function () {
    await db.insert({ id: 1, value: "test" });
  });
});
```

### 2. Resource Leaks

```rust
// BAD: Connection never closed
#[tokio::test]
async fn test_database_query() {
    let pool = create_pool().await;
    // Use pool
    // Pool never closed
}

// GOOD: Proper cleanup
#[tokio::test]
async fn test_database_query() {
    let pool = create_pool().await;
    // Use pool
    pool.close().await;
}
```

---

## Assertion Anti-Patterns

### 1. Weak Assertions

```typescript
// BAD: Too weak
it("should return something", async function () {
  const result = await contract.getValue();
  expect(result).to.exist;
});

// GOOD: Specific assertions
it("should return initial value", async function () {
  const result = await contract.getValue();
  expect(result).to.equal(100n);
});
```

### 2. Missing Assertions

```typescript
// BAD: No assertions
it("should not throw", async function () {
  await contract.doSomething();
});

// GOOD: Explicit assertions
it("should update state without throwing", async function () {
  await contract.doSomething();
  expect(await contract.getState()).to.equal("updated");
});
```

### 3. Testing Implementation Details

```typescript
// BAD: Tests internal state
it("should store in _balances mapping", async function () {
  await contract.deposit(100);
  // Accesses internal storage - brittle
  const slot = await ethers.provider.getStorageAt(contract.address, 0);
});

// GOOD: Tests behavior
it("should increase balance on deposit", async function () {
  await contract.deposit(100);
  expect(await contract.balanceOf(alice.address)).to.equal(100);
});
```

---

## Fixture Anti-Patterns

### 1. Outdated Fixtures

```typescript
// BAD: Hardcoded fixture without version
const fixture = {
  /* old format */
};

// GOOD: Versioned fixtures
import fixture from "./fixtures/v2/transfer_data.json";
// Or generate fresh
const fixture = generateFixture();
```

### 2. Magic Numbers

```typescript
// BAD: Unexplained numbers
it("should process correctly", async function () {
  await contract.process(0x7f3a2b);
});

// GOOD: Named constants with explanation
const VALID_HANDLE = 0x7f3a2b; // Test handle from fixture generator

it("should process valid handle", async function () {
  await contract.process(VALID_HANDLE);
});
```
