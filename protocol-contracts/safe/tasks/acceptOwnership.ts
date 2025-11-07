import { OperationType } from "@safe-global/types-kit";
import { Wallet, getBytes } from "ethers";
import { task, types } from "hardhat/config";
import { getSafeProxyAddress } from "./utils/addresses";

const SAFE_OWNER_PRIVATE_KEYS_ENV = "SAFE_OWNER_PRIVATE_KEYS";

task(
  "task:acceptOwnership",
  `Accepts ownership of a contract from the Safe Smart Account.`,
)
  .addParam("address", "Address of the contract to accept ownership of.", undefined, types.string)
  .setAction(async function ({ address }, { ethers }) {
    const ownerPrivateKeysEnv = process.env[SAFE_OWNER_PRIVATE_KEYS_ENV];
    if (!ownerPrivateKeysEnv) {
      throw new Error(`"${SAFE_OWNER_PRIVATE_KEYS_ENV}" env variable is not set`);
    }
    const signers = JSON.parse(ownerPrivateKeysEnv).map((ownerPrivateKey: string) =>
      new Wallet(ownerPrivateKey).connect(ethers.provider),
    );

    const acceptOwnershipAbi = [
      "function acceptOwnership() public",
    ];
    const contractToCall = await ethers.getContractAt(acceptOwnershipAbi, address);
    const { safeProxy: safe, safeProxyAddress } = await getSafeProxyAddress(ethers);

    const value = 0;
    const data = contractToCall.interface.encodeFunctionData("acceptOwnership");
    const operation = OperationType.Call;
    const safeTxGas = 0;
    const baseGas = 0;
    const gasPrice = 0;
    const gasToken = ethers.ZeroAddress;
    const refundReceiver = ethers.ZeroAddress;
    const nonce = await safe.nonce();

    const transactionHash = await safe.getTransactionHash(
      address,
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

    const signatures = await getSortedSignatures(signers, transactionHash);

    const execTransactionResponse = await safe.execTransaction(
      address,
      value,
      data,
      operation,
      safeTxGas,
      baseGas,
      gasPrice,
      gasToken,
      refundReceiver,
      signatures,
    );
    await execTransactionResponse.wait();
    console.log(
      `Ownership of the contract ${address} successfully accepted by the Safe Smart Account proxy at address: ${safeProxyAddress}`,
    );
  });

async function getSortedSignatures(signers: Wallet[], transactionHash: string): Promise<string> {
  const bytesDataHash = getBytes(transactionHash);

  let signatureBytes = "0x";

  const signerAddresses = await Promise.all(signers.map((signer) => signer.getAddress()));

  const sortedSigners = signers.sort((a, b) => {
    const addressA = signerAddresses[signers.indexOf(a)];
    const addressB = signerAddresses[signers.indexOf(b)];
    return addressA.localeCompare(addressB, "en", { sensitivity: "base" });
  });

  for (const signer of sortedSigners) {
    const signedMessage = await signer.signMessage(bytesDataHash);
    const flatSig = signedMessage.replace(/1b$/, "1f").replace(/1c$/, "20");
    signatureBytes += flatSig.slice(2);
  }

  return signatureBytes;
}
