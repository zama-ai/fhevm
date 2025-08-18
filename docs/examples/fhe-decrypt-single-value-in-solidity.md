This example demonstrates the FHE decryption mechanism in Solidity.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="DecryptSingleValueInSolidity.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, euint32 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract DecryptSingleValueInSolidity is SepoliaConfig {
  euint32 private _encryptedUint32; // = 0 (uninitizalized)
  uint32 private _clearUint32; // = 0 (uninitizalized)

  // solhint-disable-next-line no-empty-blocks
  constructor() {}

  function initializeUint32(uint32 value) external {
    // Compute a trivial FHE formula _trivialEuint32 = value + 1
    _encryptedUint32 = FHE.add(FHE.asEuint32(value), FHE.asEuint32(1));

    // Grant FHE permissions to:
    // ✅ The contract itself (`address(this)`): allows it to request async decryption to the FHEVM backend
    //
    // Note: If you forget to call `FHE.allowThis(_trivialEuint32)`,
    //       any async decryption request of `_trivialEuint32`
    //       by the contract itself (`address(this)`) will fail!
    FHE.allowThis(_encryptedUint32);
  }

  function initializeUint32Wrong(uint32 value) external {
    // Compute a trivial FHE formula _trivialEuint32 = value + 1
    _encryptedUint32 = FHE.add(FHE.asEuint32(value), FHE.asEuint32(1));
  }

  function requestDecryptSingleUint32() external {
    bytes32[] memory cypherTexts = new bytes32[](1);
    cypherTexts[0] = FHE.toBytes32(_encryptedUint32);

    // Two possible outcomes:
    // ✅ If `initializeUint32` was called, the decryption request will succeed.
    // ❌ If `initializeUint32Wrong` was called, the decryption request will fail 💥
    //
    // Explanation:
    // The request succeeds only if the contract itself (`address(this)`) was granted
    // the necessary FHE permissions. Missing `FHE.allowThis(...)` will cause failure.
    FHE.requestDecryption(
      // the list of encrypte values we want to decrypt
      cypherTexts,
      // the function selector the FHEVM backend will callback with the clear values as arguments
      this.callbackDecryptSingleUint32.selector
    );
  }

  function callbackDecryptSingleUint32(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) external {
    // The `cleartexts` argument is an ABI encoding of the decrypted values associated to the
    // handles (using `abi.encode`). 
    // 
    // ===============================
    //    ☠️🔒 SECURITY WARNING! 🔒☠️
    // ===============================
    //
    // Must call `FHE.checkSignatures(...)` here!
    //            ------------------------
    //
    // This callback must only be called by the authorized FHEVM backend.
    // To enforce this, the contract author MUST verify the authenticity of the caller
    // by using the `FHE.checkSignatures` helper. This ensures that the provided signatures
    // match the expected FHEVM backend and prevents unauthorized or malicious calls.
    //
    // Failing to perform this verification allows anyone to invoke this function with
    // forged values, potentially compromising contract integrity.
    //
    // The responsibility for signature validation lies entirely with the contract author.
    // 
    // The signatures are included in the `decryptionProof` parameter.
    //
    FHE.checkSignatures(requestID, cleartexts, decryptionProof);

    (uint32 decryptedInput) = abi.decode(cleartexts, (uint32));
    _clearUint32 = decryptedInput;
  }

  function clearUint32() public view returns (uint32) {
    return _clearUint32;
  }
}
```

{% endtab %}

{% tab title="DecryptSingleValueInSolidity.ts" %}

```ts
import { DecryptSingleValueInSolidity, DecryptSingleValueInSolidity__factory } from "../../../types";
import type { Signers } from "../../types";
import { HardhatFhevmRuntimeEnvironment } from "@fhevm/hardhat-plugin";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";
import * as hre from "hardhat";

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory(
    "DecryptSingleValueInSolidity",
  )) as DecryptSingleValueInSolidity__factory;
  const decryptSingleValueInSolidity = (await factory.deploy()) as DecryptSingleValueInSolidity;
  const decryptSingleValueInSolidity_address = await decryptSingleValueInSolidity.getAddress();

  return { decryptSingleValueInSolidity, decryptSingleValueInSolidity_address };
}

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe("DecryptSingleValueInSolidity", function () {
  let contract: DecryptSingleValueInSolidity;
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
    contract = deployment.decryptSingleValueInSolidity;
  });

  // ✅ Test should succeed
  it("decryption should succeed", async function () {
    let tx = await contract.connect(signers.alice).initializeUint32(123456);
    await tx.wait();

    tx = await contract.requestDecryptSingleUint32();
    await tx.wait();

    // We use the FHEVM Hardhat plugin to simulate the asynchronous on-chain
    // decryption
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    // Use the built-in `awaitDecryptionOracle` helper to wait for the FHEVM decryption oracle
    // to complete all pending Solidity decryption requests.
    await fhevm.awaitDecryptionOracle();

    // At this point, the Solidity callback should have been invoked by the FHEVM backend.
    // We can now retrieve the decrypted (clear) value.
    const clearUint32 = await contract.clearUint32();

    expect(clearUint32).to.equal(123456 + 1);
  });

  // ❌ Test should fail
  it("decryption should fail", async function () {
    const tx = await contract.connect(signers.alice).initializeUint32Wrong(123456);
    await tx.wait();

    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    const senderNotAllowedError = fhevm.revertedWithCustomErrorArgs("ACL", "SenderNotAllowed");

    await expect(contract.connect(signers.alice).requestDecryptSingleUint32()).to.be.revertedWithCustomError(
      ...senderNotAllowedError,
    );
  });
});
```

{% endtab %}

{% endtabs %}
