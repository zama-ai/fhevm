import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { AddressLike, BigNumberish, Signer, TypedDataDomain, Wallet, ZeroAddress } from "ethers";
import { ethers, network } from "hardhat";

import { CircuitBreakerModule, Decryption, Safe } from "../typechain-types";
import { createEIP712PauseContract, createRandomWallet, loadTestVariablesFixture } from "./utils";

// import { execTransaction } from "./utils/utils";

/**
 * Executes a transaction on the Safe contract.
 * @param wallets - The signers of the transaction.
 * @param safe - The Safe contract instance.
 * @param to - The address to send the transaction to.
 * @param value - The value to send with the transaction.
 * @param data - The data to send with the transaction.
 * @param operation - The operation type (0 for call, 1 for delegate call).
 */
const execTransaction = async function (
  wallets: Signer[],
  safe: Safe,
  to: AddressLike,
  value: BigNumberish,
  data: string,
  operation: number,
): Promise<void> {
  // Get the current nonce of the Safe contract
  const nonce = await safe.nonce();

  // Get the transaction hash for the Safe transaction
  const transactionHash = await safe.getTransactionHash(
    to,
    value,
    data,
    operation,
    0,
    0,
    0,
    ZeroAddress,
    ZeroAddress,
    nonce,
  );

  let signatureBytes = "0x";
  const bytesDataHash = ethers.getBytes(transactionHash);

  // Get the addresses of the signers
  const addresses = await Promise.all(wallets.map((wallet) => wallet.getAddress()));
  // Sort the signers by their addresses
  const sorted = wallets.sort((a, b) => {
    const addressA = addresses[wallets.indexOf(a)];
    const addressB = addresses[wallets.indexOf(b)];
    return addressA.localeCompare(addressB, "en", { sensitivity: "base" });
  });

  // Sign the transaction hash with each signer
  for (let i = 0; i < sorted.length; i++) {
    const signedMessage = await sorted[i].signMessage(bytesDataHash);
    const flatSig = signedMessage.replace(/1b$/, "1f").replace(/1c$/, "20");
    signatureBytes += flatSig.slice(2);
  }

  // Execute the transaction on the Safe contract
  await safe.execTransaction(to, value, data, operation, 0, 0, 0, ZeroAddress, ZeroAddress, signatureBytes);
};

describe.only("CircuitBreakerModule", function () {
  // Define variables
  let owner: Wallet;
  let safe: Safe;
  let circuitBreakerSafeAccountAddress: string;
  let chainId: number;

  let decryption: Decryption;

  // Before hook to setup the contracts
  before(async () => {
    const fixtureData = await loadFixture(loadTestVariablesFixture);

    decryption = fixtureData.decryption;
    owner = fixtureData.owner;
    chainId = network.config.chainId!;
    circuitBreakerSafeAccountAddress = await fixtureData.circuitBreakerSafeAccount.getAddress();
    safe = await ethers.getContractAt("Safe", circuitBreakerSafeAccountAddress);
  });

  // Enable the CircuitBreaker contract as a module in the Safe
  // A Safe Module is a smart contract that is allowed to execute transactions on behalf of a Safe Smart Account.
  // This function deploys the CircuitBreaker contract and enables it in the Safe Smart Account.
  const getEnabledCircuitBreaker = async (): Promise<CircuitBreakerModule> => {
    // Deploy the CircuitBreaker contract and pass the safe address as arguments
    const circuitBreakerFactory = await ethers.getContractFactory("CircuitBreakerModule", owner);
    const circuitBreaker = await circuitBreakerFactory.deploy(circuitBreakerSafeAccountAddress);

    // Enable the module in the safe - Step 1: generate transaction data
    const enableModuleData = safe.interface.encodeFunctionData("enableModule", [circuitBreaker.target]);

    // Enable the module in the safe - Step 2: execute the transaction
    await execTransaction([owner], safe, safe.target, 0, enableModuleData, 0);

    // Verify that the module is enabled
    expect(await safe.isModuleEnabled.staticCall(circuitBreaker.target)).to.be.true;

    return circuitBreaker;
  };

  it("Should successfully pause the Decryption contract", async function () {
    // Get the enabled CircuitBreakerModule in the Safe Smart Account
    const circuitBreaker = await getEnabledCircuitBreaker();
    const ownerAddress = await owner.getAddress();
    const deadline = 100000000000000n;
    const nonce = await circuitBreaker.nonces(ownerAddress);
    const contractAddress = await decryption.getAddress();
    const fakeOwner = createRandomWallet();
    const verifyingContract = await circuitBreaker.getAddress();

    // Get the EIP-712 typed signature expected by the CircuitBreaker module on `pauseContract` call.
    const eip712Message = createEIP712PauseContract(
      chainId,
      verifyingContract,
      contractAddress,
      ownerAddress,
      Number(nonce),
      Number(deadline),
    );

    // Sign the EIP-712 message with the owner's private key.
    const signature = await owner.signTypedData(
      eip712Message.domain,
      { PauseContract: eip712Message.types.PauseContract },
      eip712Message.message,
    );

    // We want to make sure that a fake signer's invalid input cannot call the module even with a valid signature.
    // We test this before the valid transaction, otherwise it would fail because of an invalid nonce.
    await expect(
      circuitBreaker.connect(fakeOwner).pauseContract(contractAddress, deadline, signature),
    ).to.be.revertedWith("GS026");

    // Verify the contract is unpaused
    expect(await decryption.paused()).to.be.equal(false);

    // Now we use the signature to pause a contract via our module
    await circuitBreaker.connect(owner).pauseContract(contractAddress, deadline, signature);

    // Verify the contract pausing status
    expect(await decryption.paused()).to.be.equal(true);
  });
});
