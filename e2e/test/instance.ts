import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import dotenv from "dotenv";
import { FhevmInstance, createInstance as createFhevmInstance } from "fhevmjs/node";
import * as fs from "fs";
import { network } from "hardhat";
import { NetworkConfig } from "hardhat/types";

const parsedEnv = dotenv.parse(fs.readFileSync(".env"));

export const createInstance = async () => {
  const instance = await createFhevmInstance({
    networkUrl: (network.config as NetworkConfig & { url: string }).url,
    gatewayUrl: parsedEnv.GATEWAY_URL,
    aclContractAddress: parsedEnv.ACL_CONTRACT_ADDRESS,
    kmsContractAddress: parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS,
    publicKeyId: "55729ddea48547ea837137d122e1c90043e94c41",
  });
  return instance;
};

export type Decrypt = (handle: bigint) => Promise<bigint>;
export type CreateDecrypt = (instance: FhevmInstance, signer: HardhatEthersSigner, contractAddress: string) => Decrypt;

export const createDecrypt: CreateDecrypt = (instance, signer, contractAddress) => async (handle) => {
  const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = instance.generateKeypair();
  const eip712 = instance.createEIP712(publicKeyAlice, contractAddress);
  const signatureAlice = await signer.signTypedData(
    eip712.domain,
    { Reencrypt: eip712.types.Reencrypt },
    eip712.message,
  );

  return instance.reencrypt(
    handle,
    privateKeyAlice,
    publicKeyAlice,
    signatureAlice.replace("0x", ""),
    contractAddress,
    signer.address,
  );
};
