import { config, ethers } from 'hardhat';
import type { Signer } from 'ethers';

export interface Signers {
  alice: Signer & { address: string };
  bob: Signer & { address: string };
  carol: Signer & { address: string };
  dave: Signer & { address: string };
  eve: Signer & { address: string };
}

let signers: Signers | undefined = undefined;

const signerNames: Array<keyof Signers> = ['alice', 'bob', 'carol', 'dave', 'eve'];

async function _faucet(address: string): Promise<void> {
  const balance = await ethers.provider.getBalance(address);
  if (balance === 0n) {
    throw new Error(`Signer ${address} has no balance and no faucet is available to fund it.`);
  }
}

// Set by `hardhat test --parallel`. Any non-empty value means parallel mode.
function _isHardhatParallel(): boolean {
  const value = process.env.HARDHAT_PARALLEL;
  return value !== undefined && value !== '';
}

export async function initSigners(quantity: number): Promise<void> {
  const q = _isHardhatParallel() ? Math.min(quantity, 5) : 5;
  if (!signers) {
    if (_isHardhatParallel() && config.defaultNetwork === 'local') {
      signers = {
        alice: ethers.Wallet.createRandom().connect(ethers.provider),
        bob: ethers.Wallet.createRandom().connect(ethers.provider),
        carol: ethers.Wallet.createRandom().connect(ethers.provider),
        dave: ethers.Wallet.createRandom().connect(ethers.provider),
        eve: ethers.Wallet.createRandom().connect(ethers.provider),
      };
    } else if (!_isHardhatParallel()) {
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
      const faucetP: Array<Promise<void>> = [];
      for (let i = 0; i < q; i += 1) {
        const account = signers[signerNames[i]];
        faucetP.push(_faucet(account.address));
      }
      await Promise.all(faucetP);
    }
  }
}

// eslint-disable-next-line @typescript-eslint/require-await
export async function getSigners(): Promise<Signers> {
  if (signers === undefined) {
    throw new Error('Signers not initialized: call initSigners() first');
  }
  return signers;
}
