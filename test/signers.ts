import { exec as oldExec } from 'child_process';
import { NonceManager } from 'ethers';
import { config, ethers } from 'hardhat';
import { promisify } from 'util';

import { waitForBalance } from './utils';

// Module augmentation to add 'address' to NonceManager
declare module 'ethers' {
  interface NonceManager {
    address: string;
  }
}

// Extend the NonceManager prototype
Object.defineProperty(ethers.NonceManager.prototype, 'address', {
  get: function () {
    return this.signer.address;
  },
  enumerable: true,
});

const exec = promisify(oldExec);

export interface Signers {
  alice: NonceManager;
  bob: NonceManager;
  carol: NonceManager;
  dave: NonceManager;
  eve: NonceManager;
}

let signers: Signers;

const keys: (keyof Signers)[] = ['alice', 'bob', 'carol', 'dave', 'eve'];

const getCoin = async (address: string) => {
  const containerName = process.env['TEST_CONTAINER_NAME'] || 'fhevm';
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
  const q = process.env.HARDHAT_PARALLEL ? Math.min(quantity, 4) : 4;
  if (!signers) {
    if (process.env.HARDHAT_PARALLEL && config.defaultNetwork === 'local') {
      signers = {
        alice: new NonceManager(ethers.Wallet.createRandom().connect(ethers.provider)),
        bob: new NonceManager(ethers.Wallet.createRandom().connect(ethers.provider)),
        carol: new NonceManager(ethers.Wallet.createRandom().connect(ethers.provider)),
        dave: new NonceManager(ethers.Wallet.createRandom().connect(ethers.provider)),
        eve: new NonceManager(ethers.Wallet.createRandom().connect(ethers.provider)),
      };
    } else if (!process.env.HARDHAT_PARALLEL) {
      const eSigners = await ethers.getSigners();
      signers = {
        alice: new NonceManager(eSigners[0]),
        bob: new NonceManager(eSigners[1]),
        carol: new NonceManager(eSigners[2]),
        dave: new NonceManager(eSigners[3]),
        eve: new NonceManager(eSigners[4]),
      };
    } else {
      throw new Error("Can't run parallel mode if network is not 'local'");
    }

    if (config.defaultNetwork === 'local') {
      const faucetP: Promise<void>[] = [];
      for (let i = 0; i < q; i += 1) {
        const account = signers[keys[i]];
        faucetP.push(faucet(await account.signer.getAddress()));
      }
      await Promise.all(faucetP);
    }
  }
};

export const getSigners = async (): Promise<Signers> => {
  return signers;
};

export const requestFaucet = faucet;
