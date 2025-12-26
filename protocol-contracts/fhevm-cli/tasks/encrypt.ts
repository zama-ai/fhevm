import "dotenv/config";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

function convertToHexString(uint8Array: Uint8Array): string {
  return "0x" + Buffer.from(uint8Array).toString("hex");
}

// eg use: `npx hardhat task:encryptInput --input-value 600000 --user-address 0x22162CEAac09F115797A2ca29C96119B8bf63666 --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --encrypted-type euint64 --network mainnet`
// eg use: `npx hardhat task:encryptInput --input-value true --user-address 0x22162CEAac09F115797A2ca29C96119B8bf63666--contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --encrypted-type ebool  --network mainnet`
// eg use: `npx hardhat task:encryptInput --input-value 0xc0ffee254729296a45a3885639AC7E10F9d54979 --user-address 0x22162CEAac09F115797A2ca29C96119B8bf63666 --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --encrypted-type eaddress --network mainnet`
task("task:encryptInput")
  .addParam(
    "inputValue",
    "Input to encrypt (number, 'true'/'false' for ebool, or address string)",
    undefined,
    types.string,
  )
  .addParam("userAddress", "User address to encrypt the input for", undefined, types.string)
  .addParam("contractAddress", "Contract address to encrypt the input for", undefined, types.string)
  .addOptionalParam("encryptedType", "Fhevm type to use for encryption", "euint64", types.string)
  .setAction(async function (
    { inputValue, userAddress, contractAddress, encryptedType },
    hre: HardhatRuntimeEnvironment,
  ) {
    await hre.fhevm.initializeCLIApi();
    const input = hre.fhevm.createEncryptedInput(contractAddress, userAddress);
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
    console.log(`Ciphertext handle for input ${inputValue}: `, convertToHexString(encryptedAmount.handles[0]));
    console.log("InputProof: ", convertToHexString(encryptedAmount.inputProof));
  });
