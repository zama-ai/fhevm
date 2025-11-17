import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { MaxUint256, Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";

import { getRequiredEnvVar } from "./utils";

// Funds the address with mocked $ZAMA tokens using the owner's balance
// The amount is in mocked $ZAMA tokens
async function fundTxSenderWithMockedZamaToken(
  deployer: Wallet,
  txSenderAddress: string,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
  amount: bigint = BigInt(10 ** 12),
) {
  // Get the ZamaOFT contract
  const zamaOFT = await ethers.getContractAt("ZamaOFT", getRequiredEnvVar("ZAMA_OFT_ADDRESS"), deployer);

  // Convert the amount to mocked $ZAMA base units (using 18 decimals)
  const amountInMockedZamaBaseUnits = amount * BigInt(10 ** 18);

  // Transfer the tokens to the signer
  const tx = await zamaOFT.connect(deployer).transfer(txSenderAddress, amountInMockedZamaBaseUnits);
  const receipt = await tx.wait();

  if (receipt?.status !== 1) {
    throw new Error(`Transfer failed from deployer address ${deployer.address} to address ${txSenderAddress}`);
  }

  if (verbose) {
    console.log(
      `Funding successful: ${amount} mocked $ZAMA tokens transferred from deployer ${deployer.address} to address ${txSenderAddress}\n`,
    );
  }
}

// Approve the specified contract with maximum allowance over the tx sender's tokens
export async function approveContractWithMaxAllowance(
  txSender: Wallet | HardhatEthersSigner,
  contractAddressToApprove: string,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
) {
  // Get the ZamaOFT contract
  const zamaOFT = await ethers.getContractAt("ZamaOFT", getRequiredEnvVar("ZAMA_OFT_ADDRESS"));

  // Approve the spender with the maximum uint256 value
  const tx = await zamaOFT.connect(txSender).approve(contractAddressToApprove, MaxUint256);
  const receipt = await tx.wait();

  if (receipt?.status !== 1) {
    throw new Error(`Max allowance approval failed for contract ${contractAddressToApprove}`);
  }

  if (verbose) {
    console.log(
      `Max allowance approval successful: contract ${contractAddressToApprove} approved by address ${txSender.address}\n`,
    );
  }
}

// Set the tx sender for mocked payment by:
// - Funding the tx sender with mocked $ZAMA tokens using the deployer's balance
// - Approving the ProtocolPayment contract with maximum allowance over the signer's tokens
// The deployer is expected to have deployed the mocked ZamaOFT contract and thus have an initial
// balance of mocked $ZAMA tokens
// The amount is in mocked $ZAMA tokens
export async function setTxSenderMockedPayment(
  deployer: Wallet,
  txSender: Wallet | HardhatEthersSigner,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
  amount: bigint = BigInt(10 ** 12),
) {
  // Fund the tx sender with mocked $ZAMA tokens using the deployer's balance
  await fundTxSenderWithMockedZamaToken(deployer, txSender.address, ethers, verbose, amount);

  // Get the addresses of the ProtocolPayment contract to approve
  const protocolPaymentAddress = getRequiredEnvVar("PROTOCOL_PAYMENT_ADDRESS");

  // Approve the ProtocolPayment contract with maximum allowance over the signer's tokens
  await approveContractWithMaxAllowance(txSender, protocolPaymentAddress, ethers, verbose);
}

// Amount is in mocked $ZAMA tokens (NOT in base units with 18 decimals)
task("task:setTxSenderMockedPayment")
  .addParam("amount", "The amount of mocked $ZAMA tokens to fund the tx sender with", BigInt(10 ** 12), types.bigint)
  .setAction(async function ({ amount }, hre) {
    // Compile the mocked payment bridging contracts
    await hre.run("compile:specific", { contract: "contracts/mockedPaymentBridging" });

    // Get the deployer wallet
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    // Get the tx sender wallet
    const txSenderPrivateKey = getRequiredEnvVar("TX_SENDER_PRIVATE_KEY");
    const txSender = new Wallet(txSenderPrivateKey).connect(hre.ethers.provider);

    await setTxSenderMockedPayment(deployer, txSender, hre.ethers, true, amount);
  });
