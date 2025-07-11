import { OperationType } from "@safe-global/types-kit";
import { AddressLike, Signer } from "ethers";
import hre from "hardhat";

import { Safe } from "../../typechain-types";

const DEFAULT_BALANCE = "0x1000000000000000000000000000000000000000";

// Add fund to the given address
export async function fund(address: string, balance: string = DEFAULT_BALANCE) {
  await hre.ethers.provider.send("hardhat_setBalance", [address, balance]);
}

// Create a new random address
export function createRandomAddress() {
  return hre.ethers.Wallet.createRandom().address;
}

// Create a list of random addresses
export function createRandomAddresses(length: number) {
  return Array.from({ length }, () => createRandomAddress());
}

// Create a new random wallet
export function createRandomWallet() {
  return hre.ethers.Wallet.createRandom().connect(hre.ethers.provider);
}

// Create a new random wallet with some funds
export async function createAndFundRandomWallet() {
  const user = createRandomWallet();
  await fund(user.address);
  return user;
}

/**
 * Executes a transaction on the Safe contract.
 * @param signers - The signers of the transaction.
 * @param safe - The Safe contract instance.
 * @param to - The address to send the transaction to.
 * @param value - The value to send with the transaction.
 * @param data - The data to send with the transaction.
 * @param operation - The operation type (0 for call, 1 for delegate call).
 */
export async function execSafeTransaction(signers: Signer[], safe: Safe, to: AddressLike, data: string): Promise<void> {
  const operation = OperationType.Call; // Operation type.
  const value = 0; // Ether value.
  const safeTxGas = 0; // Gas that should be used for the safe transaction.
  const baseGas = 0; // Gas costs for that are independent of the transaction execution(e.g. base transaction fee, signature check, payment of the refund)
  const gasPrice = 0; // Maximum gas price that should be used for this transaction.
  const gasToken = hre.ethers.ZeroAddress; // Token address (or 0 if ETH) that is used for the payment.
  const refundReceiver = hre.ethers.ZeroAddress; // Address of receiver of gas payment (or 0 if tx.origin).

  // Get the current nonce of the Safe contract.
  const nonce = await safe.nonce();

  // Get the transaction hash for the Safe transaction.
  const transactionHash = await safe.getTransactionHash(
    to,
    value,
    data,
    operation,
    safeTxGas,
    baseGas,
    gasPrice,
    gasToken,
    refundReceiver,
    nonce,
  );

  let signatureBytes = "0x";
  const bytesDataHash = hre.ethers.getBytes(transactionHash);

  // Get the addresses of the signers.
  const addresses = await Promise.all(signers.map((signer) => signer.getAddress()));

  // Sort the signers by their addresses. The `Safe.execTransaction` expects that the signatures
  // are sorted by owner address. This is required to easily validate no confirmation duplicates exist.
  const sorted = signers.sort((a, b) => {
    const addressA = addresses[signers.indexOf(a)];
    const addressB = addresses[signers.indexOf(b)];
    return addressA.localeCompare(addressB, "en", { sensitivity: "base" });
  });

  // Sign the transaction hash with each signer.
  for (let i = 0; i < sorted.length; i++) {
    const signedMessage = await sorted[i].signMessage(bytesDataHash);
    const flatSig = signedMessage.replace(/1b$/, "1f").replace(/1c$/, "20");
    signatureBytes += flatSig.slice(2);
  }

  // Execute the transaction on the Safe contract.
  await safe.execTransaction(
    to,
    value,
    data,
    operation,
    safeTxGas,
    baseGas,
    gasPrice,
    gasToken,
    refundReceiver,
    signatureBytes,
  );
}
