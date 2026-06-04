import type { Hex } from "viem";

import {
  createClientContext,
  resolveChain,
  type ClientOptions,
} from "../../config";
import { fheTestAbi } from "../../fhe-test/abi";
import type { ProgressReporter } from "../../shared/progress";
import type { NetworkName } from "../../types";

export type FheTestInfoOptions = ClientOptions &
  Readonly<{
    contractAddress?: Hex;
    onProgress?: ProgressReporter;
  }>;

export type FheTestInfoResult = Readonly<{
  network: NetworkName;
  contractAddress: Hex;
  contractName: string;
  confidentialProtocolId: string;
  coprocessorConfig: Readonly<{
    aclAddress: Hex;
    coprocessorAddress: Hex;
    kmsVerifierAddress: Hex;
  }>;
  hostChain: Readonly<{
    id: number;
    name?: string;
  }>;
  fhevmChain: Readonly<{
    id: number;
    relayerUrl?: string;
  }>;
  rpcUrl: string;
}>;

export const getFheTestInfo = async (
  options: FheTestInfoOptions,
): Promise<FheTestInfoResult> => {
  options.onProgress?.("Creating clients");
  const { contractAddress, publicClient, rpcUrl } = createClientContext(options);
  const fhevmChain = resolveChain(options);

  options.onProgress?.("Reading FHETest metadata");
  const [contractName, confidentialProtocolId, coprocessorConfig] =
    await Promise.all([
    publicClient.readContract({
      address: contractAddress,
      abi: fheTestAbi,
      functionName: "CONTRACT_NAME",
    }) as Promise<string>,
    publicClient.readContract({
      address: contractAddress,
      abi: fheTestAbi,
      functionName: "confidentialProtocolId",
    }) as Promise<bigint>,
    publicClient.readContract({
      address: contractAddress,
      abi: fheTestAbi,
      functionName: "getCoprocessorConfig",
    }) as Promise<{
      ACLAddress: Hex;
      CoprocessorAddress: Hex;
      KMSVerifierAddress: Hex;
    }>,
  ]);

  return {
    network: options.network,
    contractAddress,
    contractName,
    confidentialProtocolId: confidentialProtocolId.toString(),
    coprocessorConfig: {
      aclAddress: coprocessorConfig.ACLAddress,
      coprocessorAddress: coprocessorConfig.CoprocessorAddress,
      kmsVerifierAddress: coprocessorConfig.KMSVerifierAddress,
    },
    hostChain: {
      id: publicClient.chain.id,
      name: publicClient.chain.name,
    },
    fhevmChain: {
      id: fhevmChain.id,
      relayerUrl: fhevmChain.fhevm.relayerUrl,
    },
    rpcUrl,
  };
};
