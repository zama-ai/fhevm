import type { DotFhevmPaths } from "../config/dotfhevm";
import type { FhevmConfig } from "../config/model";

const BASE_COPROCESSOR_COMPOSE_FILE = "test-suite/fhevm/docker-compose/coprocessor-docker-compose.yml";
const BASE_GW_LISTENER_PORT = 8080;

export interface CoprocessorInstance {
  index: number;
  displayIndex: number;
  servicePrefix: string;
  envFileName: string;
  envFilePath: string;
  composeFile: string;
  databaseName: string;
  txSenderPrivateKey: string;
  signerAddress: string;
  gwListenerPort: number;
  usesBaseCompose: boolean;
}

export function gwListenerPort(instanceIndex: number): number {
  return BASE_GW_LISTENER_PORT + instanceIndex * 100;
}

export function isMultiCoprocessor(config: FhevmConfig): boolean {
  return config.topology.numCoprocessors > 1;
}

export function buildTopology(config: FhevmConfig, paths: DotFhevmPaths): CoprocessorInstance[] {
  return Array.from({ length: config.topology.numCoprocessors }, (_, index) => {
    const displayIndex = index + 1;
    const isBase = index === 0;
    const suffix = isBase ? "" : `-${displayIndex}`;
    const envFileName = isBase ? "coprocessor" : `coprocessor-${displayIndex}`;
    const keySet = config.keys.coprocessors[index];

    return {
      index,
      displayIndex,
      servicePrefix: `coprocessor${suffix}`,
      envFileName,
      envFilePath: `${paths.env}/${envFileName}.env`,
      composeFile: isBase ? BASE_COPROCESSOR_COMPOSE_FILE : `${paths.compose}/coprocessor-${displayIndex}.yml`,
      databaseName: isBase ? config.db.coprocessorDb : `${config.db.coprocessorDb}_${displayIndex}`,
      txSenderPrivateKey: keySet?.txSender.privateKey ?? config.keys.txSender.privateKey,
      signerAddress: keySet?.signer.address ?? "",
      gwListenerPort: gwListenerPort(index),
      usesBaseCompose: isBase,
    };
  });
}
