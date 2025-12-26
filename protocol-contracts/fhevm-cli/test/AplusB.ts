import { FhevmType } from "@fhevm/hardhat-plugin";
import "dotenv/config";
import { ethers } from "ethers";
import hre from "hardhat";

function convertToHexString(uint8Array: Uint8Array): string {
  return "0x" + Buffer.from(uint8Array).toString("hex");
}

describe("Test", function () {
  it("decrypt current encrypted wrapper balance", async function () {
    const signer = new ethers.Wallet(process.env.PRIVATE_KEY!);
    const handle = "0x980769a416dbe44044fac20626c9521085a3ba57acff00000000000000010500";
    const contractAddress = "0xb1A7026C28cB91604FB7B1669f060aB74A30c255";
    const currentEncryptedBalance = await hre.fhevm.userDecryptEuint(
      FhevmType.euint64,
      handle,
      contractAddress,
      signer,
    );
    console.log(currentEncryptedBalance);
  });

  it("encrypt some amount I want to unwrap", async function () {
    const signer = new ethers.Wallet(process.env.PRIVATE_KEY!);
    const contractAddress = "0xb1A7026C28cB91604FB7B1669f060aB74A30c255";
    const inputA = hre.fhevm.createEncryptedInput(contractAddress, signer.address);
    inputA.add64(500000);
    const encryptedAmount = await inputA.encrypt();
    console.log(convertToHexString(encryptedAmount.handles[0]));
    console.log(convertToHexString(encryptedAmount.inputProof));
  });

  it("public decrypt some amount I want to finalizeUnwrap", async function () {
    const handle = "0x6ff3f07b805363e89cf9c0a5aa625ad12509026eb6ff00000000000000010500";
    const pubDecryptRes = await hre.fhevm.publicDecrypt([handle]);
    console.log(pubDecryptRes);
  });
});
