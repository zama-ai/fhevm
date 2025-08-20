This example demonstrates the FHE user decryption mechanism with a single value.

User decryption is a mechanism that allows specific users to decrypt encrypted values while keeping them hidden from others. Unlike public decryption where decrypted values become visible to everyone, user decryption maintains privacy by only allowing authorized users with the proper permissions to view the data. While permissions are granted onchain through smart contracts, the actual **decryption call occurs off-chain in the frontend application**.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="UserDecryptSingleValue.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, euint32 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights common pitfalls developers may encounter.
 */
contract UserDecryptSingleValue is SepoliaConfig {
  euint32 private _trivialEuint32;

  // solhint-disable-next-line no-empty-blocks
  constructor() {}

  function initializeUint32(uint32 value) external {
    // Compute a trivial FHE formula _trivialEuint32 = value + 1
    _trivialEuint32 = FHE.add(FHE.asEuint32(value), FHE.asEuint32(1));

    // Grant FHE permissions to:
    // ✅ The contract caller (`msg.sender`): allows them to decrypt `_trivialEuint32`.
    // ✅ The contract itself (`address(this)`): allows it to operate on `_trivialEuint32` and
    //    also enables the caller to perform user decryption.
    //
    // Note: If you forget to call `FHE.allowThis(_trivialEuint32)`, the user will NOT be able
    //       to user decrypt the value! Both the contract and the caller must have FHE permissions
    //       for user decryption to succeed.
    FHE.allowThis(_trivialEuint32);
    FHE.allow(_trivialEuint32, msg.sender);
  }

  function initializeUint32Wrong(uint32 value) external {
    // Compute a trivial FHE formula _trivialEuint32 = value + 1
    _trivialEuint32 = FHE.add(FHE.asEuint32(value), FHE.asEuint32(1));

    // ❌ Common FHE permission mistake:
    // ================================================================
    // We grant FHE permissions to the contract caller (`msg.sender`),
    // expecting they will be able to user decrypt the encrypted value later.
    //
    // However, this will fail! 💥
    // The contract itself (`address(this)`) also needs FHE permissions to allow user decryption.
    // Without granting the contract access using `FHE.allowThis(...)`,
    // the user decryption attempt by the user will not succeed.
    FHE.allow(_trivialEuint32, msg.sender);
  }

  function encryptedUint32() public view returns (euint32) {
    return _trivialEuint32;
  }
}
```

{% endtab %}

{% tab title="UserDecryptSingleValue.ts" %}

```ts
import { UserDecryptSingleValue, UserDecryptSingleValue__factory } from "../../../types";
import type { Signers } from "../../types";
import { FhevmType, HardhatFhevmRuntimeEnvironment } from "@fhevm/hardhat-plugin";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";
import * as hre from "hardhat";

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory("UserDecryptSingleValue")) as UserDecryptSingleValue__factory;
  const userUserDecryptSingleValue = (await factory.deploy()) as UserDecryptSingleValue;
  const userUserDecryptSingleValue_address = await userUserDecryptSingleValue.getAddress();

  return { userUserDecryptSingleValue, userUserDecryptSingleValue_address };
}

/**
 * This trivial example demonstrates the FHE user decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe("UserDecryptSingleValue", function () {
  let contract: UserDecryptSingleValue;
  let contractAddress: string;
  let signers: Signers;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!hre.fhevm.isMock) {
      throw new Error(`This hardhat test suite cannot run on Sepolia Testnet`);
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1] };
  });

  beforeEach(async function () {
    // Deploy a new contract each time we run a new test
    const deployment = await deployFixture();
    contractAddress = deployment.userUserDecryptSingleValue_address;
    contract = deployment.userUserDecryptSingleValue;
  });

  // ✅ Test should succeed
  it("user decryption should succeed", async function () {
    const tx = await contract.connect(signers.alice).initializeUint32(123456);
    await tx.wait();

    const encryptedUint32 = await contract.encryptedUint32();

    // The FHEVM Hardhat plugin provides a set of convenient helper functions
    // that make it easy to perform FHEVM operations within your Hardhat environment.
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    const clearUint32 = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedUint32,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    expect(clearUint32).to.equal(123456 + 1);
  });

  // ❌ Test should fail
  it("user decryption should fail", async function () {
    const tx = await contract.connect(signers.alice).initializeUint32Wrong(123456);
    await tx.wait();

    const encryptedUint32 = await contract.encryptedUint32();

    await expect(
      hre.fhevm.userDecryptEuint(FhevmType.euint32, encryptedUint32, contractAddress, signers.alice),
    ).to.be.rejectedWith(new RegExp("^dapp contract (.+) is not authorized to user decrypt handle (.+)."));
  });
});
```

{% endtab %}

{% endtabs %}
