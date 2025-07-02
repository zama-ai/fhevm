# Test the FHEVM contract

In this tutorial, you’ll learn how to migrate a standard Hardhat test suite - from `Counter.ts` to its FHEVM-compatible version `FHECounter.ts` — and progressively enhance it to support Fully Homomorphic Encryption using Zama’s FHEVM library.

## Set up the FHEVM testing environment

{% stepper %}
{% step %}

## Create a test script `test/FHECounter.ts`

Go to your project's `test` directory

```sh
cd <your-project-root-directory>/test
```

From there, create a new file named `FHECounter.ts` and copy/paste the following Typescript skeleton code in it.

```ts
import { FHECounter, FHECounter__factory } from "../types";
import { FhevmType } from "@fhevm/hardhat-plugin";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers, fhevm } from "hardhat";

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

async function deployFixture() {
  const factory = (await ethers.getContractFactory("FHECounter")) as FHECounter__factory;
  const fheCounterContract = (await factory.deploy()) as FHECounter;
  const fheCounterContractAddress = await fheCounterContract.getAddress();

  return { fheCounterContract, fheCounterContractAddress };
}

describe("FHECounter", function () {
  let signers: Signers;
  let fheCounterContract: FHECounter;
  let fheCounterContractAddress: string;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { deployer: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  beforeEach(async () => {
    ({ fheCounterContract, fheCounterContractAddress } = await deployFixture());
  });

  it("should be deployed", async function () {
    console.log(`FHECounter has been deployed at address ${fheCounterContractAddress}`);
    // Test the deployed address is valid
    expect(ethers.isAddress(fheCounterContractAddress)).to.eq(true);
  });

  //   it("count should be zero after deployment", async function () {
  //     const count = await counterContract.getCount();
  //     console.log(`Counter.getCount() === ${count}`);
  //     // Expect initial count to be 0 after deployment
  //     expect(count).to.eq(0);
  //   });

  //   it("increment the counter by 1", async function () {
  //     const countBeforeInc = await counterContract.getCount();
  //     const tx = await counterContract.connect(signers.alice).increment(1);
  //     await tx.wait();
  //     const countAfterInc = await counterContract.getCount();
  //     expect(countAfterInc).to.eq(countBeforeInc + 1n);
  //   });

  //   it("decrement the counter by 1", async function () {
  //     // First increment, count becomes 1
  //     let tx = await counterContract.connect(signers.alice).increment();
  //     await tx.wait();
  //     // Then decrement, count goes back to 0
  //     tx = await counterContract.connect(signers.alice).decrement(1);
  //     await tx.wait();
  //     const count = await counterContract.getCount();
  //     expect(count).to.eq(0);
  //   });
});
```

### What’s Different from `Counter.ts`?

- This test file is structurally similar to the original `Counter.ts`, but it uses the FHEVM-compatible smart contract `FHECounter` instead of the regular `Counter`.

– For clarity, the `Counter` unit tests are included as comments, allowing you to better understand how each part is adapted during the migration to FHEVM.

- While the test logic remains the same, this version is now set up to support encrypted computations via the FHEVM library — enabling tests that manipulate confidential values directly on-chain.

{% endstep %}

{% step %}

## Run the test `test/FHECounter.ts`

From your project's root directory, run:

```sh
npx hardhat test
```

Output:

```sh
  FHECounter
FHECounter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed


  1 passing (1ms)
```

Great! Your Hardhat FHEVM test environment is properly setup.

{% endstep %}
{% endstepper %}

## Test functions

Now everything is up and running, you can start testing your contract functions.

{% stepper %}
{% step %}

## Call the contract `getCount()` view function

Replace the commented‐out test for the legacy `Counter` contract:

```ts
//   it("count should be zero after deployment", async function () {
//     const count = await counterContract.getCount();
//     console.log(`Counter.getCount() === ${count}`);
//     // Expect initial count to be 0 after deployment
//     expect(count).to.eq(0);
//   });
```

with its FHEVM equivalent:

```ts
it("encrypted count should be uninitialized after deployment", async function () {
  const encryptedCount = await fheCounterContract.getCount();
  // Expect initial count to be bytes32(0) after deployment,
  // (meaning the encrypted count value is uninitialized)
  expect(encryptedCount).to.eq(ethers.ZeroHash);
});
```

#### What’s different?

– `encryptedCount` is no longer a plain TypeScript number. It is now a hexadecimal string representing a Solidity `bytes32` value, known as an **FHEVM handle**. This handle points to an encrypted FHEVM primitive of type `euint32`, which internally represents an encrypted Solidity `uint32` primitive type.

