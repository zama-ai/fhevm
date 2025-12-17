import { ethers } from "hardhat";
import { expect } from "chai";
import {
  deployConfidentialToken,
  getWrappedEvent,
} from "./utils";
import { getSigners, Signers } from "./signers";
import type {
  DeploymentCoordinator,
  ERC20WithCallback,
  ReentrancyAttacker,
  WrapperUpgradeable,
  TestERC20,
} from "../types";
import { deployWrapperFixture } from "./fixtures";

async function deployERC20WithCallback(name: string, symbol: string, decimals: number) {
  const ERC20WithCallbackFactory = await ethers.getContractFactory("ERC20WithCallback");
  const token = await ERC20WithCallbackFactory.deploy(name, symbol, decimals);
  await token.waitForDeployment();
  return token;
}

async function deployReentrancyAttacker(
  wrapper: WrapperUpgradeable,
  token: ERC20WithCallback
) {
  const AttackerFactory = await ethers.getContractFactory("ReentrancyAttacker");
  const attacker = await AttackerFactory.deploy(await wrapper.getAddress(), await token.getAddress());
  await attacker.waitForDeployment();
  return attacker;
}

async function deployFixture(signers: Signers) {
  const { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory } =
    await deployWrapperFixture(signers);

  // Deploy ERC20 token with callback functionality
  const callbackToken = await deployERC20WithCallback("CallbackToken", "CBT", 6);

  return {
    coordinator,
    coordinatorAddress,
    adminProvider,
    confidentialTokenFactory,
    wrapperFactory,
    callbackToken,
  };
}

describe("Wrapper Reentrancy Protection", function () {
  let signers: Signers;
  let coordinator: DeploymentCoordinator;
  let callbackToken: ERC20WithCallback;
  let attacker: ReentrancyAttacker;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    // Deploy fixture
    ({ coordinator, callbackToken } = await deployFixture(signers));

    // Fund alice with callback tokens
    const mintAmount = ethers.parseUnits("10000", 6);
    await callbackToken.mint(signers.alice.address, mintAmount);

    // Deploy confidential token for callback token
    const { wrapper } = await deployConfidentialToken(coordinator, callbackToken as TestERC20, signers.alice);

    // Deploy attacker contract
    attacker = await deployReentrancyAttacker(wrapper, callbackToken);
    const attackerAddress = await attacker.getAddress();

    // Fund attacker with tokens
    await callbackToken.mint(attackerAddress, mintAmount);
  });

  describe("Reentrancy Attack Prevention", function () {
    it("should prevent reentrancy attack during wrap()", async function () {
      const wrapAmount = ethers.parseUnits("1000", 6);
      const maxAttacks = 3; // Try to reenter multiple times

      // Execute attack - the attacker will try to reenter but should be blocked
      const attackTx = await attacker.connect(signers.alice).attack(wrapAmount, maxAttacks);
      const attackReceipt = await attackTx.wait();

      // Check if reentrancy was attempted and failed
      const reentrancyEvents = attackReceipt?.logs
        .filter((log) => {
          try {
            const parsed = attacker.interface.parseLog({ topics: log.topics, data: log.data });
            return parsed?.name === "ReentrancyAttempted";
          } catch {
            return false;
          }
        })
        .map((log) => attacker.interface.parseLog({ topics: log.topics, data: log.data }));

      // Should have attempted reentrancy
      expect(reentrancyEvents?.length).to.be.greaterThan(0, "Should have attempted reentrancy");

      // All reentrancy attempts should have failed
      reentrancyEvents?.forEach((event) => {
        expect(event?.args.success).to.equal(false, "Reentrancy should be blocked");
      });

      // Verify only ONE wrap succeeded
      const wrappedEvents = getWrappedEvent(attackReceipt);
      expect(wrappedEvents.length).to.equal(1, "Should have exactly 1 successful wrap");
    });
  });
});
