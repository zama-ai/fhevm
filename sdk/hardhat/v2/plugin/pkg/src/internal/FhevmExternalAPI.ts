import { type AddressLike, ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';
import type { HardhatFhevmRuntimeDebugger, HardhatFhevmRuntimeEnvironment } from '../types';
import type { FhevmEnvironment } from './FhevmEnvironment';
import constants from './constants';
import { type CoprocessorConfig, getCoprocessorConfig } from './coprocessorConfig';
import { type FhevmEncryptedInput, createEncryptedInput } from './encryptedInput';
import { type FhevmContractError, parseFhevmError } from './errors/FhevmContractError';
import { FhevmType, type FhevmTypeEuint, type FhevmTypeName, fhevmTypeToSdkType, isFhevmEuint } from './fheType';
import { assertHandleIsInitialized, parseFhevmHandle } from './fhevmHandle';
import { isCoprocessorEventName } from './hcu/events';
import { getTxHCUFromTxReceipt } from './hcu/hcu';
import type {
  CoprocessorEvent,
  FhevmContractName,
  FhevmPublicDecryptOptions,
  FhevmTransactionHCUInfo,
  FhevmUserDecryptOptions,
  HandleContractPair,
  KmsDelegatedUserDecryptEIP712Type,
  KmsUserDecryptEIP712Type,
  PublicDecryptResults,
  RelayerEncryptedInput,
  RelayerMetadata,
  UserDecryptResults,
} from './migration/placeholders';
import type { FhevmClient } from './sdkTypes';
import { logBox } from './utils/log';

/** Matches the legacy default from `@fhevm/mock-utils`. */
const DEFAULT_DURATION_DAYS = 365;

/**
 * A decrypted value as `@fhevm/sdk` returns it: `{ type: 'uint32', value: 123n }`. The legacy API
 * returned the bare value, so every `*E*` method unwraps one of these.
 */
type TypedValueLike = { readonly type: string; readonly value: unknown };

// eslint-disable-next-line @typescript-eslint/naming-convention
function __handleKey(handle: string | Uint8Array): string {
  return typeof handle === 'string' ? handle : EthersT.hexlify(handle);
}

// eslint-disable-next-line @typescript-eslint/naming-convention
function __typedValue(value: TypedValueLike | undefined): bigint | boolean | string {
  if (value === undefined) {
    throw new HardhatFhevmError(`Missing decrypted value in the response.`);
  }
  const v = value.value;
  if (typeof v === 'bigint' || typeof v === 'boolean' || typeof v === 'string') {
    return v;
  }
  if (typeof v === 'number') {
    return BigInt(v);
  }
  throw new HardhatFhevmError(`Unexpected decrypted value type '${typeof v}'.`);
}

// eslint-disable-next-line @typescript-eslint/naming-convention
function __asBoolean(value: TypedValueLike, handleBytes32: string): boolean {
  const v = __typedValue(value);
  if (typeof v !== 'boolean') {
    throw new HardhatFhevmError(
      `Unexpected type for decrypted value of ebool handle '${handleBytes32}': expected a boolean, but got '${typeof v}' instead.`,
    );
  }
  return v;
}

// eslint-disable-next-line @typescript-eslint/naming-convention
function __asBigInt(value: TypedValueLike, handleBytes32: string): bigint {
  const v = __typedValue(value);
  if (typeof v !== 'bigint') {
    throw new HardhatFhevmError(
      `Unexpected type for decrypted value of handle '${handleBytes32}': expected a bigint, but got '${typeof v}' instead.`,
    );
  }
  return v;
}

// eslint-disable-next-line @typescript-eslint/naming-convention
function __asAddress(value: TypedValueLike, handleBytes32: string): string {
  const v = __typedValue(value);
  if (typeof v !== 'string' || !EthersT.isAddress(v)) {
    throw new HardhatFhevmError(
      `Unexpected type for decrypted value of eaddress handle '${handleBytes32}': expected an address, but got '${String(v)}' instead.`,
    );
  }
  return v;
}

/**
 * Public External API, implemented on `@fhevm/sdk`'s `FhevmClient`.
 *
 * The method names and signatures are inherited from the `@zama-fhe/relayer-sdk` era on purpose: they
 * are what the test suite and downstream users call, and keeping them stable is what let the suite act
 * as the regression net while every body was rewritten.
 *
 * The `@deprecated` members are the exception — they expose the relayer-sdk's keypair/EIP-712 model,
 * which decryption permits replace, and they throw rather than being ported.
 */
export class FhevmExternalAPI implements HardhatFhevmRuntimeEnvironment {
  public readonly fhevmEnv: FhevmEnvironment;

  constructor(fhevmEnv: FhevmEnvironment) {
    this.fhevmEnv = fhevmEnv;
  }

  /**
   * Uniform failure for entries that are going away rather than being ported.
   */
  private __deprecated(method: string, replacement: string): never {
    throw new HardhatFhevmError(
      `fhevm.${method}() is deprecated and will be removed: it exposes the @zama-fhe/relayer-sdk model, which the plugin no longer uses. Use ${replacement} instead.`,
    );
  }

  //////////////////////////////////////////////////////////////////////////////
  // Environment
  //////////////////////////////////////////////////////////////////////////////

  public async initializeCLIApi(): Promise<void> {
    await this.fhevmEnv.initializeCLIApi();
  }

  /** @deprecated  */
  public get isMock(): boolean {
    return this.fhevmEnv.cleartextProvider.isCleartext;
  }

  public get isCleartext(): boolean {
    return this.fhevmEnv.cleartextProvider.isCleartext;
  }

  public get isDevelopment(): boolean {
    return this.fhevmEnv.cleartextProvider.isDevelopment;
  }

  public get debugger(): HardhatFhevmRuntimeDebugger {
    return this.fhevmEnv.debugger;
  }

  /** The client every action below goes through. Already resolved — `createInstance` awaits `ready`. */
  public get client(): FhevmClient {
    return this.fhevmEnv.instance;
  }

  /** The client every action below goes through. Already resolved — `createInstance` awaits `ready`. */
  private get __client(): FhevmClient {
    return this.fhevmEnv.instance;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Introspection / diagnostics
  //////////////////////////////////////////////////////////////////////////////

  /** The FHE type a handle encodes, read from byte 30 of the handle itself. */
  public typeof(handleBytes32: string): FhevmTypeName {
    return parseFhevmHandle(handleBytes32).typeName;
  }

  public async tryParseFhevmError(
    e: unknown,
    options?: {
      encryptedInput?: RelayerEncryptedInput;
      out?: 'stderr' | 'stdout' | 'console';
    },
  ): Promise<FhevmContractError | undefined> {
    const err = await parseFhevmError(this.fhevmEnv, e, options);
    if (err && options?.out !== undefined) {
      logBox(`${err.name} error`, err.longMessage, options);
    }
    return err;
  }

  /**
   * Chai matcher support: `expect(tx).to.be.revertedWithCustomError(...fhevm.revertedWithCustomErrorArgs(...))`.
   */
  public revertedWithCustomErrorArgs(
    contractName: FhevmContractName,
    customErrorName: string,
  ): [{ interface: EthersT.Interface }, string] {
    const itf = this.fhevmEnv.getContractsRepository().getContractFromName(contractName)?.interface;
    if (!itf) {
      throw new HardhatFhevmError(`Unable to retrieve the FHEVM contract interface for contract ${contractName}`);
    }
    return [{ interface: itf }, customErrorName];
  }

  /**
   * The FHEVMExecutor operator events in a transaction's logs.
   *
   * Not needed to *compute* anything any more — the cleartext stack evaluates operators on-chain —
   * but the events are still emitted, and this exposes them for inspection.
   */
  public parseCoprocessorEvents(logs: Array<EthersT.EventLog | EthersT.Log> | null | undefined): CoprocessorEvent[] {
    if (!logs) {
      return [];
    }

    const itf = this.fhevmEnv.getContractsRepository().fhevmExecutor.interface;
    const events: CoprocessorEvent[] = [];

    for (const log of logs) {
      let parsed: EthersT.LogDescription | null;
      try {
        parsed = itf.parseLog(log);
      } catch {
        continue;
      }
      if (!parsed || !isCoprocessorEventName(parsed.name)) {
        continue;
      }
      events.push({
        eventName: parsed.name,
        args: parsed.args,
        blockNumber: log.blockNumber,
        index: log.index,
        transactionHash: log.transactionHash,
        transactionIndex: log.transactionIndex,
      });
    }

    return events;
  }

  /**
   * The HCU (Homomorphic Complexity Units) a transaction consumed.
   *
   * A cost model applied to the executor's operator events, not something read off-chain — which is
   * why it survived the move to an on-chain cleartext stack: `CleartextFHEVMExecutor` emits the same
   * events the pricing table is keyed by.
   */
  public computeTransactionHCU(transactionReceipt: EthersT.TransactionReceipt): FhevmTransactionHCUInfo {
    const executor = this.fhevmEnv.getContractsRepository().fhevmExecutor;
    return getTxHCUFromTxReceipt(executor.address as `0x${string}`, executor.interface, transactionReceipt);
  }

  public async getCoprocessorConfig(contractAddress: string): Promise<CoprocessorConfig> {
    return await getCoprocessorConfig(this.fhevmEnv.readonlyEthersProvider, contractAddress);
  }

  /**
   * Checks that a dApp contract was compiled against the FHEVM stack this network actually runs.
   *
   * The classic failure this catches: a contract inheriting `ZamaConfig`'s Sepolia addresses being
   * deployed on a local node, or vice versa. It reads the config the contract stored rather than
   * trusting the source.
   */
  public async assertCoprocessorInitialized(contract: AddressLike, contractName?: string): Promise<void> {
    const contractAddress = await this.fhevmEnv.hre.ethers.resolveAddress(contract);

    const expected = {
      ACLAddress: this.fhevmEnv.getACLAddress(),
      CoprocessorAddress: this.fhevmEnv.getFHEVMExecutorAddress(),
      KMSVerifierAddress: this.fhevmEnv.getKMSVerifierAddress(),
    };

    const prefix =
      contractName === undefined ? `Contract at ${contractAddress}` : `Contract ${contractName} at ${contractAddress}`;
    const configFile = `${constants.FHEVM_SOLIDITY_PACKAGE.name}/${constants.FHEVM_SOLIDITY_PACKAGE.configFile}`;

    const actual = await this.getCoprocessorConfig(contractAddress);

    if (
      actual.ACLAddress === EthersT.ZeroAddress ||
      actual.CoprocessorAddress === EthersT.ZeroAddress ||
      actual.KMSVerifierAddress === EthersT.ZeroAddress
    ) {
      throw new HardhatFhevmError(
        `${prefix} is not initialized for FHE operations. Make sure it either inherits from ${configFile}:${constants.FHEVM_SOLIDITY_PACKAGE.configContractName} or explicitly calls FHE.setCoprocessor() in its constructor.`,
      );
    }

    const mismatch = `${prefix} was initialized with FHEVM contract addresses that do not match the currently deployed FHEVM contracts. This is likely due to incorrect addresses in the file ${configFile}`;
    for (const key of ['ACLAddress', 'CoprocessorAddress', 'KMSVerifierAddress'] as const) {
      if (actual[key] !== expected[key]) {
        throw new HardhatFhevmError(
          `Coprocessor ${key} mismatch. ${mismatch}. ${key}: ${actual[key]}, expected ${key}: ${expected[key]}`,
        );
      }
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Encryption
  //////////////////////////////////////////////////////////////////////////////

  /**
   * One encrypted input. `@fhevm/sdk` takes the value and its type directly, where the relayer-sdk
   * needed an input builder (`createEncryptedInput(…).add32(v).encrypt()`) — which is why
   * `createEncryptedInput` is deprecated rather than ported.
   */
  private async __encrypt(
    fhevmType: FhevmType,
    value: boolean | bigint | number | string,
    contractAddress: string,
    userAddress: string,
  ): Promise<{ handle: string; inputProof: string }> {
    if (!EthersT.isAddress(contractAddress)) {
      throw new HardhatFhevmError(
        `The 'contractAddress' argument is not a valid address. Got '${contractAddress}' instead.`,
      );
    }
    if (!EthersT.isAddress(userAddress)) {
      throw new HardhatFhevmError(`The 'userAddress' argument is not a valid address. Got '${userAddress}' instead.`);
    }

    const res = await this.__client.encryptValue({
      value: { type: fhevmTypeToSdkType(fhevmType), value },
      contractAddress,
      userAddress,
    });

    return { handle: res.encryptedValue, inputProof: res.inputProof };
  }

  /**
   * A batched encrypted input: several values sharing one input proof, which the singular helpers
   * cannot express. Backed by `@fhevm/sdk`'s `encryptValues`.
   */
  public createEncryptedInput(contractAddress: string, userAddress: string): FhevmEncryptedInput {
    return createEncryptedInput(() => this.__client, contractAddress, userAddress);
  }

  public async encryptUint(
    fhevmType: FhevmTypeEuint,
    value: number | bigint,
    contractAddress: string,
    userAddress: string,
  ): Promise<{ externalEuint: string; inputProof: string }> {
    if (!isFhevmEuint(fhevmType)) {
      throw new HardhatFhevmError(`encryptUint: '${String(fhevmType)}' is not a valid FhevmTypeEuint.`);
    }
    const { handle, inputProof } = await this.__encrypt(fhevmType, value, contractAddress, userAddress);
    return { externalEuint: handle, inputProof };
  }

  public async encryptBool(
    value: boolean,
    contractAddress: string,
    userAddress: string,
  ): Promise<{ externalEbool: string; inputProof: string }> {
    const { handle, inputProof } = await this.__encrypt(FhevmType.ebool, value, contractAddress, userAddress);
    return { externalEbool: handle, inputProof };
  }

  public async encryptAddress(
    value: string,
    contractAddress: string,
    userAddress: string,
  ): Promise<{ externalEaddress: string; inputProof: string }> {
    const { handle, inputProof } = await this.__encrypt(FhevmType.eaddress, value, contractAddress, userAddress);
    return { externalEaddress: handle, inputProof };
  }

  //////////////////////////////////////////////////////////////////////////////
  // Public decryption
  //////////////////////////////////////////////////////////////////////////////

  public async publicDecrypt(handles: Array<string | Uint8Array>): Promise<PublicDecryptResults> {
    if (this.fhevmEnv.isRunningInHHNode) {
      throw new HardhatFhevmError(`Cannot call publicDecrypt from a 'hardhat node' server.`);
    }

    /*
      `decryptPublicValuesWithSignatures` rather than `decryptPublicValues`: callers need the KMS
      signatures too, so a contract can verify the decryption on-chain
      (`contract.verify(handles, abiEncodedClearValues, decryptionProof)`).
    */
    handles.forEach((h) => {
      assertHandleIsInitialized(__handleKey(h));
    });

    const res = await this.__client.decryptPublicValuesWithSignatures({ encryptedValues: handles });

    // The legacy shape is keyed by handle; the SDK returns values positionally.
    const clearValues: Record<string, bigint | boolean | string> = {};
    handles.forEach((handle, i) => {
      clearValues[__handleKey(handle)] = __typedValue(res.clearValues[i]);
    });

    return {
      clearValues,
      abiEncodedClearValues: res.checkSignaturesArgs.abiEncodedCleartexts,
      decryptionProof: res.checkSignaturesArgs.decryptionProof,
    };
  }

  private async __publicDecryptOne(handleBytes32: string, method: string): Promise<TypedValueLike> {
    if (this.fhevmEnv.isRunningInHHNode) {
      throw new HardhatFhevmError(`Cannot call ${method} from a 'hardhat node' server.`);
    }
    assertHandleIsInitialized(handleBytes32);
    const value = await this.__client.decryptPublicValue({ encryptedValue: handleBytes32 });
    if ((value as unknown) === undefined) {
      throw new HardhatFhevmError(`Failed to publicly decrypt handle '${handleBytes32}'.`);
    }
    return value;
  }

  public async publicDecryptEbool(handleBytes32: string, _options?: FhevmPublicDecryptOptions): Promise<boolean> {
    return __asBoolean(await this.__publicDecryptOne(handleBytes32, 'publicDecryptEbool'), handleBytes32);
  }

  public async publicDecryptEuint(
    fhevmType: FhevmTypeEuint,
    handleBytes32: string,
    _options?: FhevmPublicDecryptOptions,
  ): Promise<bigint> {
    void fhevmType;
    return __asBigInt(await this.__publicDecryptOne(handleBytes32, 'publicDecryptEuint'), handleBytes32);
  }

  public async publicDecryptEaddress(handleBytes32: string, _options?: FhevmPublicDecryptOptions): Promise<string> {
    return __asAddress(await this.__publicDecryptOne(handleBytes32, 'publicDecryptEaddress'), handleBytes32);
  }

  //////////////////////////////////////////////////////////////////////////////
  // User decryption
  //////////////////////////////////////////////////////////////////////////////

  /**
   * One user decryption, end to end.
   *
   * The relayer-sdk needed the caller to `generateKeypair()`, build an EIP-712 payload with
   * `createEIP712(...)`, sign it, and pass all four pieces back into `userDecrypt(...)`. `@fhevm/sdk`
   * replaces that with a transport key pair plus a signed decryption permit — done here so the
   * ergonomic `userDecryptE*` surface keeps its old shape and the tests keep theirs.
   */
  private async __userDecryptOne(
    handleBytes32: string,
    contractAddress: EthersT.AddressLike,
    user: EthersT.Signer,
    options: FhevmUserDecryptOptions | undefined,
    method: string,
  ): Promise<TypedValueLike> {
    if (this.fhevmEnv.isRunningInHHNode) {
      throw new HardhatFhevmError(`Cannot call ${method} from a 'hardhat node' server.`);
    }
    assertHandleIsInitialized(handleBytes32);

    const client = this.__client;
    const resolvedContractAddress = await EthersT.resolveAddress(contractAddress);
    const signerAddress = await user.getAddress();

    const transportKeyPair = await client.generateTransportKeyPair();

    const startTimestamp = Number(options?.validity?.startTimestamp ?? Math.floor(Date.now() / 1000));
    const durationDays = Number(options?.validity?.durationDays ?? DEFAULT_DURATION_DAYS);

    const signedPermit = await client.signLegacyDecryptionPermit({
      contractAddresses: [resolvedContractAddress],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress,
      signer: user,
      transportKeyPair,
    });

    const values = await client.decryptValues({
      encryptedValues: [handleBytes32],
      contractAddress: resolvedContractAddress,
      transportKeyPair,
      signedPermit,
    });

    const value = values[0];
    if (value === undefined) {
      throw new HardhatFhevmError(`Failed to decrypt handle '${handleBytes32}' for user ${signerAddress}.`);
    }
    return value;
  }

  public async userDecryptEbool(
    handleBytes32: string,
    contractAddress: EthersT.AddressLike,
    user: EthersT.Signer,
    options?: FhevmUserDecryptOptions,
  ): Promise<boolean> {
    return __asBoolean(
      await this.__userDecryptOne(handleBytes32, contractAddress, user, options, 'userDecryptEbool'),
      handleBytes32,
    );
  }

  public async userDecryptEuint(
    fhevmType: FhevmTypeEuint,
    handleBytes32: string,
    contractAddress: EthersT.AddressLike,
    user: EthersT.Signer,
    options?: FhevmUserDecryptOptions,
  ): Promise<bigint> {
    void fhevmType;
    return __asBigInt(
      await this.__userDecryptOne(handleBytes32, contractAddress, user, options, 'userDecryptEuint'),
      handleBytes32,
    );
  }

  public async userDecryptEaddress(
    handleBytes32: string,
    contractAddress: EthersT.AddressLike,
    user: EthersT.Signer,
    options?: FhevmUserDecryptOptions,
  ): Promise<string> {
    return __asAddress(
      await this.__userDecryptOne(handleBytes32, contractAddress, user, options, 'userDecryptEaddress'),
      handleBytes32,
    );
  }

  //////////////////////////////////////////////////////////////////////////////
  // Deprecated — relayer-sdk surface. Not to be implemented; scheduled for removal.
  //////////////////////////////////////////////////////////////////////////////

  /** @deprecated Replaced by `@fhevm/sdk` decryption permits. */
  public createEIP712(
    _publicKey: string,
    _contractAddresses: string[],
    _startTimestamp: string | number,
    _durationDays: string | number,
  ): KmsUserDecryptEIP712Type {
    return this.__deprecated('createEIP712', 'the decryption-permit API (fhevm.userDecryptE*)');
  }

  /** @deprecated Replaced by delegated decryption permits. */
  public createDelegatedUserDecryptEIP712(
    _publicKey: string,
    _contractAddresses: string[],
    _delegatorAddress: string,
    _startTimestamp: number,
    _durationDays: number,
  ): KmsDelegatedUserDecryptEIP712Type {
    return this.__deprecated('createDelegatedUserDecryptEIP712', 'the delegated decryption-permit API');
  }

  /** @deprecated Replaced by `generateTransportKeyPair()`. */
  public generateKeypair(): { publicKey: string; privateKey: string } {
    return this.__deprecated('generateKeypair', 'the transport key pair generated by fhevm.userDecryptE*');
  }

  /** @deprecated Use `userDecryptE*`. */
  // eslint-disable-next-line @typescript-eslint/require-await
  public async userDecrypt(
    _handles: HandleContractPair[],
    _privateKey: string,
    _publicKey: string,
    _signature: string,
    _contractAddresses: string[],
    _userAddress: string,
    _startTimestamp: string | number,
    _durationDays: string | number,
  ): Promise<UserDecryptResults> {
    return this.__deprecated('userDecrypt', 'fhevm.userDecryptEbool()/userDecryptEuint()/userDecryptEaddress()');
  }

  /** @deprecated Use `userDecryptE*`. */
  // eslint-disable-next-line @typescript-eslint/require-await
  public async delegatedUserDecrypt(
    _handleContractPairs: HandleContractPair[],
    _privateKey: string,
    _publicKey: string,
    _signature: string,
    _contractAddresses: string[],
    _delegatorAddress: string,
    _delegateAddress: string,
    _startTimestamp: number,
    _durationDays: number,
  ): Promise<UserDecryptResults> {
    return this.__deprecated('delegatedUserDecrypt', 'fhevm.userDecryptE* with delegation options');
  }

  /** @deprecated Served by the JS mock engine, which no longer exists. */
  // eslint-disable-next-line @typescript-eslint/require-await
  public async getRelayerMetadata(): Promise<RelayerMetadata> {
    return this.__deprecated('getRelayerMetadata', 'the chain definition exposed by @fhevm/sdk');
  }
}
