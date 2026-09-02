// Prepares a development chain before anyone can use it. Runs inside `newConnection`, which hardhat
// completes BEFORE `hardhat node` starts listening, so the stack is ready on the first request.
//
// Once per CHAIN, not per process: every in-process `network.create()` is a fresh chain and needs its
// own stack, while an `http` development node keeps whatever it already holds (the deploy is
// idempotent: an ACL that carries code means "present"). Either way the stack is then verified, so a
// half-deployed or foreign stack fails here, by name, not later in a test. Public networks are never
// touched.

import type { Deployed } from '@fhevm/host-contracts-cleartext/ts';
import type { NetworkConnection } from 'hardhat/types/network';

import { deployCleartextStack } from './deploy.js';
import { type FhevmNetworkInfo, isDevelopmentNetwork } from './network.js';
import { verifyCleartextStack } from './verify.js';

export async function prepareDevelopmentChain(
  connection: NetworkConnection<string>,
  network: FhevmNetworkInfo,
): Promise<Deployed | undefined> {
  if (!isDevelopmentNetwork(network)) return undefined;
  const deployed = await deployCleartextStack(connection.provider);
  await verifyCleartextStack(connection.provider, deployed);
  return deployed;
}
