This example demonstrates the FHE decryption mechanism in Solidity with multiple values.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="DecryptMultipleValuesInSolidity.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, ebool, euint32, euint64 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract DecryptMultipleValuesInSolidity is SepoliaConfig {
  ebool private _encryptedBool; // = 0 (uninitialized)
  euint32 private _encryptedUint32; // = 0 (uninitialized)
  euint64 private _encryptedUint64; // = 0 (uninitialized)

  bool private _clearBool; // = 0 (uninitialized)
  uint32 private _clearUint32; // = 0 (uninitialized)
  uint64 private _clearUint64; // = 0 (uninitialized)

  // solhint-disable-next-line no-empty-blocks
  constructor() {}

  function initialize(bool a, uint32 b, uint64 c) external {
    // Compute 3 trivial FHE formulas

    // _encryptedBool = a ^ false
    _encryptedBool = FHE.xor(FHE.asEbool(a), FHE.asEbool(false));

    // _encryptedUint32 = b + 1
    _encryptedUint32 = FHE.add(FHE.asEuint32(b), FHE.asEuint32(1));

    // _encryptedUint64 = c + 1
    _encryptedUint64 = FHE.add(FHE.asEuint64(c), FHE.asEuint64(1));

    // see `DecryptSingleValueInSolidity.sol` for more detailed explanations
    // about FHE permissions and asynchronous decryption requests.
    FHE.allowThis(_encryptedBool);
    FHE.allowThis(_encryptedUint32);
    FHE.allowThis(_encryptedUint64);
  }

  function requestDecryptMultipleValues() external {
    // To decrypt multiple values, we must construct an array of the encrypted values
    // we want to decrypt.
    //
    // ⚠️ Warning: The order of values in the array is critical!
    // The FHEVM backend will pass the decrypted values to the callback function
    // in the exact same order they appear in this array.
    // Therefore, the order must match the parameter declaration in the callback.
    bytes32[] memory cypherTexts = new bytes32[](3);
    cypherTexts[0] = FHE.toBytes32(_encryptedBool);
    cypherTexts[1] = FHE.toBytes32(_encryptedUint32);
    cypherTexts[2] = FHE.toBytes32(_encryptedUint64);

    FHE.requestDecryption(
      // the list of encrypte values we want to decrypt
      cypherTexts,
      // Selector of the Solidity callback function that the FHEVM backend will call with
      // the decrypted (clear) values as arguments
      this.callbackDecryptMultipleValues.selector
    );
  }

  // ⚠️ WARNING: The `cleartexts` argument is an ABI encoding of the decrypted values associated 
  // to the handles (using `abi.encode`). 
  // 
  // These values' types must match exactly! Mismatched types—such as using `uint32 decryptedUint64` 
  // instead of the correct `uint64 decryptedUint64` can cause subtle and hard-to-detect bugs, 
  // especially for developers new to the FHEVM stack.
  // Always ensure that the parameter types align with the expected decrypted value types.
  // 
  // !DOUBLE-CHECK!
  function callbackDecryptMultipleValues(
    uint256 requestID,
    bytes memory cleartexts,
    bytes memory decryptionProof
  ) external {
    // ⚠️ Don't forget the signature checks! (see `DecryptSingleValueInSolidity.sol` for detailed explanations)
    // The signatures are included in the `decryptionProof` parameter.
    FHE.checkSignatures(requestID, cleartexts, decryptionProof);

    (bool decryptedBool, uint32 decryptedUint32, uint64 decryptedUint64) = abi.decode(cleartexts, (bool, uint32, uint64));
    _clearBool = decryptedBool;
    _clearUint32 = decryptedUint32;
    _clearUint64 = decryptedUint64;
  }

  function clearBool() public view returns (bool) {
    return _clearBool;
  }

  function clearUint32() public view returns (uint32) {
    return _clearUint32;
  }

  function clearUint64() public view returns (uint64) {
    return _clearUint64;
  }
}
```

{% endtab %}

{% tab title="DecryptMultipleValuesInSolidity.ts" %}

```ts
import { DecryptMultipleValuesInSolidity, DecryptMultipleValuesInSolidity__factory } from "../../../types";
import type { Signers } from "../../types";
import { HardhatFhevmRuntimeEnvironment } from "@fhevm/hardhat-plugin";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";
import * as hre from "hardhat";

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory(
    "DecryptMultipleValuesInSolidity",
  )) as DecryptMultipleValuesInSolidity__factory;
  const decryptMultipleValuesInSolidity = (await factory.deploy()) as DecryptMultipleValuesInSolidity;
  const decryptMultipleValuesInSolidity_address = await decryptMultipleValuesInSolidity.getAddress();

  return { decryptMultipleValuesInSolidity, decryptMultipleValuesInSolidity_address };
}

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe("DecryptMultipleValuesInSolidity", function () {
  let contract: DecryptMultipleValuesInSolidity;
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
    contract = deployment.decryptMultipleValuesInSolidity;
  });

  // ✅ Test should succeed
  it("decryption should succeed", async function () {
    // For simplicity, we create 3 trivially encrypted values on-chain.
    let tx = await contract.connect(signers.alice).initialize(true, 123456, 78901234567);
    await tx.wait();

    tx = await contract.requestDecryptMultipleValues();
    await tx.wait();

    // We use the FHEVM Hardhat plugin to simulate the asynchronous on-chain
    // decryption
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    // Use the built-in `awaitDecryptionOracle` helper to wait for the FHEVM decryption oracle
    // to complete all pending Solidity decryption requests.
    await fhevm.awaitDecryptionOracle();

    // At this point, the Solidity callback should have been invoked by the FHEVM backend.
    // We can now retrieve the 3 decrypted (clear) values.
    const clearBool = await contract.clearBool();
    const clearUint32 = await contract.clearUint32();
    const clearUint64 = await contract.clearUint64();

    expect(clearBool).to.equal(true);
    expect(clearUint32).to.equal(123456 + 1);
    expect(clearUint64).to.equal(78901234567 + 1);
  });
});
```

{% endtab %}

{% endtabs %}
