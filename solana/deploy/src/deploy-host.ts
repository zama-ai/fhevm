import { bootstrapZamaHost } from './bootstrap';
import { deployProgramArtifacts } from './deploy-programs';
import type { GatewayBootstrapInputs } from './gateway';
import { loadKeypairSigner } from './keypair';
import { programIdsFor } from './program-profile';
import { createHostDeployContext } from './send';

/** The shared host lifecycle: validate bindings, deploy/upgrade, then complete initialization. */
export const deployHostProgram = async (
  parameters: Omit<Parameters<typeof deployProgramArtifacts>[0], 'programs' | 'programKeypairPaths'> & {
    readonly programKeypairPath?: string;
    readonly gateway: GatewayBootstrapInputs;
    readonly coprocessorThreshold?: number;
    readonly kmsCorruptionThreshold?: number;
  },
) => {
  const programAddress = programIdsFor(parameters.profile ?? 'localnet').zamaHost;
  const context = createHostDeployContext(parameters.rpcUrl, parameters.signal);
  const bootstrap = {
    payer: await loadKeypairSigner(parameters.deployerKeypairPath),
    programAddress,
    gateway: parameters.gateway,
    coprocessorThreshold: parameters.coprocessorThreshold,
    kmsCorruptionThreshold: parameters.kmsCorruptionThreshold,
  };
  await bootstrapZamaHost(context, { ...bootstrap, validateOnly: true });
  const ids = await deployProgramArtifacts({
    ...parameters,
    programs: ['zama_host'],
    programKeypairPaths: { zama_host: parameters.programKeypairPath },
  });
  await bootstrapZamaHost(context, bootstrap);
  return ids;
};
