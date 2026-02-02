# Architecture - Test Suite

Overview of the fhevm testing infrastructure.

---

## Test Organization

```text
fhevm/
├── coprocessor/
│   └── tests/           # Rust unit tests
│       ├── worker_tests.rs
│       └── scheduler_tests.rs
├── kms-connector/
│   └── tests/           # Rust integration tests
│       └── kms_tests.rs
├── library-solidity/
│   └── test/            # Hardhat tests
│       └── ConfidentialERC20.test.ts
├── host-contracts/
│   └── test/            # Foundry tests
│       └── ACL.t.sol
└── test-suite/          # E2E tests
    ├── e2e/
    │   ├── transfer.test.ts
    │   └── decryption.test.ts
    ├── fixtures/
    │   └── v2/
    └── utils/
        └── helpers.ts
```

---

## Test Types

### Unit Tests

**Location**: Within each component
**Framework**: Native to language (Rust test, Foundry, Jest)
**Purpose**: Test individual functions in isolation

```rust
// coprocessor/tests/worker_tests.rs
#[test]
fn test_worker_processes_valid_operation() {
    let worker = Worker::new(MockScheduler::new());
    let result = worker.process(valid_operation());
    assert!(result.is_ok());
}
```

### Integration Tests

**Location**: Within each component
**Framework**: Native frameworks with mocks
**Purpose**: Test component interactions

```rust
// kms-connector/tests/integration_tests.rs
#[tokio::test]
async fn test_listener_creates_jobs_for_events() {
    let mock_contract = MockGatewayContract::new();
    let mock_queue = MockJobQueue::new();

    let listener = GwListener::new(mock_contract, mock_queue.clone());
    listener.process_block(100).await.unwrap();

    assert_eq!(mock_queue.len(), 1);
}
```

### E2E Tests

**Location**: `test-suite/e2e/`
**Framework**: Hardhat + fhevmjs
**Purpose**: Test full system flows

```typescript
// test-suite/e2e/transfer.test.ts
describe("E2E: Confidential Transfer", function () {
  it("should complete transfer from mint to final balance", async function () {
    // 1. Deploy contract
    const token = await deployConfidentialToken();

    // 2. Mint tokens
    await token.mint(alice.address, 1000);

    // 3. Create encrypted transfer
    const input = fhevm.createEncryptedInput(token.address, alice.address);
    input.add64(100n);
    const encrypted = await input.encrypt();

    // 4. Execute transfer
    await token.transfer(bob.address, encrypted.handles[0], encrypted.inputProof);

    // 5. Verify final state
    // Note: Actual balance verification requires decryption
  });
});
```

### Contract Tests

**Location**: Component test directories
**Frameworks**: Hardhat (TypeScript) + Foundry (Solidity)

#### Hardhat Pattern

```typescript
// library-solidity/test/ConfidentialERC20.test.ts
import { expect } from "chai";
import { ethers } from "hardhat";
import { fhevm } from "hardhat";

describe("ConfidentialERC20", function () {
  let contract: ConfidentialERC20;

  beforeEach(async function () {
    const Factory = await ethers.getContractFactory("ConfidentialERC20");
    contract = await Factory.deploy("Test Token", "TEST");
  });

  it("should mint tokens", async function () {
    await contract.mint(1000);
    expect(await contract.totalSupply()).to.equal(1000);
  });
});
```

#### Foundry Pattern

```solidity
// host-contracts/test/ACL.t.sol
pragma solidity ^0.8.24;

import "forge-std/Test.sol";
import "../contracts/ACL.sol";

contract ACLTest is Test {
    ACL public acl;

    function setUp() public {
        acl = new ACL();
    }

    function test_AllowSetsPermission() public {
        bytes32 handle = bytes32(uint256(1));
        address user = address(0x1);

        acl.allow(handle, user);

        assertTrue(acl.isAllowed(handle, user));
    }

    function testFuzz_AllowArbitraryHandle(bytes32 handle) public {
        address user = address(0x1);
        acl.allow(handle, user);
        assertTrue(acl.isAllowed(handle, user));
    }
}
```

---

## Test Infrastructure

### Mock Services

```text
test-suite/mocks/
├── MockKMS.ts           # KMS service mock
├── MockCoprocessor.ts   # Coprocessor mock
└── MockBlockchain.ts    # Blockchain mock for fast tests
```

### Fixtures

```text
test-suite/fixtures/
├── v1/                  # Legacy format (deprecated)
└── v2/                  # Current format
    ├── ciphertexts/
    ├── proofs/
    └── signatures/
```

### Test Utilities

```typescript
// test-suite/utils/helpers.ts
export async function deployFreshContract(): Promise<Contract> {
  const Factory = await ethers.getContractFactory("ConfidentialERC20");
  return await Factory.deploy("Test", "TST");
}

export async function createEncryptedAmount(
  contract: Contract,
  signer: SignerWithAddress,
  amount: bigint
): Promise<EncryptedInput> {
  const input = fhevm.createEncryptedInput(await contract.getAddress(), signer.address);
  input.add64(amount);
  return await input.encrypt();
}
```

---

## CI Integration

### Test Workflows

| Workflow          | Tests Run                    | Trigger         |
| ----------------- | ---------------------------- | --------------- |
| `test-unit`       | Unit tests only              | Every PR        |
| `test-integration`| Integration tests            | Every PR        |
| `test-e2e`        | Full E2E suite               | Main branch     |
| `test-nightly`    | Extended test suite          | Nightly         |

### Test Matrix

```yaml
strategy:
  matrix:
    component: [coprocessor, kms-connector, library-solidity]
    test-type: [unit, integration]
steps:
  - run: ./run-tests.sh ${{ matrix.component }} ${{ matrix.test-type }}
```

---

## Coverage Reporting

### Rust Coverage

```bash
cargo tarpaulin --out Html --output-dir coverage/
```

### Solidity Coverage

```bash
npx hardhat coverage
forge coverage --report lcov
```

### Combined Reports

Coverage reports are aggregated and published to:

- PR comments (summary)
- Codecov (detailed)
- CI artifacts (full reports)
