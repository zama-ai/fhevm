import type { ContractFactory } from 'ethers';
import { upgrades } from 'hardhat';

export async function deployEmptyProxy(factory: ContractFactory, args: unknown[] = []): Promise<string> {
  const proxy = await upgrades.deployProxy(factory, args, { initializer: 'initialize', kind: 'uups' });
  await proxy.waitForDeployment();
  return proxy.getAddress();
}
