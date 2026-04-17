import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, upgrades } from 'hardhat';
import path from 'path';

import type { KMSGeneration } from '../../types';

export const HOST_ENV_FILE = path.join(__dirname, '../../addresses/.env.host');

export function readHostAddress(key: string): string {
  const value = dotenv.parse(fs.readFileSync(HOST_ENV_FILE))[key];
  if (!value) {
    throw new Error(`Missing ${key} in ${HOST_ENV_FILE}`);
  }
  return value;
}

export async function deployFreshEmptyProxy(deployer: Wallet): Promise<string> {
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const proxy = await upgrades.deployProxy(factory, { initializer: 'initialize', kind: 'uups' });
  await proxy.waitForDeployment();
  return proxy.getAddress();
}

export function buildProtocolConfigNodes(): Array<{
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
}> {
  return [
    {
      txSenderAddress: '0x0000000000000000000000000000000000001111',
      signerAddress: '0x0000000000000000000000000000000000002222',
      ipAddress: '127.0.0.1',
      storageUrl: 'https://s0.example.com',
    },
    {
      txSenderAddress: '0x0000000000000000000000000000000000003333',
      signerAddress: '0x0000000000000000000000000000000000004444',
      ipAddress: '127.0.0.2',
      storageUrl: 'https://s1.example.com',
    },
    {
      txSenderAddress: '0x0000000000000000000000000000000000005555',
      signerAddress: '0x0000000000000000000000000000000000006666',
      ipAddress: '127.0.0.3',
      storageUrl: 'https://s2.example.com',
    },
    {
      txSenderAddress: '0x0000000000000000000000000000000000007777',
      signerAddress: '0x0000000000000000000000000000000000008888',
      ipAddress: '127.0.0.4',
      storageUrl: 'https://s3.example.com',
    },
  ];
}

export function buildProtocolConfigThresholds() {
  return {
    publicDecryption: 1,
    userDecryption: 2,
    kmsGen: 3,
    mpc: 4,
  };
}

export async function withPatchedMethods<T extends object, R>(
  target: T,
  patches: Partial<{ [K in keyof T]: T[K] }>,
  action: () => Promise<R>,
): Promise<R> {
  const originalValues = new Map<keyof T, T[keyof T]>();

  for (const [key, value] of Object.entries(patches) as Array<[keyof T, T[keyof T]]>) {
    originalValues.set(key, target[key]);
    target[key] = value;
  }

  try {
    return await action();
  } finally {
    for (const [key, value] of originalValues.entries()) {
      target[key] = value;
    }
  }
}

export async function deployFreshKMSGenerationProxy(deployer: Wallet): Promise<KMSGeneration> {
  const proxyAddress = await deployFreshEmptyProxy(deployer);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory('KMSGeneration', deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const upgraded = await upgrades.upgradeProxy(proxy, newImplementation, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  await upgraded.waitForDeployment();

  return (await ethers.getContractAt('KMSGeneration', proxyAddress, deployer)) as unknown as KMSGeneration;
}