- `encryptedCount` is equal to `0x0000000000000000000000000000000000000000000000000000000000000000` which means that `encryptedCount` is uninitialized, and does not reference to any encrypted value at this point.

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
    ✔ encrypted count should be uninitialized after deployment


  2 passing (7ms)
```

{% endstep %}

{% step %}

## Setup the `increment()` function unit test

We’ll migrate the `increment()` unit test to FHEVM step by step.
To start, let’s handle the value of the counter before the first increment.
As explained above, the counter is initially a `bytes32` value equal to zero, meaning the FHEVM `euint32` variable is uninitialized.

We’ll interpret this as if the underlying clear value is 0.

Replace the commented‐out test for the legacy `Counter` contract:

```ts
//   it("increment the counter by 1", async function () {
//     const countBeforeInc = await counterContract.getCount();
//     const tx = await counterContract.connect(signers.alice).increment(1);
//     await tx.wait();
//     const countAfterInc = await counterContract.getCount();
//     expect(countAfterInc).to.eq(countBeforeInc + 1n);
//   });
```

with the following:

```ts
it("increment the counter by 1", async function () {
  const encryptedCountBeforeInc = await fheCounterContract.getCount();
  expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
  const clearCountBeforeInc = 0;

  // const tx = await counterContract.connect(signers.alice).increment(1);
  // await tx.wait();
  // const countAfterInc = await counterContract.getCount();
  // expect(countAfterInc).to.eq(countBeforeInc + 1n);
});
```

{% endstep %}

{% step %}

## Encrypt the `increment()` function argument

The `increment()` function takes a single argument: the value by which the counter should be incremented. In the initial version of `Counter.sol`, this value is a clear `uint32`.

We’ll switch to passing an encrypted value instead, using FHEVM `externalEuint32` primitive type. This allows us to securely increment the counter without revealing the input value on-chain.

{% hint style="info" %}
We are using an `externalEuint32` instead of a regular `euint32`. This tells the FHEVM that the encrypted `uint32` was provided externally (e.g., by a user) and must be verified for integrity and authenticity before it can be used within the contract.
{% endhint %}

Replace :

```ts
it("increment the counter by 1", async function () {
  const encryptedCountBeforeInc = await fheCounterContract.getCount();
  expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
  const clearCountBeforeInc = 0;

  // const tx = await counterContract.connect(signers.alice).increment(1);
  // await tx.wait();
  // const countAfterInc = await counterContract.getCount();
  // expect(countAfterInc).to.eq(countBeforeInc + 1n);
});
```

with the following:

```ts
it("increment the counter by 1", async function () {
  const encryptedCountBeforeInc = await fheCounterContract.getCount();
  expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
  const clearCountBeforeInc = 0;

  // Encrypt constant 1 as a euint32
  const clearOne = 1;
  const encryptedOne = await fhevm
    .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
    .add32(clearOne)
    .encrypt();

  // const tx = await counterContract.connect(signers.alice).increment(1);
  // await tx.wait();
  // const countAfterInc = await counterContract.getCount();
  // expect(countAfterInc).to.eq(countBeforeInc + 1n);
});
```

{% hint style="info" %}
`fhevm.createEncryptedInput(fheCounterContractAddress, signers.alice.address)` creates an encrypted value that is bound to both the contract (`fheCounterContractAddress`) and the user (`signers.alice.address`).
This means only Alice can use this encrypted value, and only within the `FHECounter.sol` contract at that specific address. **It cannot be reused by another user or in a different contract, ensuring data confidentiality and binding context-specific encryption.**
{% endhint %}

{% endstep %}

{% step %}

## Call the `increment()` function with the encrypted argument

Now that we have an encrypted argument, we can call the `increment()` function with it.

Below, you’ll notice that the updated `increment()` function now takes **two arguments instead of one.**

This is because the FHEVM requires both:

1. The `externalEuint32` — the encrypted value itself
2. An accompanying **Zero-Knowledge Proof of Knowledge** (`inputProof`) — which verifies that the encrypted input is securely bound to:
   - the caller (Alice, the transaction signer), and
   - the target smart contract (where `increment()` is being executed)

This ensures that the encrypted value cannot be reused in a different context or by a different user, preserving **confidentiality and integrity.**

Replace :

```ts
// const tx = await counterContract.connect(signers.alice).increment(1);
// await tx.wait();
```

with the following:

```ts
const tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handles[0], encryptedOne.inputProof);
await tx.wait();
```

At this point the counter has been successfully incremented by 1 using a **Fully Homomorphic Encryption (FHE)**. In the next step, we will retrieve the updated encrypted counter value and decrypt it locally.
But before we move on, let’s quickly run the tests to make sure everything is working correctly.

---

#### Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

#### Expected Output

```sh
  FHECounter
