// The per-connection fhevm object — hardhat 3 scopes networks to CONNECTIONS, so fhevm state lives on
// each one (v2 had a per-process singleton). It implements the public surface in ../types.ts; the
// network facts are real, and every method not ported yet throws a named not-implemented error so a
// test reaches for a missing group by name instead of by a TypeError.

import { HardhatPluginError } from 'hardhat/plugins';
import type { NetworkConnection } from 'hardhat/types/network';

import type {
  FhevmClient,
  FhevmNetworkInfo,
  HardhatFhevmRuntimeDebugger,
  HardhatFhevmRuntimeEnvironment,
} from '../types.js';
import { PLUGIN_ID } from './constants.js';
import { isCleartextNetwork, isDevelopmentNetwork } from './network.js';

function notImplemented(member: string): never {
  throw new HardhatPluginError(PLUGIN_ID, `fhevm.${member} is not implemented yet in the hardhat 3 plugin.`);
}

class FhevmRuntimeEnvironment implements HardhatFhevmRuntimeEnvironment {
  readonly network: FhevmNetworkInfo;
  readonly isCleartext: boolean;
  readonly isDevelopment: boolean;

  constructor(network: FhevmNetworkInfo) {
    this.network = network;
    this.isCleartext = isCleartextNetwork(network);
    this.isDevelopment = isDevelopmentNetwork(network);
  }

  get isMock(): boolean {
    return this.isCleartext;
  }

  get debugger(): HardhatFhevmRuntimeDebugger {
    return notImplemented('debugger');
  }

  get client(): FhevmClient {
    return notImplemented('client');
  }

  typeof(): never {
    return notImplemented('typeof');
  }

  parseCoprocessorEvents(): never {
    return notImplemented('parseCoprocessorEvents');
  }

  computeTransactionHCU(): never {
    return notImplemented('computeTransactionHCU');
  }

  assertCoprocessorInitialized(): Promise<never> {
    return Promise.reject(notImplementedError('assertCoprocessorInitialized'));
  }

  getCoprocessorConfig(): Promise<never> {
    return Promise.reject(notImplementedError('getCoprocessorConfig'));
  }

  revertedWithCustomErrorArgs(): never {
    return notImplemented('revertedWithCustomErrorArgs');
  }

  tryParseFhevmError(): Promise<never> {
    return Promise.reject(notImplementedError('tryParseFhevmError'));
  }

  createEncryptedInput(): never {
    return notImplemented('createEncryptedInput');
  }

  encryptUint(): Promise<never> {
    return Promise.reject(notImplementedError('encryptUint'));
  }

  encryptBool(): Promise<never> {
    return Promise.reject(notImplementedError('encryptBool'));
  }

  encryptAddress(): Promise<never> {
    return Promise.reject(notImplementedError('encryptAddress'));
  }

  createEIP712(): never {
    return notImplemented('createEIP712');
  }

  createDelegatedUserDecryptEIP712(): never {
    return notImplemented('createDelegatedUserDecryptEIP712');
  }

  publicDecrypt(): Promise<never> {
    return Promise.reject(notImplementedError('publicDecrypt'));
  }

  publicDecryptEbool(): Promise<never> {
    return Promise.reject(notImplementedError('publicDecryptEbool'));
  }

  publicDecryptEuint(): Promise<never> {
    return Promise.reject(notImplementedError('publicDecryptEuint'));
  }

  publicDecryptEaddress(): Promise<never> {
    return Promise.reject(notImplementedError('publicDecryptEaddress'));
  }

  userDecryptEbool(): Promise<never> {
    return Promise.reject(notImplementedError('userDecryptEbool'));
  }

  userDecryptEuint(): Promise<never> {
    return Promise.reject(notImplementedError('userDecryptEuint'));
  }

  userDecryptEaddress(): Promise<never> {
    return Promise.reject(notImplementedError('userDecryptEaddress'));
  }
}

function notImplementedError(member: string): HardhatPluginError {
  return new HardhatPluginError(PLUGIN_ID, `fhevm.${member} is not implemented yet in the hardhat 3 plugin.`);
}

export function createFhevmConnection(
  _connection: NetworkConnection<string>,
  network: FhevmNetworkInfo,
): HardhatFhevmRuntimeEnvironment {
  return Object.freeze(new FhevmRuntimeEnvironment(network));
}
