import { AddressLike, BigNumberish, Signer, ZeroAddress } from "ethers";
import { ethers } from "hardhat";

import { Safe } from "../../typechain-types";

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
    const flatSig = (await sorted[i].signMessage(bytesDataHash)).replace(/1b$/, "1f").replace(/1c$/, "20");
    signatureBytes += flatSig.slice(2);
  }

  // Execute the transaction on the Safe contract
  await safe.execTransaction(to, value, data, operation, 0, 0, 0, ZeroAddress, ZeroAddress, signatureBytes);
};

export { execTransaction };
