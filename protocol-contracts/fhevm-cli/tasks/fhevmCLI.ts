import { FhevmType, FhevmTypeEuint } from "@fhevm/hardhat-plugin";
import "dotenv/config";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

function convertToHexString(uint8Array: Uint8Array): string {
  return "0x" + Buffer.from(uint8Array).toString("hex");
}

// eg use: `npx hardhat task:userDecrypt --handle 0x980769a416dbe44044fac20626c9521085a3ba57acff00000000000000010500 --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --network mainnet`
task("task:userDecrypt")
  .addParam("handle", "Ciphertext handle to decrypt", undefined, types.string)
  .addParam("contractAddress", "Contract address associated with the handle", undefined, types.string)
  .addOptionalParam("encryptedType", "Fhevm type to use for decryption", "euint64", types.string)
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

// eg use: `npx hardhat task:encryptInput --input-value 500000 --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --network mainnet`
// eg use: `npx hardhat task:encryptInput --input-value true --encrypted-type ebool --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --network mainnet`
// eg use: `npx hardhat task:encryptInput --input-value 0xc0ffee254729296a45a3885639AC7E10F9d54979 --encrypted-type eaddress --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --network mainnet`
task("task:encryptInput")
  .addParam(
    "inputValue",
    "Input to encrypt (number, 'true'/'false' for ebool, or address string)",
    undefined,
    types.string,
  )
  .addParam("contractAddress", "Contract address to encrypt the input for", undefined, types.string)
  .addOptionalParam("encryptedType", "Fhevm type to use for encryption", "euint64", types.string)
  .setAction(async function ({ inputValue, contractAddress, encryptedType }, hre: HardhatRuntimeEnvironment) {
    await hre.fhevm.initializeCLIApi();
    const signer = new hre.ethers.Wallet(process.env.PRIVATE_KEY!);
    const input = hre.fhevm.createEncryptedInput(contractAddress, signer.address);
    switch (encryptedType) {
      case "ebool":
        if (inputValue !== "true" && inputValue !== "false") {
          throw Error(`For ebool, inputValue must be 'true' or 'false', got: ${inputValue}`);
        }
        input.addBool(inputValue === "true");
        break;
      case "euint8":
        input.add8(BigInt(inputValue));
        break;
      case "euint16":
        input.add16(BigInt(inputValue));
        break;
      case "euint32":
        input.add32(BigInt(inputValue));
        break;
      case "euint64":
        input.add64(BigInt(inputValue));
        break;
      case "euint128":
        input.add128(BigInt(inputValue));
        break;
      case "euint256":
        input.add256(BigInt(inputValue));
        break;
      case "eaddress":
        input.addAddress(inputValue);
        break;
      default:
        throw Error(
          `Unrecognized encrypted type : ${encryptedType}, use a valid type: ebool, euint8, euint16, euint32, euint64, euint128, euint256 or eaddress`,
        );
    }
    const encryptedAmount = await input.encrypt();
    console.log("Ciphertext handle: ", convertToHexString(encryptedAmount.handles[0]));
    console.log("InputProof: ", convertToHexString(encryptedAmount.inputProof));
  });
