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

// Mint mocked $ZAMA tokens to the tx sender
// Amount is in mocked $ZAMA tokens (NOT in base units with 18 decimals)
task("task:txSenderMintMockedZamaTokens")
  .addOptionalParam(
    "amount",
    "The amount of mocked $ZAMA tokens to mint to the tx sender",
    BigInt(10 ** 12),
    types.bigint,
  )
  .setAction(async function ({ amount }, hre) {
    // Get the tx sender wallet
    const txSenderPrivateKey = getRequiredEnvVar("TX_SENDER_PRIVATE_KEY");
    const txSender = new Wallet(txSenderPrivateKey).connect(hre.ethers.provider);

    await mintMockedZamaTokens(txSender, hre.ethers, true, amount);
  });

// Approve the ProtocolPayment contract with maximum allowance over the tx sender's tokens
task("task:txSenderMaxApprovePayment").setAction(async function (_, hre) {
  // Get the tx sender wallet
  const txSenderPrivateKey = getRequiredEnvVar("TX_SENDER_PRIVATE_KEY");
  const txSender = new Wallet(txSenderPrivateKey).connect(hre.ethers.provider);

  // Get the addresses of the ProtocolPayment contract to approve
  const protocolPaymentAddress = getRequiredEnvVar("PROTOCOL_PAYMENT_ADDRESS");

  await approveContractWithMaxAllowance(txSender, protocolPaymentAddress, hre.ethers, true);
});

// Set the account that sends request transactions (input verification, decryption) for mocked payment
// - Funding the tx sender with mocked $ZAMA tokens (by minting them)
// - Approving the ProtocolPayment contract with maximum allowance over the tx sender's tokens
// Amount is in mocked $ZAMA tokens (NOT in base units with 18 decimals)
task("task:setTxSenderMockedPayment")
  .addOptionalParam(
    "amount",
    "The amount of mocked $ZAMA tokens to fund the tx sender with",
    BigInt(10 ** 12),
    types.bigint,
  )
  .setAction(async function ({ amount }, hre) {
    await hre.run("task:txSenderMintMockedZamaTokens", { amount });
    await hre.run("task:txSenderMaxApprovePayment");
  });
