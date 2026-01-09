import { FhevmType, FhevmTypeEuint } from "@fhevm/hardhat-plugin";
import "dotenv/config";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

// Example usage:
// npx hardhat task:userDecrypt --handle 0x980769a416dbe44044fac20626c9521085a3ba57acff00000000000000010500 --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --encrypted-type euint64 --network mainnet
task("task:userDecrypt")
  .addParam("handle", "Ciphertext handle to user decrypt", undefined, types.string)
  .addParam("contractAddress", "Contract address for which the handle is allowed", undefined, types.string)
  .addOptionalParam("encryptedType", "Fhevm type to use for user decryption", "euint64", types.string)
  .setAction(async function ({ handle, contractAddress, encryptedType }, hre: HardhatRuntimeEnvironment) {
    await hre.fhevm.initializeCLIApi();
    const signer = new hre.ethers.Wallet(process.env.PRIVATE_KEY!);
    const userDecryptedHandle = await hre.fhevm.userDecryptEuint(
      FhevmType[encryptedType as keyof typeof FhevmType] as FhevmTypeEuint,
      handle,
      contractAddress,
      signer,
    );
    console.log(`User decrypted value for handle ${handle} is: `, userDecryptedHandle);
  });
