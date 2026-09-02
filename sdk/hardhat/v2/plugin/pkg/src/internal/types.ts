import type { ethers as EthersT } from 'ethers';
import type { FhevmEnvironment } from './FhevmEnvironment';

export type FhevmContext = {
  fhevmEnv: FhevmEnvironment | undefined;
  rand: number;
  get: () => FhevmEnvironment;
};

export interface FhevmProvider extends EthersT.Provider {
  send(method: string, params?: unknown[]): Promise<unknown>;
}
