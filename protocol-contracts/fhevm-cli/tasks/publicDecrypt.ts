import "dotenv/config";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

// Example usage:
// npx hardhat task:publicDecrypt --handle 0xb5681d0dae644b3ef76aa161b78e61cca125e9aed6ff00000000000000010500 --network mainnet
task("task:publicDecrypt")
  .addParam("handle", "Ciphertext handle to public decrypt", undefined, types.string)
  .setAction(async function ({ handle }, hre: HardhatRuntimeEnvironment) {
    await hre.fhevm.initializeCLIApi();
    const publicDecryptedHandle = await hre.fhevm.publicDecrypt([handle]);
    console.log(`Public decrypted value for handle ${handle} is: `, publicDecryptedHandle.clearValues[handle]);
    console.log(`Abi-encoded cleartext is: `, publicDecryptedHandle.abiEncodedClearValues);
    console.log(`DecryptionProof is: `, publicDecryptedHandle.decryptionProof);
  });
