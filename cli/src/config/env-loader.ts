import { existsSync } from "fs";

import { composeEnvFilePath } from "./env-writer";
import { ENV_FILE_NAMES } from "./service-map";

export function findExistingEnvFiles(envDir: string): Map<string, string> {
  const envFileByName = new Map<string, string>();

  for (const name of ENV_FILE_NAMES) {
    const path = composeEnvFilePath(envDir, name);
    if (existsSync(path)) {
      envFileByName.set(name, path);
    }
  }

  return envFileByName;
}
