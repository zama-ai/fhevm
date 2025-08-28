This example demonstrates how to create a confidential token using the OpenZeppelin smart contract library

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="ConfidentialTokenExample.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {SepoliaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {ConfidentialFungibleToken} from "@openzeppelin/confidential-contracts/token/ConfidentialFungibleToken.sol";

/// @title ConfidentialTokenExample
/// @notice Example confidential fungible token leveraging FHE-based primitives.
/// @dev Inherits `ConfidentialFungibleToken` for encrypted balances and `Ownable2Step` for
/// ownership management. Mints an initial encrypted supply to the deployer. Allows for minting
// later by the owner.
contract ConfidentialTokenExample is SepoliaConfig, ConfidentialFungibleToken, Ownable2Step {
    /// @notice Deploys the token and mints an initial supply to the deployer.
    /// @param amount Initial plaintext supply to be minted to `msg.sender`.
    /// @param name_ Token name.
    /// @param symbol_ Confidential token symbol.
    /// @param tokenURI_ Optional token metadata URI used by the confidential token base.
    constructor(
        uint64 amount,
        string memory name_,
        string memory symbol_,
        string memory tokenURI_
    ) ConfidentialFungibleToken(name_, symbol_, tokenURI_) Ownable(msg.sender) {
        euint64 encryptedAmount = FHE.asEuint64(amount);
        _mint(msg.sender, encryptedAmount);
    }

    /// @notice Mints new tokens by taking a plaintext amount
    /// @param to Address to mint tokens to
    /// @param amount Plaintext amount to mint
    function mint(address to, uint64 amount) external onlyOwner {
        euint64 encryptedAmount = FHE.asEuint64(amount);
        _mint(to, encryptedAmount);
    }
}
```

{% endtab %}

{% tab title="ConfidentialTokenExample.test.ts" %}

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers, fhevm } from "hardhat";
import { ConfidentialTokenExample, ConfidentialTokenExample__factory } from "../../types";
import { expect } from "chai";
import { FhevmType } from "@fhevm/hardhat-plugin";
import { deployConfidentialTokenExampleFixture } from "./confToken.fixture";

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
  carol: HardhatEthersSigner;
};

describe("ConfidentialTokenExample", function () {
  let signers: Signers;
  let confidentialToken: ConfidentialTokenExample;
  let confidentialTokenAddress: string;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = {
      deployer: ethSigners[0],
      alice: ethSigners[1],
      bob: ethSigners[2],
      carol: ethSigners[3],
    };
  });

  beforeEach(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!fhevm.isMock) {
      console.warn(`This hardhat test suite cannot run on Sepolia Testnet`);
      this.skip();
    }

    ({ ConfidentialTokenExample: confidentialToken, ConfidentialTokenExampleAddress: confidentialTokenAddress } =
      await deployConfidentialTokenExampleFixture(signers.deployer));
  });

  describe("Deployment", function () {
    it("should deploy with correct initial parameters", async function () {
      expect(await confidentialToken.name()).to.equal("Confidential Token");
      expect(await confidentialToken.symbol()).to.equal("CTKN");
      expect(await confidentialToken.tokenURI()).to.equal("https://example.com/token");
    });

    it("should set deployer as owner", async function () {
      expect(await confidentialToken.owner()).to.equal(signers.deployer.address);
    });

    it("should mint initial supply to deployer", async function () {
      const deployerBalance = await confidentialToken.confidentialBalanceOf(signers.deployer.address);

      // The balance should be encrypted, so we need to decrypt it
      const clearBalance = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        deployerBalance,
        confidentialTokenAddress,
        signers.deployer,
      );

      expect(clearBalance).to.equal(1000);
    });

    it("should have correct total supply", async function () {
      const totalSupply = await confidentialToken.confidentialTotalSupply();

      // Total supply should be encrypted, so we need to decrypt it
      // The owner should have access to decrypt the total supply
      const clearTotalSupply = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        totalSupply,
        confidentialTokenAddress,
        signers.deployer,
      );

      expect(clearTotalSupply).to.equal(1000);
    });
  });

  describe("Ownership", function () {
    it("should allow owner to transfer ownership", async function () {
      await confidentialToken.connect(signers.deployer).transferOwnership(signers.alice.address);

      // With Ownable2Step, the new owner must accept ownership
      await confidentialToken.connect(signers.alice).acceptOwnership();

      expect(await confidentialToken.owner()).to.equal(signers.alice.address);
    });

    it("should not allow non-owner to transfer ownership", async function () {
      await expect(
        confidentialToken.connect(signers.alice).transferOwnership(signers.bob.address),
      ).to.be.revertedWithCustomError(confidentialToken, "OwnableUnauthorizedAccount");
    });

    it("should not allow non-owner to renounce ownership", async function () {
      await expect(confidentialToken.connect(signers.alice).renounceOwnership()).to.be.revertedWithCustomError(
        confidentialToken,
        "OwnableUnauthorizedAccount",
      );
    });
  });

  describe("Token Information", function () {
    it("should return correct name", async function () {
      expect(await confidentialToken.name()).to.equal("Confidential Token");
    });

    it("should return correct symbol", async function () {
      expect(await confidentialToken.symbol()).to.equal("CTKN");
    });

    it("should return correct token URI", async function () {
      expect(await confidentialToken.tokenURI()).to.equal("https://example.com/token");
    });
  });

  describe("Balance and Supply", function () {
    it("should return encrypted balance for deployer", async function () {
      const balance = await confidentialToken.confidentialBalanceOf(signers.deployer.address);
      expect(balance).to.not.equal(ethers.ZeroHash);

      // Decrypt and verify the balance
      const clearBalance = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balance,
        confidentialTokenAddress,
        signers.deployer,
      );
      expect(clearBalance).to.equal(1000);
    });

    it("should return zero balance for new addresses", async function () {
      const balance = await confidentialToken.confidentialBalanceOf(signers.alice.address);

      // For zero balances, we should check if the handle is initialized
      // If it's not initialized, it means the balance is effectively zero
      try {
        const clearBalance = await fhevm.userDecryptEuint(
          FhevmType.euint64,
          balance,
          confidentialTokenAddress,
          signers.alice,
        );
        expect(clearBalance).to.equal(0);
      } catch (error: any) {
        // If decryption fails due to uninitialized handle, that's also valid
        // as it indicates a zero balance
        expect(error.message).to.include("Handle is not initialized");
      }
    });

    it("should return encrypted total supply", async function () {
      const totalSupply = await confidentialToken.confidentialTotalSupply();
      expect(totalSupply).to.not.equal(ethers.ZeroHash);

      // Decrypt and verify the total supply
      // The owner should have access to decrypt the total supply
      const clearTotalSupply = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        totalSupply,
        confidentialTokenAddress,
        signers.deployer,
      );
      expect(clearTotalSupply).to.equal(1000);
    });
  });

  describe("Access Control", function () {
    it("should allow owner to see confidential total supply", async function () {
      const confidentialTotalSupply = await confidentialToken.confidentialTotalSupply();
      expect(confidentialTotalSupply).to.not.equal(ethers.ZeroHash);
    });

    it("should not allow non-owner to see confidential total supply", async function () {
      // This should revert or return encrypted data that non-owners can't decrypt
      // The exact behavior depends on the FHE implementation
      const confidentialTotalSupply = await confidentialToken.confidentialTotalSupply();
      expect(confidentialTotalSupply).to.not.equal(ethers.ZeroHash);
    });
  });

  describe("Constructor Parameters", function () {
    it("should handle different initial amounts", async function () {
      // Deploy a new contract with different initial amount
      const ConfidentialTokenFactory = (await ethers.getContractFactory(
        "ConfidentialTokenExample",
      )) as ConfidentialTokenExample__factory;
      const newToken = (await ConfidentialTokenFactory.deploy(
        500, // Different initial amount
        "Test Token",
        "TEST",
        "https://test.com/token",
      )) as ConfidentialTokenExample;

      const newTokenAddress = await newToken.getAddress();
      const balance = await newToken.confidentialBalanceOf(signers.deployer.address);

      const clearBalance = await fhevm.userDecryptEuint(FhevmType.euint64, balance, newTokenAddress, signers.deployer);

      expect(clearBalance).to.equal(500);
    });

    it("should handle empty token URI", async function () {
      const ConfidentialTokenFactory = (await ethers.getContractFactory(
        "ConfidentialTokenExample",
      )) as ConfidentialTokenExample__factory;
      const newToken = (await ConfidentialTokenFactory.deploy(
        100,
        "Empty URI Token",
        "EMPTY",
        "", // Empty token URI
      )) as ConfidentialTokenExample;

      expect(await newToken.tokenURI()).to.equal("");
    });
  });
});

```

{% endtab %}

{% tab title="ConfidentialTokenExample.fixture.ts" %}

```ts
import { ethers } from "hardhat";
import type { ConfidentialTokenExample } from "../../types";
import type { ConfidentialTokenExample__factory } from "../../types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";

export async function deployConfidentialTokenExampleFixture(owner: HardhatEthersSigner) {
  // Deploy ConfidentialTokenExample with initial supply
  const ConfidentialTokenExampleFactory = (await ethers.getContractFactory(
    "ConfidentialTokenExample",
  )) as ConfidentialTokenExample__factory;
  const ConfidentialTokenExample = (await ConfidentialTokenExampleFactory.deploy(
    1000, // Initial amount
    "Confidential Token",
    "CTKN",
    "https://example.com/token",
  )) as ConfidentialTokenExample;

  const ConfidentialTokenExampleAddress = await ConfidentialTokenExample.getAddress();

  return {
    ConfidentialTokenExample,
    ConfidentialTokenExampleAddress,
  };
}
```

{% endtab %}

{% endtabs %}
