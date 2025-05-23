import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { exec as oldExec } from 'child_process';
import { config, ethers } from 'hardhat';
import { promisify } from 'util';

import { waitForBalance } from './utils';

const exec = promisify(oldExec);

export interface Signers {
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
  carol: HardhatEthersSigner;
  dave: HardhatEthersSigner;
  eve: HardhatEthersSigner;
}

let signers: Signers;

const keys: (keyof Signers)[] = ['alice', 'bob', 'carol', 'dave', 'eve'];

const getCoin = async (address: string) => {
  const containerName = process.env['TEST_CONTAINER_NAME'] || 'zama-dev-fhevm-validator-1';
  const response = await exec(`docker exec -i ${containerName} faucet ${address} | grep height`);
  const res = JSON.parse(response.stdout);
  if (res.raw_log.match('account sequence mismatch')) await getCoin(address);
};

const faucet = async (address: string) => {
  const balance = await ethers.provider.getBalance(address);
  if (balance > 0) return;
  await getCoin(address);
  await waitForBalance(address);
};

export const initSigners = async (quantity: number): Promise<void> => {
  const q = process.env.HARDHAT_PARALLEL ? Math.min(quantity, 5) : 5;
  if (!signers) {
    if (process.env.HARDHAT_PARALLEL && config.defaultNetwork === 'local') {
      signers = {
        alice: ethers.Wallet.createRandom().connect(ethers.provider),
        bob: ethers.Wallet.createRandom().connect(ethers.provider),
        carol: ethers.Wallet.createRandom().connect(ethers.provider),
        dave: ethers.Wallet.createRandom().connect(ethers.provider),
        eve: ethers.Wallet.createRandom().connect(ethers.provider),
      };
    } else if (!process.env.HARDHAT_PARALLEL) {
      const eSigners = await ethers.getSigners();
      signers = {
        alice: eSigners[0],
        bob: eSigners[1],
        carol: eSigners[2],
        dave: eSigners[3],
        eve: eSigners[4],
      };
    } else {
      throw new Error("Can't run parallel mode if network is not 'local'");
    }

    if (config.defaultNetwork === 'local') {
      const faucetP: Promise<void>[] = [];
      for (let i = 0; i < q; i += 1) {
        const account = signers[keys[i]];
        faucetP.push(faucet(account.address));
      }
      await Promise.all(faucetP);
    }
  }
};

export const getSigners = async (): Promise<Signers> => {
  return signers;
};

export const requestFaucet = faucet;
