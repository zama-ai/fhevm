// Post-deploy check of the cleartext stack, by the package that deployed it: `verify()` in deploy
// mode reads code, versions, wiring, ownership and pausers back from the chain. Run after every
// preparation — including the reuse of a stack an `http` node already held, where it is what tells a
// complete stack from a half-deployed or foreign one. No `history` adapter (in-process there is no
// URL), so the two event-scan checks report as skipped; the signer and threshold expectations are the
// package's own defaults and are not stated here, so those checks skip too.

import { type Deployed, verify } from '@fhevm/host-contracts-cleartext/ts';
import { HardhatPluginError } from 'hardhat/plugins';
import type { EthereumProvider } from 'hardhat/types/providers';
import { mnemonicToAccount } from 'viem/accounts';

import { developmentChain, developmentPublicClient, developmentWalletClient } from './clients.js';
import { LOCALHOST_DEPLOYER, PLUGIN_ID } from './constants.js';
import { createViemEthereumAdaptersFromClients } from './vendored/viemEthereumLib.js';

export async function verifyCleartextStack(provider: EthereumProvider, deployed: Deployed): Promise<void> {
  const chain = await developmentChain(provider);
  const publicClient = developmentPublicClient(provider, chain);
  // The adapters factory wants a wallet client although only reads happen here; the deployer's costs nothing.
  const account = mnemonicToAccount(LOCALHOST_DEPLOYER.mnemonic, { path: LOCALHOST_DEPLOYER.path });
  const { provider: ethProvider } = createViemEthereumAdaptersFromClients({
    publicClient,
    walletClient: developmentWalletClient(provider, chain, account),
  });

  // A missing contract makes `verify()` THROW from its later reads rather than only recording the
  // code-check failure, so a throw is a failed verification too.
  let failures: string;
  try {
    const report = await verify({
      mode: 'deploy',
      ethProvider,
      deployed,
      // Deployer and admin are one account (see deploy.ts), so this is who must own the ACLOwner.
      expected: { admin: LOCALHOST_DEPLOYER.address },
    });
    if (report.ok) return;
    failures = report.failures.map((check) => `  - ${check.name}: ${check.detail ?? 'failed'}`).join('\n');
  } catch (error) {
    failures = `  - ${error instanceof Error ? error.message : String(error)}`;
  }
  throw new HardhatPluginError(
    PLUGIN_ID,
    `The cleartext stack at ACL ${deployed.fhevmAddresses.aclAddress} did not verify:\n${failures}`,
  );
}
