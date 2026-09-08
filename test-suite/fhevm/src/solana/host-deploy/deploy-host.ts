import { bootstrapZamaHost } from './bootstrap';
import { deployProgramArtifacts } from './deploy-programs';
import type { GatewayBootstrapInputs } from './gateway';
import { loadKeypairSigner } from './keypair';
import { withDeploymentLock } from './lock';
import { programIdsFor } from './program-profile';
import { createHostDeployContext } from './send';

/** The shared host lifecycle: validate bindings, deploy/upgrade, then complete initialization. */
export const deployHostPrograms = async (
  parameters: Parameters<typeof deployProgramArtifacts>[0] & {
    readonly databaseUrl: string;
    readonly gateway: GatewayBootstrapInputs;
    readonly coprocessorThreshold?: number;
    readonly kmsCorruptionThreshold?: number;
  },
) => {
  if (!parameters.programs.includes('zama_host')) throw new Error('host deployment must select zama_host');
  const programAddress = programIdsFor(parameters.profile ?? 'localnet').zamaHost;
  return withDeploymentLock(parameters.databaseUrl, parameters.rpcUrl, programAddress, async (signal) => {
    const context = createHostDeployContext(parameters.rpcUrl, signal);
    const bootstrap = {
      payer: await loadKeypairSigner(parameters.deployerKeypairPath),
      programAddress,
      gateway: parameters.gateway,
      coprocessorThreshold: parameters.coprocessorThreshold,
      kmsCorruptionThreshold: parameters.kmsCorruptionThreshold,
    };
    await bootstrapZamaHost(context, { ...bootstrap, validateOnly: true });
    const ids = await deployProgramArtifacts({ ...parameters, signal });
    await bootstrapZamaHost(context, bootstrap);
    return ids;
  });
};
