# 📘 Introduction

In this tutorial, you'll set up a minimal Hardhat development environment, then write and test a simple Solidity smart contract. This forms the foundation for working with more advanced contract types.

In the next tutorial, you'll learn how to transition from a standard Solidity contract to an FHEVM-compatible Solidity contract.

### 🧠 What you'll learn

- How to write a minimal Solidity contract using Hardhat
- How to test that contract using TypeScript and Hardhat’s testing framework

# 🚀 Create an Empty FHEVM Hardhat Project

Let’s begin by creating a fresh Hardhat project preconfigured for FHEVM.

{% content-ref url="../hardhat/setup.md#optional-start-a-new-empty-fhevm-project" %}➡️ Follow the setup guide here{% endcontent-ref %}

# Step 1: Create `Counter.sol`

Go to your project's `contracts` directory

```sh
cd <your-project-root-directory>/contracts
```

From there, create a new file named `Counter.sol` and copy/paste the following Solidity code in it.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title A simple counter contract
contract Counter {
    uint32 private _count;

    /// @notice Returns the current count
    function getCount() external view returns (uint32) {
        return _count;
    }

    /// @notice Increments the counter by 1
    function increment(uint32 value) external {
        _count += value;
    }

    /// @notice Decrements the counter by 1
    function decrement(uint32 value) external {
        require(_count >= value, "Counter: cannot decrement below zero");
        _count -= value;
    }
}
```

# Step 2: Compile `Counter.sol`

From your project's root directory, run:

```sh
npx hardhat compile
```

🎉 Great! Your Smart Contract is now compiled. 

Let's set up the Hasrdhat test environment.

# Step 3: Create `test/Counter.ts`

Go to your project's `test` directory

```sh
cd <your-project-root-directory>/test
```

From there, create a new file named `Counter.ts` and copy/paste the following Typescript skeleton code in it.

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

describe("Counter", function () {
  it("Empty test", async function () {
    console.log("Cool! The test basic skeleton is running!");
  });
});
```

The file contains the following:
- all the required `import` statements we will need during the various tests
- The `chai` basic statements to run a first empty test named `Empty test`

# Step 4: Run `test/Counter.ts`

From your project's root directory, run:

```sh
npx hardhat test
```

Output: 

```sh
  Counter
Cool! The test basic skeleton is running!
    ✔ Empty test


  1 passing (1ms)
```

🎉 Great! Your Hardhat test environment is properly setup. 

# Step 5: Set up the test signers

Before interacting with smart contracts in Hardhat tests, we need to initialize signers.

> [!Note]
>
> In the context of Ethereum development, a signer represents an entity (usually a wallet) that can send transactions and sign messages.
> In Hardhat, `ethers.getSigners()` returns a list of pre-funded test accounts.

We’ll define three named signers for convenience:

- `owner` — the deployer of the contract
- `alice` and `bob` — additional simulated users

### ✍️ Replace the contents of `test/Counter.ts` with the following:

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

