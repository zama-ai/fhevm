import { HDNodeWallet, MaxUint256, Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";

import { getRequiredEnvVar } from "./utils";

// Mint mocked $ZAMA tokens to the specified address
// The amount is in mocked $ZAMA tokens
async function mintMockedZamaTokens(
  txSender: Wallet | HDNodeWallet,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
  amount: bigint = BigInt(10 ** 4),
) {
  // Get the ZamaOFT contract
  const zamaOFT = await ethers.getContractAt("ZamaOFT", getRequiredEnvVar("ZAMA_OFT_ADDRESS"));

  // Convert the amount to mocked $ZAMA base units (using 18 decimals)
  const amountInMockedZamaBaseUnits = amount * BigInt(10 ** 18);

  // Mint the tokens to the tx sender
  const tx = await zamaOFT.connect(txSender).mint(txSender.address, amountInMockedZamaBaseUnits);
  const receipt = await tx.wait();

  if (receipt?.status !== 1) {
    throw new Error(`Mint failed for address ${txSender.address} using itself as the signer`);
  }

  if (verbose) {
    console.log(`Funding successful: ${amount} mocked $ZAMA tokens minted for address ${txSender.address}\n`);
  }
}

// Approve the specified contract with maximum allowance over the tx sender's tokens
export async function approveContractWithMaxAllowance(
  txSender: Wallet | HDNodeWallet,
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
// - Funding the tx sender with mocked $ZAMA tokens (by minting them)
// - Approving the ProtocolPayment contract with maximum allowance over the signer's tokens
// The amount is in mocked $ZAMA tokens
export async function setTxSenderMockedPayment(
  txSender: Wallet | HDNodeWallet,
  ethers: HardhatEthersHelpers,
  verbose: boolean = false,
  amount: bigint = BigInt(10 ** 4),
) {
  // Mint mocked $ZAMA tokens to the tx sender
  await mintMockedZamaTokens(txSender, ethers, verbose, amount);

  // Get the addresses of the ProtocolPayment contract to approve
  const protocolPaymentAddress = getRequiredEnvVar("PROTOCOL_PAYMENT_ADDRESS");

  // Approve the ProtocolPayment contract with maximum allowance over the signer's tokens
  await approveContractWithMaxAllowance(txSender, protocolPaymentAddress, ethers, verbose);
}

// Set the account that sends request transactions (input verification, decryption) for mocked payment
// Amount is in mocked $ZAMA tokens (NOT in base units with 18 decimals)
task("task:setTxSenderMockedPayment")
  .addOptionalParam("amount", "The amount of mocked $ZAMA tokens to fund the tx sender with", BigInt(10 ** 12), types.bigint)
  .setAction(async function ({ amount }, hre) {
    // Compile the mocked payment bridging contracts
    await hre.run("compile:specific", { contract: "contracts/mockedPaymentBridging" });

    // Get the tx sender wallet
    const txSenderPrivateKey = getRequiredEnvVar("TX_SENDER_PRIVATE_KEY");
    const txSender = new Wallet(txSenderPrivateKey).connect(hre.ethers.provider);

    await setTxSenderMockedPayment(txSender, hre.ethers, true, amount);
  });
