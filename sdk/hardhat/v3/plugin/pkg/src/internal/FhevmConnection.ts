// The per-connection fhevm object — hardhat 3 scopes networks to CONNECTIONS, so fhevm state lives on
// each one (v2 had a per-process singleton). It implements the public surface in ../types.ts; the
// network facts are real, and every method not ported yet throws a named not-implemented error so a
// test reaches for a missing group by name instead of by a TypeError.

import { HardhatPluginError } from 'hardhat/plugins';
import type { Address, Hex } from 'viem';

import {
  type FhevmClient,
  type FhevmEncryptedInput,
  type FhevmNetworkInfo,
  FhevmType,
  type PublicDecryptResults,
  type FhevmTypeEuint,
  type HardhatFhevmRuntimeDebugger,
  type HardhatFhevmRuntimeEnvironment,
} from '../types.js';
import { PLUGIN_ID } from './constants.js';
import { asAddress, asBigInt, asBoolean, publicDecrypt, publicDecryptOne } from './decrypt.js';
import { createEncryptedInput, encryptOne } from './encrypt.js';
import { isFhevmEuint } from './fheType.js';
import { isCleartextNetwork, isDevelopmentNetwork } from './network.js';

function notImplementedError(member: string): HardhatPluginError {
  return new HardhatPluginError(PLUGIN_ID, `fhevm.${member} is not implemented yet in the hardhat 3 plugin.`);
}

function notImplemented(member: string): never {
  throw notImplementedError(member);
}

class FhevmRuntimeEnvironment implements HardhatFhevmRuntimeEnvironment {
  readonly network: FhevmNetworkInfo;
  readonly isCleartext: boolean;
  readonly isDevelopment: boolean;
  readonly #client: FhevmClient | undefined;

  constructor(network: FhevmNetworkInfo, client: FhevmClient | undefined) {
    this.network = network;
    this.isCleartext = isCleartextNetwork(network);
    this.isDevelopment = isDevelopmentNetwork(network);
    this.#client = client;
  }

  get isMock(): boolean {
    return this.isCleartext;
  }

  get debugger(): HardhatFhevmRuntimeDebugger {
    return notImplemented('debugger');
  }

  get client(): FhevmClient {
    if (this.#client !== undefined) return this.#client;
    const { networkName, chainId } = this.network;
    throw new HardhatPluginError(
      PLUGIN_ID,
      `fhevm.client is not available on '${networkName}' (chainId ${String(chainId)}): only development networks are supported yet.`,
    );
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

  createEncryptedInput(contractAddress: Address, userAddress: Address): FhevmEncryptedInput {
    return createEncryptedInput(() => this.client, contractAddress, userAddress);
  }

  async encryptUint(
    fhevmType: FhevmTypeEuint,
    value: number | bigint,
    contractAddress: Address,
    userAddress: Address,
  ): Promise<{ externalEuint: Hex; inputProof: Hex }> {
    if (!isFhevmEuint(fhevmType)) {
      throw new HardhatPluginError(PLUGIN_ID, `encryptUint: '${String(fhevmType)}' is not a valid FhevmTypeEuint.`);
    }
    const { handle, inputProof } = await encryptOne(
      this.client,
      'encryptUint',
      fhevmType,
      value,
      contractAddress,
      userAddress,
    );
    return { externalEuint: handle, inputProof };
  }

  async encryptBool(
    value: boolean,
    contractAddress: Address,
    userAddress: Address,
  ): Promise<{ externalEbool: Hex; inputProof: Hex }> {
    const { handle, inputProof } = await encryptOne(
      this.client,
      'encryptBool',
      FhevmType.ebool,
      value,
      contractAddress,
      userAddress,
    );
    return { externalEbool: handle, inputProof };
  }

  async encryptAddress(
    value: Address,
    contractAddress: Address,
    userAddress: Address,
  ): Promise<{ externalEaddress: Hex; inputProof: Hex }> {
    const { handle, inputProof } = await encryptOne(
      this.client,
      'encryptAddress',
      FhevmType.eaddress,
      value,
      contractAddress,
      userAddress,
    );
    return { externalEaddress: handle, inputProof };
  }

  publicDecrypt(handles: Array<Hex | Uint8Array>): Promise<PublicDecryptResults> {
    return publicDecrypt(this.client, handles);
  }

  async publicDecryptEbool(handleBytes32: Hex): Promise<boolean> {
    return asBoolean(await publicDecryptOne(this.client, handleBytes32), handleBytes32);
  }

  async publicDecryptEuint(_fhevmType: FhevmTypeEuint, handleBytes32: Hex): Promise<bigint> {
    return asBigInt(await publicDecryptOne(this.client, handleBytes32), handleBytes32);
  }

  async publicDecryptEaddress(handleBytes32: Hex): Promise<Address> {
    return asAddress(await publicDecryptOne(this.client, handleBytes32), handleBytes32);
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

export function createFhevmConnection(
  network: FhevmNetworkInfo,
  client: FhevmClient | undefined,
): HardhatFhevmRuntimeEnvironment {
  return Object.freeze(new FhevmRuntimeEnvironment(network, client));
}
