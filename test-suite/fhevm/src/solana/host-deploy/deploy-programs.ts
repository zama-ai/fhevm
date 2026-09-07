// `solana program deploy --use-rpc` of the .so files baked into the deployer image. Does not
// compile: bytecode (and therefore `declare_id!`) is the image tag. The program keypair secret
// must match that `declare_id!` on first deploy; later runs upgrade in place with the deployer
// as upgrade authority.

import { spawn } from "node:child_process";
import { access } from "node:fs/promises";
import path from "node:path";

import { SOLANA_DEPLOY_PROGRAMS, type SolanaDeployProgram } from "./constants";
import { programIdsFor, type SolanaProgramProfile } from "./program-profile";

const declaredProgramId = (
  profile: SolanaProgramProfile,
): Record<SolanaDeployProgram, string> => {
  const ids = programIdsFor(profile);
  return { zama_host: ids.zamaHost, confidential_token: ids.confidentialToken };
};

export type DeployedProgramIds = {
  readonly zamaHostId: string;
  readonly confidentialTokenId: string;
};

const run = (argv: string[]): Promise<string> =>
  new Promise((resolve, reject) => {
    const child = spawn(argv[0]!, argv.slice(1), { stdio: ["ignore", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (chunk: Buffer) => {
      stdout += chunk.toString();
    });
    child.stderr.on("data", (chunk: Buffer) => {
      stderr += chunk.toString();
    });
    child.on("error", reject);
    child.on("close", (code) => {
      if (code === 0) {
        resolve(stdout.trim());
        return;
      }
      reject(new Error(`${argv.join(" ")} exited ${code}:\n${stderr || stdout}`));
    });
  });

const addressOf = (keypairPath: string): Promise<string> => run(["solana", "address", "-k", keypairPath]);

export const deployProgramArtifacts = async (parameters: {
  readonly rpcUrl: string;
  readonly deployerKeypairPath: string;
  readonly artifactsDir: string;
  readonly programKeypairPaths: Readonly<Record<SolanaDeployProgram, string>>;
  readonly profile?: SolanaProgramProfile;
}): Promise<DeployedProgramIds> => {
  const declared = declaredProgramId(parameters.profile ?? "localnet");
  const ids: Partial<Record<SolanaDeployProgram, string>> = {};
  for (const program of SOLANA_DEPLOY_PROGRAMS) {
    const soPath = path.join(parameters.artifactsDir, `${program}.so`);
    const keypairPath = parameters.programKeypairPaths[program];
    await access(soPath);
    await access(keypairPath);
    const programId = await addressOf(keypairPath);
    const expected = declared[program];
    if (programId !== expected) {
      throw new Error(
        `${program} keypair pubkey ${programId} does not match declare_id! ${expected} ` +
          `for profile ${parameters.profile ?? "localnet"}; rotate ids by regenerating the keypair, updating declare_id!, and rebuilding`,
      );
    }
    await run([
      "solana",
      "program",
      "deploy",
      "-u",
      parameters.rpcUrl,
      "-k",
      parameters.deployerKeypairPath,
      "--upgrade-authority",
      parameters.deployerKeypairPath,
      "--use-rpc",
      "--program-id",
      keypairPath,
      soPath,
    ]);
    ids[program] = programId;
    console.log(`    ${program}=${programId} deployed`);
  }
  return {
    zamaHostId: ids.zama_host!,
    confidentialTokenId: ids.confidential_token!,
  };
};