type Signers = {
  owner: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

describe("Counter", function () {
  let signers: Signers;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  it("should work", async function () {
    console.log(`address of user owner is ${signers.owner.address}`);
    console.log(`address of user alice is ${signers.alice.address}`);
    console.log(`address of user bob is ${signers.bob.address}`);
  });
});
```

---

### ▶️ Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

### ✅ Expected Output

```sh
  Counter
address of user owner is 0x37AC010c1c566696326813b840319B58Bb5840E4
address of user alice is 0xD9F9298BbcD72843586e7E08DAe577E3a0aC8866
address of user bob is 0x3f0CdAe6ebd93F9F776BCBB7da1D42180cC8fcC1
    ✔ should work


  1 passing (2ms)
```

# Step 6: Deploy the `Counter.sol` contract

Now that we have our signers set up, we can deploy the smart contract.

To ensure isolated and deterministic tests, we should deploy a fresh instance of `Counter.sol` before each test. This avoids any side effects from previous tests.

The standard approach is to define a `deployFixture()` function that handles contract deployment.

```ts
async function deployFixture() {
  const factory = (await ethers.getContractFactory("Counter")) as Counter__factory;
  const counterContract = (await factory.deploy()) as Counter;
  const counterContractAddress = await counterContract.getAddress();

  return { counterContract, counterContractAddress };
}
```

To run this setup before each test case, call `deployFixture()` inside a `beforeEach` block:

```ts
beforeEach(async () => {
  ({ counterContract, counterContractAddress } = await deployFixture());
});
```

This ensures each test runs with a clean, independent contract instance.

---

Let's put it together,

### ✍️ Replace the contents of `test/Counter.ts` with the following:

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";
import { expect } from "chai";
import { Counter, Counter__factory } from "../types";

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

async function deployFixture() {
  const factory = (await ethers.getContractFactory("Counter")) as Counter__factory;
  const counterContract = (await factory.deploy()) as Counter;
  const counterContractAddress = await counterContract.getAddress();

  return { counterContract, counterContractAddress };
}

describe("Counter", function () {
  let signers: Signers;
  let counterContract: Counter;
  let counterContractAddress: Counter;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { deployer: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  beforeEach(async () => {
    // Deploy a new instance of the contract before each test
    ({ counterContract, counterContractAddress } = await deployFixture());
  });

  it("should be deployed", async function () {
    console.log(`Counter has been deployed at address ${counterContractAddress}`);
    // Test the deployed address is valid
    expect(ethers.isAddress(counterContractAddress)).to.eq(true);
  });
});
```
---

### ▶️ Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

### ✅ Expected Output

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed


  1 passing (7ms)
```

# Step 7: Call the contract `getCount()` view function

Everything is up and running, we can now call the `Counter.sol` view function `getCount()` !

✍️ Just below the test block `it("should be deployed", async function () {...}`,

add the following unit test:

```ts
it("count should be zero after deployment", async function () {
    const count = await counterContract.getCount();
    console.log(`Counter.getCount() === ${count}`);
    // Expect initial count to be 0 after deployment
    expect(count).to.eq(0);
});
```

---

### ▶️ Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

### ✅ Expected Output

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
Counter.getCount() === 0
    ✔ count should be zero after deployment


  1 passing (7ms)
```

# Step 8: Call the contract `increment()` transaction function

✍️ Just below the test block `it("count should be zero after deployment", async function () {...}`,

add the following test block:

```ts
it("increment the counter by 1", async function () {
    const countBeforeInc = await counterContract.getCount();
    const tx = await counterContract.connect(signers.alice).increment(1);
    await tx.wait();
    const countAfterInc = await counterContract.getCount();
    expect(countAfterInc).to.eq(countBeforeInc + 1n);
});
```

### 📝 Remarks:
- `increment()` is a transactional function that modifies the blockchain state.
- It must be signed by a user — here we use `alice`.
- `await wait()` to wait for the transaction to mined.
- The test compares the counter before and after the transaction to ensure it incremented as expected.

---

### ▶️ Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

### ✅ Expected Output

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
Counter.getCount() === 0
    ✔ count should be zero after deployment
    ✔ increment the counter by 1


  2 passing (12ms)
```

# Step 9: Call the contract `decrement()` transaction function

✍️ Just below the test block `it("increment the counter by 1", async function () {...}`,

add the following test block:

```ts
it("decrement the counter by 1", async function () {
    // First increment, count becomes 1
    let tx = await counterContract.connect(signers.alice).increment(1);
    await tx.wait();
    // Then decrement, count goes back to 0
    tx = await counterContract.connect(signers.alice).decrement(1);
    await tx.wait();
    const count = await counterContract.getCount();
    expect(count).to.eq(0);
});
```

---

### ▶️ Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

### ✅ Expected Output

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
Counter.getCount() === 0
    ✔ count should be zero after deployment
    ✔ increment the counter by 1
    ✔ decrement the counter by 1


  2 passing (12ms)
```

# 📁 Final Project Files

By the end of this tutorial, you should have the following files in your project:

- [`contracts/Counter.sol`](./code/contracts/Counter.sol.md) — your Solidity smart contract
- [`test/Counter.ts`](./code/contracts/Counter.ts.md) — your Hardhat test suite written in TypeScript

These files form the foundation of a basic Hardhat-based smart contract project.

# 🚀 What’s Next?

Now that you've written and tested a basic Solidity smart contract, you're ready to take the next step.

In the next tutorial, we’ll transform this standard `Counter.sol` contract into a trivial FHEVM-compatible version — allowing the counter value to be stored and updated using trivial fully homomorphic encryption.

This will introduce you to the FHEVM workflow while building on the foundation you've just completed.
