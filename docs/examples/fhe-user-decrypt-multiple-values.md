This example demonstrates the FHE user decryption mechanism with multiple values.

User decryption is a mechanism that allows specific users to decrypt encrypted values while keeping them hidden from others. Unlike public decryption where decrypted values become visible to everyone, user decryption maintains privacy by only allowing authorized users with the proper permissions to view the data. While permissions are granted onchain through smart contracts, the actual **decryption call occurs off-chain in the frontend application**.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="UserDecryptMultipleValues.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, ebool, euint32, euint64 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract UserDecryptMultipleValues is SepoliaConfig {
  ebool private _encryptedBool; // = 0 (uninitizalized)
  euint32 private _encryptedUint32; // = 0 (uninitizalized)
  euint64 private _encryptedUint64; // = 0 (uninitizalized)

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

    // see `DecryptSingleValue.sol` for more detailed explanations
    // about FHE permissions and asynchronous user decryption requests.
    FHE.allowThis(_encryptedBool);
    FHE.allowThis(_encryptedUint32);
    FHE.allowThis(_encryptedUint64);

    FHE.allow(_encryptedBool, msg.sender);
    FHE.allow(_encryptedUint32, msg.sender);
    FHE.allow(_encryptedUint64, msg.sender);
  }

  function encryptedBool() public view returns (ebool) {
    return _encryptedBool;
  }

  function encryptedUint32() public view returns (euint32) {
    return _encryptedUint32;
  }

  function encryptedUint64() public view returns (euint64) {
    return _encryptedUint64;
  }
}
```

{% endtab %}

{% tab title="UserDecryptMultipleValues.ts" %}

```ts
import { UserDecryptMultipleValues, UserDecryptMultipleValues__factory } from "../../../types";
import type { Signers } from "../../types";
import { HardhatFhevmRuntimeEnvironment } from "@fhevm/hardhat-plugin";
import { utils as fhevm_utils } from "@fhevm/mock-utils";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { DecryptedResults } from "@zama-fhe/relayer-sdk";
import { expect } from "chai";
import { ethers } from "hardhat";
import * as hre from "hardhat";

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory("UserDecryptMultipleValues")) as UserDecryptMultipleValues__factory;
  const userDecryptMultipleValues = (await factory.deploy()) as UserDecryptMultipleValues;
  const userDecryptMultipleValues_address = await userDecryptMultipleValues.getAddress();

  return { userDecryptMultipleValues, userDecryptMultipleValues_address };
}

/**
 * This trivial example demonstrates the FHE user decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe("UserDecryptMultipleValues", function () {
  let contract: UserDecryptMultipleValues;
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
    contractAddress = deployment.userDecryptMultipleValues_address;
    contract = deployment.userDecryptMultipleValues;
  });

  // ✅ Test should succeed
  it("user decryption should succeed", async function () {
    const tx = await contract.connect(signers.alice).initialize(true, 123456, 78901234567);
    await tx.wait();

    const encryptedBool = await contract.encryptedBool();
    const encryptedUint32 = await contract.encryptedUint32();
    const encryptedUint64 = await contract.encryptedUint64();

    // The FHEVM Hardhat plugin provides a set of convenient helper functions
    // that make it easy to perform FHEVM operations within your Hardhat environment.
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    const aliceKeypair = fhevm.generateKeypair();

    const startTimestamp = fhevm_utils.timestampNow();
    const durationDays = 365;

    const aliceEip712 = fhevm.createEIP712(aliceKeypair.publicKey, [contractAddress], startTimestamp, durationDays);
    const aliceSignature = await signers.alice.signTypedData(
      aliceEip712.domain,
      { UserDecryptRequestVerification: aliceEip712.types.UserDecryptRequestVerification },
      aliceEip712.message,
    );

    const decrytepResults: DecryptedResults = await fhevm.userDecrypt(
      [
        { handle: encryptedBool, contractAddress: contractAddress },
        { handle: encryptedUint32, contractAddress: contractAddress },
        { handle: encryptedUint64, contractAddress: contractAddress },
      ],
      aliceKeypair.privateKey,
      aliceKeypair.publicKey,
      aliceSignature,
      [contractAddress],
      signers.alice.address,
      startTimestamp,
      durationDays,
    );

    expect(decrytepResults[encryptedBool]).to.equal(true);
    expect(decrytepResults[encryptedUint32]).to.equal(123456 + 1);
    expect(decrytepResults[encryptedUint64]).to.equal(78901234567 + 1);
  });
});
```

{% endtab %}

{% endtabs %}
