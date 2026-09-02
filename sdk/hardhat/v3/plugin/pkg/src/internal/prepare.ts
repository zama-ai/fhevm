// Prepares a development chain before anyone can use it. Runs inside `newConnection`, which hardhat
// completes BEFORE `hardhat node` starts listening, so the chain is ready on the first request.
//
// Today the preparation is a marker — one mined block — that lets the ordering be observed; the
// cleartext stack deploy replaces it. Gating is by network config type on purpose: the node task
// serves a network named `node`, users rename freely, and only an in-process EDR chain is ours to
// prepare — an `http` connection reuses whatever the remote node already holds.

import type { NetworkConnection } from 'hardhat/types/network';

export function isDevelopmentChain(connection: NetworkConnection<string>): boolean {
  return connection.networkConfig.type === 'edr-simulated';
}

export async function prepareDevelopmentChain(connection: NetworkConnection<string>): Promise<void> {
  if (!isDevelopmentChain(connection)) return;
  await connection.provider.request({ method: 'hardhat_mine', params: ['0x1'] });
}
