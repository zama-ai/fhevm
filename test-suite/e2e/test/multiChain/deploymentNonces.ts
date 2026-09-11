interface Deployer {
  getAddress(): Promise<string>;
  getNonce(blockTag: 'pending'): Promise<number>;
  sendTransaction(tx: { to: string; value: bigint }): Promise<{
    wait(): Promise<{ status: number | null } | null>;
  }>;
}

/** CREATE addresses must match even when an earlier suite run used the chains unevenly. */
export async function alignDeploymentNonces(deployers: Deployer[]): Promise<void> {
  const addresses = await Promise.all(deployers.map((deployer) => deployer.getAddress()));
  if (new Set(addresses.map((address) => address.toLowerCase())).size !== 1) {
    throw new Error('Matching deployment addresses require the same deployer on every chain');
  }
  const nonces = await Promise.all(deployers.map((deployer) => deployer.getNonce('pending')));
  const target = Math.max(...nonces);
  for (const [index, deployer] of deployers.entries()) {
    for (let nonce = nonces[index]; nonce < target; nonce++) {
      // Advance only the lagging chain, using mined transactions rather than
      // changing node state or reverting history already ingested by the stack.
      const transaction = await deployer.sendTransaction({ to: addresses[index], value: 0n });
      if ((await transaction.wait())?.status !== 1) {
        throw new Error(`Failed to align deployment nonce on chain ${index}`);
      }
    }
  }
}
