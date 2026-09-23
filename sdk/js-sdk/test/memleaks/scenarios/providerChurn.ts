import { ethers } from 'ethers';
import { prepareSingleChain } from '../../fheTest/setupCommon.js';
import type { Scenario } from './scenario.js';

// Not a WASM/FHE test: isolates listener/socket/interval leaks in the ethers
// layer itself, independent of the SDK's own encrypt/decrypt code paths — a
// distinct and common leak class that would otherwise be invisible inside the
// FHE-focused scenarios. Also mirrors how test/fheTest/setup-ethers.ts itself
// builds providers (a fresh `ethers.JsonRpcProvider` per config, never
// explicitly `.destroy()`-ed), so this doubles as a check on whether that
// pattern is safe to repeat at scale.

export const providerChurnScenario: Scenario = {
  name: 'providerChurn',
  description: 'Creates an ephemeral ethers provider + signer every iteration, does one read call, and discards them.',
  defaultIterations: 5_000,
  defaultIterationsDuration: '~1 min',
  setup: async ({ config }) => {
    const { rpcUrl } = prepareSingleChain();

    const iterate = async (): Promise<void> => {
      const provider = new ethers.JsonRpcProvider(rpcUrl);
      const signer = config.wallet.connect(provider);
      await provider.getBlockNumber();
      await signer.getAddress();
    };

    return { iterate };
  },
};
