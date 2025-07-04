# Write a simple contract

In this tutorial, you'll write and test a simple regular Solidity smart contract within the FHEVM Hardhat template to get familiar with Hardhat workflow.

In the [next tutorial](turn_it_into_fhevm.md), you'll learn how to convert this contract into an FHEVM contract.

## Prerequiste

- [Set up your Hardhat envrionment](setup.md).
- Make sure that you Hardhat project is clean and ready to start. See the instructions [here](setup.md#rest-set-the-hardhat-envrionment).

## What you'll learn

By the end of this tutorial, you will learn to:

- Write a minimal Solidity contract using Hardhat.
- Test the contract using TypeScript and Hardhat’s testing framework.

## Write a simple contract

{% stepper %} {% step %}

## Create `Counter.sol`

Go to your project's `contracts` directory:

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

  /// @notice Increments the counter by a specific value
  function increment(uint32 value) external {
    _count += value;
  }

  /// @notice Decrements the counter by a specific value
  function decrement(uint32 value) external {
    require(_count >= value, "Counter: cannot decrement below zero");
    _count -= value;
  }
}
```

{% endstep %}

{% step %}

## Compile `Counter.sol`

From your project's root directory, run:

```sh
npx hardhat compile
```

Great! Your Smart Contract is now compiled. {% endstep %} {% endstepper %}

## Set up the testing environment

{% stepper %} {% step %}

## Create a test script `test/Counter.ts`

Go to your project's `test` directory

```sh
cd <your-project-root-directory>/test
```

From there, create a new file named `Counter.ts` and copy/paste the following Typescript skeleton code in it.

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

describe("Counter", function () {
  it("empty test", async function () {
    console.log("Cool! The test basic skeleton is running!");
  });
});
```

The file contains the following:

- all the required `import` statements we will need during the various tests
- The `chai` basic statements to run a first empty test named `empty test` {% endstep %}

{% step %}

## Run the test `test/Counter.ts`

From your project's root directory, run:

```sh
npx hardhat test
```

Output:

```sh
  Counter
Cool! The test basic skeleton is running!
    ✔ empty test


  1 passing (1ms)
```

Great! Your Hardhat test environment is properly setup.

{% endstep %}

{% step %}

## Set up the test signers

Before interacting with smart contracts in Hardhat tests, we need to initialize signers.

{% hint style="info" %}
In the context of Ethereum development, a signer represents an entity (usually a wallet) that can send transactions and sign messages. In Hardhat, `ethers.getSigners()` returns a list of pre-funded test accounts.
{% endhint %}

We’ll define three named signers for convenience:

- `owner` — the deployer of the contract
- `alice` and `bob` — additional simulated users

#### Replace the contents of `test/Counter.ts` with the following:

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

#### Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

**Expected Output**

```sh
  Counter
address of user owner is 0x37AC010c1c566696326813b840319B58Bb5840E4
address of user alice is 0xD9F9298BbcD72843586e7E08DAe577E3a0aC8866
address of user bob is 0x3f0CdAe6ebd93F9F776BCBB7da1D42180cC8fcC1
    ✔ should work


  1 passing (2ms)
```

{% endstep %}

{% step %}

## Set up testing instance

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

Let's put it together. Now your`test/Counter.ts` should look like the following:

```ts
import { Counter, Counter__factory } from "../types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";

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
  let counterContractAddress: string;

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

**Run the test:**

From your project's root directory, run:

```sh
npx hardhat test
```

#### Expected Output:

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed


  1 passing (7ms)
```

{% endstep %} {% endstepper %}

## Test functions

Now everything is up and running, you can start testing your contract functions.

{% stepper %} {% step %}

## Call the contract `getCount()` view function

Everything is up and running, we can now call the `Counter.sol` view function `getCount()` !

Just below the test block `it("should be deployed", async function () {...}`,

add the following unit test:

```ts
it("count should be zero after deployment", async function () {
  const count = await counterContract.getCount();
  console.log(`Counter.getCount() === ${count}`);
  // Expect initial count to be 0 after deployment
  expect(count).to.eq(0);
});
```

#### Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

#### Expected Output

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
Counter.getCount() === 0
    ✔ count should be zero after deployment


  1 passing (7ms)
```

{% endstep %}

{% step %}

## Call the contract `increment()` transaction function

Just below the test block `it("count should be zero after deployment", async function () {...}`, add the following test block:

```ts
it("increment the counter by 1", async function () {
  const countBeforeInc = await counterContract.getCount();
  const tx = await counterContract.connect(signers.alice).increment(1);
  await tx.wait();
  const countAfterInc = await counterContract.getCount();
  expect(countAfterInc).to.eq(countBeforeInc + 1n);
});
```

#### Remarks:

- `increment()` is a transactional function that modifies the blockchain state.
- It must be signed by a user — here we use `alice`.
- `await wait()` to wait for the transaction to mined.
- The test compares the counter before and after the transaction to ensure it incremented as expected.

#### Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

#### Expected Output

```sh
  Counter
Counter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
Counter.getCount() === 0
    ✔ count should be zero after deployment
    ✔ increment the counter by 1


  2 passing (12ms)
```

{% endstep %}

{% step %}

## Call the contract `decrement()` transaction function

Just below the test block `it("increment the counter by 1", async function () {...}`,

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

#### Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

#### Expected Output

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

{% endstep %} {% endstepper %}

Now you have succesefully write and test your counter contract. You should have the following files in your project:

- [`contracts/Counter.sol`](https://docs.zama.ai/protocol/examples/basic/fhe-counter#counter.sol) — your Solidity smart contract
- [`test/Counter.ts`](https://docs.zama.ai/protocol/examples/basic/fhe-counter#counter.ts) — your Hardhat test suite written in TypeScript

These files form the foundation of a basic Hardhat-based smart contract project.

## Next step

Now that you've written and tested a basic Solidity smart contract, you're ready to take the next step.

In the [next tutorial](turn_it_into_fhevm.md), we’ll transform this standard `Counter.sol` contract into `FHECounter.sol`, a trivial FHEVM-compatible version — allowing the counter value to be stored and updated using trivial fully homomorphic encryption.
