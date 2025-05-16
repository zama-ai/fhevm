import { HTTPZInstance, createInstance as createHTTPZInstance } from "@httpz/sdk/node";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import dotenv from "dotenv";
import * as fs from "fs";
import { network } from "hardhat";
import { NetworkConfig } from "hardhat/types";

const parsedEnv = dotenv.parse(fs.readFileSync(".env"));

export const createInstance = async () => {
  const instance = await createHTTPZInstance({
    gatewayChainId: parseInt(parsedEnv.GATEWAY_CHAINID),
    verifyingContractAddress: parsedEnv.VERIFYING_CONTRACT_ADDRESS,
    network: (network.config as NetworkConfig & { url: string }).url,
    relayerUrl: parsedEnv.RELAYER_URL,
    aclContractAddress: parsedEnv.ACL_CONTRACT_ADDRESS,
    kmsContractAddress: parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS,
  });
  return instance;
};

export type Decrypt = (handles: { ctHandle: bigint; contractAddress: string }[]) => Promise<bigint[]>;
export type CreateDecrypt = (
  instance: HTTPZInstance,
  signer: HardhatEthersSigner,
  contractAddresses: string[],
) => Decrypt;

export const createDecrypt: CreateDecrypt = (instance, signer, contractAddresses) => async (handles) => {
  const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = instance.generateKeypair();
  const startTimestamp = Date.now();
  const eip712 = instance.createEIP712(publicKeyAlice, contractAddresses, startTimestamp, 365);
  const signatureAlice = await signer.signTypedData(
    eip712.domain,
    { UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification },
    eip712.message,
  );

  return instance.userDecrypt(
    handles,
    privateKeyAlice,
    publicKeyAlice,
    signatureAlice.replace("0x", ""),
    contractAddresses,
    signer.address,
    startTimestamp,
    365,
  );
};
