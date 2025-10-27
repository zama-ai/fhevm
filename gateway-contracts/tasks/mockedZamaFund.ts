import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { MaxUint256, Wallet } from "ethers";
import { task } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";

import { getRequiredEnvVar } from "./utils";

// Funds the address with mocked $ZAMA tokens using the owner's balance
async function fundTxSenderWithMockedZamaToken(
  deployer: Wallet,
  txSenderAddress: string,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
  amount: bigint = BigInt(10 ** 22),
) {
  // Get the ZamaOFT contract
  const zamaOFT = await ethers.getContractAt("ZamaOFT", getRequiredEnvVar("ZAMA_OFT_ADDRESS"), deployer);

  // Transfer the tokens to the signer
  const tx = await zamaOFT.connect(deployer).transfer(txSenderAddress, amount);
  const receipt = await tx.wait();

  if (receipt?.status !== 1) {
    throw new Error(`Transfer failed from deployer address ${deployer.address} to address ${txSenderAddress}`);
  }

  if (verbose) {
    // Convert the amount to mocked $ZAMA base units (using 18 decimals)
    const amountInMockedZamaTokens = amount / BigInt(10 ** 18);
    console.log(
      `Funding successful: ${amountInMockedZamaTokens} mocked $ZAMA tokens transferred from deployer ${deployer.address} to address ${txSenderAddress}\n`,
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
// - Approving the relevant contracts with maximum allowance over the signer's tokens
// The deployer is expected to have deployed the mocked ZamaOFT contract and thus have an initial
// balance of mocked $ZAMA tokens
export async function setTxSenderMockedPayment(
  deployer: Wallet,
  txSender: Wallet | HardhatEthersSigner,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
) {
  // Fund the tx sender with mocked $ZAMA tokens using the deployer's balance
  await fundTxSenderWithMockedZamaToken(deployer, txSender.address, ethers, verbose);

  // Get the addresses of the contracts to approve
  const decryptionAddress = getRequiredEnvVar("DECRYPTION_ADDRESS");
  const inputVerificationAddress = getRequiredEnvVar("INPUT_VERIFICATION_ADDRESS");

  // Approve the contracts with maximum allowance over the signer's tokens
  await approveContractWithMaxAllowance(txSender, decryptionAddress, ethers, verbose);
  await approveContractWithMaxAllowance(txSender, inputVerificationAddress, ethers, verbose);
}

task("task:setTxSenderMockedPayment").setAction(async function (_, hre) {
  // Compile the mocked payment bridging contracts
  await hre.run("compile:specific", { contract: "contracts/mockedPaymentBridging" });

  // Get the deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  // Get the tx sender wallet
  const txSenderPrivateKey = getRequiredEnvVar("TX_SENDER_PRIVATE_KEY");
  const txSender = new Wallet(txSenderPrivateKey).connect(hre.ethers.provider);

  await setTxSenderMockedPayment(deployer, txSender, hre.ethers, true);
});
