// The per-connection fhevm object — hardhat 3 scopes networks to CONNECTIONS, so fhevm state lives on
// each one (v2 had a per-process singleton). It implements the public surface in ../types.ts over the
// SDK client and the contracts repository of a development connection; on any other network the
// members that need them fail by name.

import { HardhatPluginError } from 'hardhat/plugins';
import type { Abi, Address, Hex } from 'viem';

import {
  type FhevmClient,
  type CoprocessorConfig,
  type CoprocessorEvent,
  type FhevmAddressLike,
  type FhevmContractError,
  type FhevmContractName,
  type FhevmEncryptedInput,
  type FhevmErrorInterface,
  type FhevmLog,
  type FhevmTransactionHCUInfo,
  type FhevmTransactionReceipt,
  type FhevmTypeName,
  type FhevmNetworkInfo,
  FhevmType,
  type PublicDecryptResults,
  type FhevmTypeEuint,
  type FhevmUser,
  type FhevmUserDecryptOptions,
  type HardhatFhevmRuntimeDebugger,
  type HardhatFhevmRuntimeEnvironment,
} from '../types.js';
import { PLUGIN_ID } from './constants.js';
import type { FhevmContractsRepository } from './contracts.js';
import { assertCoprocessorInitialized, readCoprocessorConfig, resolveAddress } from './coprocessorConfig.js';
import { asAddress, asBigInt, asBoolean, publicDecrypt, publicDecryptOne } from './decrypt.js';
import { createEncryptedInput, encryptOne } from './encrypt.js';
import { parseCoprocessorEvents } from './events.js';
import { createDebugger } from './debugger.js';
import { createErrorInterface } from './errors/interface.js';
import { parseFhevmHandle } from './fhevmHandle.js';
import { computeTransactionHCU } from './hcu/hcu.js';
import { parseFhevmError } from './errors/parse.js';
import { isFhevmEuint } from './fheType.js';
import { type LogOutput, logBox } from './log.js';
import { isCleartextNetwork, isDevelopmentNetwork } from './network.js';
import { userDecryptOne } from './userDecrypt.js';

class FhevmRuntimeEnvironment implements HardhatFhevmRuntimeEnvironment {
  readonly network: FhevmNetworkInfo;
  readonly isCleartext: boolean;
  readonly isDevelopment: boolean;
  readonly #client: FhevmClient | undefined;
  readonly #repository: FhevmContractsRepository | undefined;

  constructor(
    network: FhevmNetworkInfo,
    client: FhevmClient | undefined,
    repository: FhevmContractsRepository | undefined,
  ) {
    this.network = network;
    this.isCleartext = isCleartextNetwork(network);
    this.isDevelopment = isDevelopmentNetwork(network);
    this.#client = client;
    this.#repository = repository;
  }

  get #contracts(): FhevmContractsRepository {
    if (this.#repository !== undefined) return this.#repository;
    const { networkName, chainId } = this.network;
    throw new HardhatPluginError(
      PLUGIN_ID,
      `The FHEVM contracts are not available on '${networkName}' (chainId ${String(chainId)}): only development networks are supported yet.`,
    );
  }

  get isMock(): boolean {
    return this.isCleartext;
  }

  get debugger(): HardhatFhevmRuntimeDebugger {
    return createDebugger(this.#contracts, this.network);
  }

  get client(): FhevmClient {
    if (this.#client !== undefined) return this.#client;
    const { networkName, chainId } = this.network;
    throw new HardhatPluginError(
      PLUGIN_ID,
      `fhevm.client is not available on '${networkName}' (chainId ${String(chainId)}): only development networks are supported yet.`,
    );
  }

  typeof(handleBytes32: Hex): FhevmTypeName {
    return parseFhevmHandle(handleBytes32).typeName;
  }

  parseCoprocessorEvents(logs: readonly FhevmLog[] | null | undefined): CoprocessorEvent[] {
    return parseCoprocessorEvents(this.#contracts.fhevmExecutor, logs);
  }

  computeTransactionHCU(transactionReceipt: FhevmTransactionReceipt): FhevmTransactionHCUInfo {
    return computeTransactionHCU(this.#contracts.fhevmExecutor, transactionReceipt);
  }

  assertCoprocessorInitialized(contract: FhevmAddressLike, contractName?: string): Promise<void> {
    return assertCoprocessorInitialized(this.#contracts, contract, contractName);
  }

  async getCoprocessorConfig(contract: FhevmAddressLike): Promise<CoprocessorConfig> {
    return readCoprocessorConfig(this.#contracts.client, await resolveAddress(contract));
  }

  revertedWithCustomErrorArgs(
    contractName: FhevmContractName,
    customErrorName: string,
  ): [{ abi: Abi; interface: FhevmErrorInterface }, string] {
    const wrapper = this.#contracts.getContractFromName(contractName);
    if (wrapper === undefined) {
      throw new HardhatPluginError(PLUGIN_ID, `Unknown FHEVM contract '${contractName}' on this network.`);
    }
    const errorInterface = createErrorInterface(wrapper);
    if (errorInterface.getError(customErrorName) === null) {
      throw new HardhatPluginError(
        PLUGIN_ID,
        `FHEVM contract '${contractName}' declares no custom error '${customErrorName}'.`,
      );
    }
    return [{ abi: wrapper.abi, interface: errorInterface }, customErrorName];
  }

  async tryParseFhevmError(e: unknown, options?: { out?: LogOutput }): Promise<FhevmContractError | undefined> {
    const error = await parseFhevmError(this.#contracts, e);
    if (error !== undefined && options?.out !== undefined)
      logBox(`${error.name} error`, error.longMessage, options.out);
    return error;
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

  async userDecryptEbool(
    handleBytes32: Hex,
    contractAddress: Address,
    user: FhevmUser,
    options?: FhevmUserDecryptOptions,
  ): Promise<boolean> {
    const value = await userDecryptOne(this.client, 'userDecryptEbool', handleBytes32, contractAddress, user, options);
    return asBoolean(value, handleBytes32);
  }

  async userDecryptEuint(
    _fhevmType: FhevmTypeEuint,
    handleBytes32: Hex,
    contractAddress: Address,
    user: FhevmUser,
    options?: FhevmUserDecryptOptions,
  ): Promise<bigint> {
    const value = await userDecryptOne(this.client, 'userDecryptEuint', handleBytes32, contractAddress, user, options);
    return asBigInt(value, handleBytes32);
  }

  async userDecryptEaddress(
    handleBytes32: Hex,
    contractAddress: Address,
    user: FhevmUser,
    options?: FhevmUserDecryptOptions,
  ): Promise<Address> {
    const value = await userDecryptOne(
      this.client,
      'userDecryptEaddress',
      handleBytes32,
      contractAddress,
      user,
      options,
    );
    return asAddress(value, handleBytes32);
  }
}

export function createFhevmConnection(
  network: FhevmNetworkInfo,
  client: FhevmClient | undefined,
  repository: FhevmContractsRepository | undefined,
): HardhatFhevmRuntimeEnvironment {
  return Object.freeze(new FhevmRuntimeEnvironment(network, client, repository));
}