FHECounter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
    ✔ encrypted count should be uninitialized after deployment
    ✔ increment the counter by 1


  3 passing (7ms)
```

{% endstep %}

{% step %}

## Call the `getCount()` function and Decrypt the value

Now that the counter has been incremented using an encrypted input, it's time to **read the updated encrypted value** from the smart contract and **decrypt it** using the `userDecryptEuint` function provided by the FHEVM Hardhat Plugin.

The `userDecryptEuint` function takes four parameters:

1. **FhevmType**: The integer type of the FHE-encrypted value. In this case, we're using `FhevmType.euint32` because the counter is a `uint32`.
2. **Encrypted handle**: A 32-byte FHEVM handle representing the encrypted value you want to decrypt.
3. **Smart contract address**: The address of the contract that has permission to access the encrypted handle.
4. **User signer**: The signer (e.g., signers.alice) who has permission to access the handle.

{% hint style="info" %}
Note: Permissions to access the FHEVM handle are set on-chain using the `FHE.allow()` Solidity function (see FHECounter.sol).
{% endhint %}

Replace :

```ts
// const countAfterInc = await counterContract.getCount();
// expect(countAfterInc).to.eq(countBeforeInc + 1n);
```

with the following:

```ts
const encryptedCountAfterInc = await fheCounterContract.getCount();
const clearCountAfterInc = await fhevm.userDecryptEuint(
  FhevmType.euint32,
  encryptedCountAfterInc,
  fheCounterContractAddress,
  signers.alice,
);
expect(clearCountAfterInc).to.eq(clearCountBeforeInc + clearOne);
```

---

#### Run the test

From your project's root directory, run:

```sh
npx hardhat test
```

#### Expected Output

```sh
  FHECounter
FHECounter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
    ✔ encrypted count should be uninitialized after deployment
    ✔ increment the counter by 1


  3 passing (7ms)
```

{% endstep %}

{% step %}

## Call the contract `decrement()` function

Similarly to the previous test, we’ll now call the `decrement()` function using an encrypted input.

Replace :

```ts
//   it("decrement the counter by 1", async function () {
//     // First increment, count becomes 1
//     let tx = await counterContract.connect(signers.alice).increment();
//     await tx.wait();
//     // Then decrement, count goes back to 0
//     tx = await counterContract.connect(signers.alice).decrement(1);
//     await tx.wait();
//     const count = await counterContract.getCount();
//     expect(count).to.eq(0);
//   });
```

with the following:

```ts
it("decrement the counter by 1", async function () {
  // Encrypt constant 1 as a euint32
  const clearOne = 1;
  const encryptedOne = await fhevm
    .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
    .add32(clearOne)
    .encrypt();

  // First increment by 1, count becomes 1
  let tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handles[0], encryptedOne.inputProof);
  await tx.wait();

  // Then decrement by 1, count goes back to 0
  tx = await fheCounterContract.connect(signers.alice).decrement(encryptedOne.handles[0], encryptedOne.inputProof);
  await tx.wait();

  const encryptedCountAfterDec = await fheCounterContract.getCount();
  const clearCountAfterDec = await fhevm.userDecryptEuint(
    FhevmType.euint32,
    encryptedCountAfterDec,
    fheCounterContractAddress,
    signers.alice,
  );

  expect(clearCountAfterDec).to.eq(0);
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
  FHECounter
FHECounter has been deployed at address 0x7553CB9124f974Ee475E5cE45482F90d5B6076BC
    ✔ should be deployed
    ✔ encrypted count should be uninitialized after deployment
    ✔ increment the counter by 1
    ✔ decrement the counter by 1


  4 passing (7ms)
```

{% endstep %}

{% endstepper %}

## Congratulations! You've completed the full tutorial.

You have successfully written and tested your FHEVM-based counter smart contract.
By now, your project should include the following files:

- [`contracts/FHECounter.sol`](https://docs.zama.ai/protocol/examples#tab-fhecounter.sol) — your Solidity smart contract
- [`test/FHECounter.ts`](https://docs.zama.ai/protocol/examples#tab-fhecounter.ts) — your Hardhat test suite written in TypeScript

## Next step
If you would like to deploy your project on the Testnet, or learn more about using FHEVM Hardhat Plug in, head to [Deploy contracts and run tests](../../hardhat/run_test.md).