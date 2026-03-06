import { generateCoprocessorEnvWithOverrides } from "../config/env-mapping";
import { writeEnvFile } from "../config/env-writer";
import type { FhevmConfig } from "../config/model";

import type { CoprocessorInstance } from "./topology";

function formatDbUrl(config: FhevmConfig, databaseName: string): string {
  return `postgresql://${config.db.user}:${config.db.password}@${config.db.host}:${config.db.port}/${databaseName}`;
}

export function generateCoprocessorInstanceEnv(
  config: FhevmConfig,
  instance: CoprocessorInstance,
): Record<string, string> {
  return generateCoprocessorEnvWithOverrides(config, {
    databaseUrl: formatDbUrl(config, instance.databaseName),
    txSenderPrivateKey: instance.txSenderPrivateKey,
  });
}

export async function generateAllCoprocessorEnvFiles(
  config: FhevmConfig,
  instances: CoprocessorInstance[],
): Promise<Map<string, string>> {
  const outputs = new Map<string, string>();

  await Promise.all(
    instances.map(async (instance) => {
      const filePath = instance.envFilePath;
      const values = generateCoprocessorInstanceEnv(config, instance);
      await writeEnvFile(filePath, values);
      outputs.set(instance.envFileName, filePath);
    }),
  );

  return outputs;
}
