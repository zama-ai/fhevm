# Best Practices - Test Suite

Current best practices for testing fhevm components with sources.

---

## Dual Testing Strategy (Solidity)

Use both Hardhat and Foundry for comprehensive coverage:

### Hardhat for Integration Tests

```typescript
import { expect } from "chai";
import { ethers } from "hardhat";
import { fhevm } from "hardhat";

describe("ConfidentialERC20", function () {
  it("should handle encrypted transfer", async function () {
    const [alice, bob] = await ethers.getSigners();
    const contract = await deployContract();

    // Create encrypted input using fhevmjs
    const input = fhevm.createEncryptedInput(await contract.getAddress(), alice.address);
    input.add64(100n);
    const encrypted = await input.encrypt();

    await contract.transfer(bob.address, encrypted.handles[0], encrypted.inputProof);

    // Verify event emitted
    await expect(contract.transfer(bob.address, encrypted.handles[0], encrypted.inputProof))
      .to.emit(contract, "Transfer")
      .withArgs(alice.address, bob.address);
  });
});
```

**Source**: [Hardhat Testing](https://hardhat.org/hardhat-runner/docs/guides/test-contracts)

### Foundry for Unit Tests and Fuzzing

```solidity
pragma solidity ^0.8.24;

import "forge-std/Test.sol";

contract ACLTest is Test {
    function test_AllowSetsPermission() public {
        bytes32 handle = bytes32(uint256(1));
        acl.allow(handle, address(this));
        assertTrue(acl.isAllowed(handle, address(this)));
    }

    function testFuzz_AllowArbitraryUser(address user) public {
        vm.assume(user != address(0));
        bytes32 handle = bytes32(uint256(1));

        acl.allow(handle, user);

        assertTrue(acl.isAllowed(handle, user));
    }

    function testFail_AllowZeroAddress() public {
        acl.allow(bytes32(uint256(1)), address(0));
    }
}
```

**Source**: [Foundry Book - Testing](https://book.getfoundry.sh/forge/tests)

---

## Rust Testing Patterns

### Async Test Setup

```rust
use tokio::test;

#[tokio::test]
async fn test_worker_processes_job() {
    let pool = setup_test_db().await;
    let worker = Worker::new(pool.clone());

    let job = create_test_job();
    let result = worker.process(job).await;

    assert!(result.is_ok());
    cleanup_test_db(pool).await;
}
```

**Source**: [Tokio Testing](https://tokio.rs/tokio/topics/testing)

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_serialization_roundtrip(data: Vec<u8>) {
        let encoded = encode(&data);
        let decoded = decode(&encoded)?;
        prop_assert_eq!(data, decoded);
    }

    #[test]
    fn test_addition_commutative(a: u64, b: u64) {
        prop_assert_eq!(add(a, b), add(b, a));
    }
}
```

**Source**: [proptest](https://docs.rs/proptest)

### Test Fixtures with rstest

```rust
use rstest::*;

#[fixture]
fn test_config() -> Config {
    Config::default()
}

#[fixture]
fn mock_kms() -> MockKms {
    MockKms::new()
}

#[rstest]
fn test_decryption(test_config: Config, mock_kms: MockKms) {
    let service = Service::new(test_config, mock_kms);
    assert!(service.decrypt(test_handle()).is_ok());
}
```

**Source**: [rstest](https://docs.rs/rstest)

---

## Mock Patterns

### External Service Mocking

```rust
use mockall::*;

#[automock]
trait KmsClient {
    async fn get_key(&self, key_id: &str) -> Result<Key>;
    async fn sign(&self, data: &[u8]) -> Result<Signature>;
}

#[tokio::test]
async fn test_with_mock_kms() {
    let mut mock = MockKmsClient::new();
    mock.expect_get_key()
        .with(eq("test-key"))
        .returning(|_| Ok(test_key()));

    let service = Service::new(mock);
    let result = service.process().await;

    assert!(result.is_ok());
}
```

**Source**: [mockall](https://docs.rs/mockall)

### Contract Mocking (Solidity)

```solidity
contract MockACL {
    mapping(bytes32 => mapping(address => bool)) private _allowed;

    function allow(bytes32 handle, address user) external {
        _allowed[handle][user] = true;
    }

    function isAllowed(bytes32 handle, address user) external view returns (bool) {
        return _allowed[handle][user];
    }
}

contract MyContractTest is Test {
    MockACL mockAcl;

    function setUp() public {
        mockAcl = new MockACL();
        // Inject mock
    }
}
```

---

## Test Organization

### Arrange-Act-Assert Pattern

```typescript
it("should transfer tokens", async function () {
  // Arrange
  const initialBalance = 1000n;
  const transferAmount = 100n;
  await contract.mint(alice.address, initialBalance);

  // Act
  const encrypted = await createEncryptedAmount(contract, alice, transferAmount);
  await contract.transfer(bob.address, encrypted.handles[0], encrypted.inputProof);

  // Assert
  const aliceBalance = await contract.balanceOf(alice.address);
  expect(aliceBalance).to.be.lessThan(initialBalance);
});
```

### Given-When-Then for BDD

```rust
#[test]
fn given_valid_acl_when_decrypt_requested_then_succeeds() {
    // Given
    let service = setup_service_with_valid_acl();

    // When
    let result = service.request_decryption(test_handle(), test_user());

    // Then
    assert!(result.is_ok());
}
```

---

## Snapshot Testing

For complex outputs:

```rust
use insta::assert_snapshot;

#[test]
fn test_error_message_format() {
    let error = create_error();
    assert_snapshot!(format!("{}", error));
}
```

**Source**: [insta](https://docs.rs/insta)

---

## Test Data Generation

### Deterministic Generation

```typescript
import { faker } from "@faker-js/faker";

function setupTestData() {
  // Seed for reproducibility
  faker.seed(12345);

  return {
    address: faker.finance.ethereumAddress(),
    amount: faker.number.bigInt({ min: 1n, max: 1000000n }),
  };
}
```

### Factory Pattern

```rust
fn create_test_job() -> Job {
    Job {
        id: 1,
        handle: test_handle(),
        requester: test_address(),
        status: JobStatus::Pending,
        created_at: Utc::now(),
    }
}

fn create_test_job_with_status(status: JobStatus) -> Job {
    let mut job = create_test_job();
    job.status = status;
    job
}
```

---

## Coverage Best Practices

### Meaningful Coverage

```typescript
// BAD: Coverage without meaning
it("should not throw", async function () {
  await contract.doSomething();
});

// GOOD: Coverage with assertions
it("should update state correctly", async function () {
  const before = await contract.getState();
  await contract.doSomething();
  const after = await contract.getState();
  expect(after).to.not.equal(before);
  expect(after).to.equal(expectedState);
});
```

### Branch Coverage

```rust
// Test all branches
#[test]
fn test_handle_error_recoverable() {
    let result = handle_error(Error::Recoverable(timeout()));
    assert!(result.should_retry());
}

#[test]
fn test_handle_error_irrecoverable() {
    let result = handle_error(Error::Irrecoverable(invalid_input()));
    assert!(!result.should_retry());
}
```
