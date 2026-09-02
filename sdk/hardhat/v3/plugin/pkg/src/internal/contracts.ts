// The FHEVM host contracts, by ABI. One job: give the plugin the ABI (revert decoding) and a
// read-only viem contract per host contract, with lookups by name and by address. ABIs come from
// @fhevm/host-contracts-cleartext's `./abi/*.json` export, so they track the deployed contracts by
// construction. Addresses are the caller's: a public network supplies the four core ones, the local
// cleartext stack supplies all of them.

import { createRequire } from 'node:module';

import { HardhatPluginError } from 'hardhat/plugins';
import { type Abi, type Address, type GetContractReturnType, type PublicClient, getContract } from 'viem';

import { FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME, PLUGIN_ID } from './constants.js';

export type FhevmContractName =
  | 'ACL'
  | 'FHEVMExecutor'
  | 'InputVerifier'
  | 'KMSVerifier'
  | 'HCULimit'
  | 'ProtocolConfig'
  | 'KMSGeneration'
  | 'PauserSet'
  | 'CleartextArithmetic'
  | 'CleartextDB';

/** Maps a host contract to the ABI file that describes it in the cleartext package. */
const ABI_FILE: Readonly<Record<FhevmContractName, string>> = {
  ACL: 'ACL',
  FHEVMExecutor: 'CleartextFHEVMExecutor',
  InputVerifier: 'CleartextInputVerifier',
  KMSVerifier: 'CleartextKMSVerifier',
  HCULimit: 'HCULimit',
  ProtocolConfig: 'ProtocolConfig',
  KMSGeneration: 'KMSGeneration',
  CleartextArithmetic: 'CleartextArithmetic',
  CleartextDB: 'CleartextDB',
  PauserSet: 'PauserSet',
};

/** The four core contracts exist on every network; the rest is known for the local stack only. */
export type FhevmHostContractsAddresses = {
  readonly aclAddress: string;
  readonly fhevmExecutorAddress: string;
  readonly inputVerifierAddress: string;
  readonly kmsVerifierAddress: string;
  readonly hcuLimitAddress?: string | undefined;
  readonly protocolConfigAddress?: string | undefined;
  readonly kmsGenerationAddress?: string | undefined;
  readonly pauserSetAddress?: string | undefined;
};

/** The cleartext-only pair; `deploy()` returns them together, so one never exists without the other. */
export type FhevmCleartextContractsAddresses = {
  readonly cleartextArithmeticAddress: string;
  readonly cleartextDbAddress: string;
};

export type FhevmContractWrapper = {
  readonly name: FhevmContractName;
  readonly address: Address;
  readonly package: string;
  readonly abi: Abi;
  /** Read-only: bound to the public client, so `.read.<fn>()` works and nothing else is offered. */
  readonly contract: GetContractReturnType<Abi, PublicClient>;
};

// JSON through `require`, lazily: the ABIs stay out of the module graph until a contract is wrapped.
const require = createRequire(import.meta.url);

function loadAbi(name: FhevmContractName): Abi {
  const specifier = `${FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME}/abi/${ABI_FILE[name]}.json`;
  try {
    const abi: unknown = require(specifier);
    return abi as Abi;
  } catch (error) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Unable to load the '${name}' ABI from '${specifier}'.`,
      error instanceof Error ? error : undefined,
    );
  }
}

function wrap(name: FhevmContractName, address: string, client: PublicClient): FhevmContractWrapper {
  const abi = loadAbi(name);
  return {
    name,
    address: address as Address,
    package: FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME,
    abi,
    contract: getContract({ address: address as Address, abi, client }),
  };
}

/** The real host contracts — what a public network gets. Cleartext mode adds two more below. */
export class FhevmContractsRepository {
  readonly acl: FhevmContractWrapper;
  readonly fhevmExecutor: FhevmContractWrapper;
  readonly inputVerifier: FhevmContractWrapper;
  readonly kmsVerifier: FhevmContractWrapper;

  readonly hcuLimit: FhevmContractWrapper | undefined;
  readonly protocolConfig: FhevmContractWrapper | undefined;
  readonly kmsGeneration: FhevmContractWrapper | undefined;
  readonly pauserSet: FhevmContractWrapper | undefined;

  readonly #client: PublicClient;
  readonly #byAddress = new Map<string, FhevmContractWrapper>();
  readonly #byName = new Map<FhevmContractName, FhevmContractWrapper>();

  constructor(client: PublicClient, addresses: FhevmHostContractsAddresses) {
    this.#client = client;
    this.acl = this.register('ACL', addresses.aclAddress);
    this.fhevmExecutor = this.register('FHEVMExecutor', addresses.fhevmExecutorAddress);
    this.inputVerifier = this.register('InputVerifier', addresses.inputVerifierAddress);
    this.kmsVerifier = this.register('KMSVerifier', addresses.kmsVerifierAddress);
    this.hcuLimit = this.registerOptional('HCULimit', addresses.hcuLimitAddress);
    this.protocolConfig = this.registerOptional('ProtocolConfig', addresses.protocolConfigAddress);
    this.kmsGeneration = this.registerOptional('KMSGeneration', addresses.kmsGenerationAddress);
    this.pauserSet = this.registerOptional('PauserSet', addresses.pauserSetAddress);
  }

  // Addresses are keyed lower-case: a revert reports whatever casing the node used.
  protected register(name: FhevmContractName, address: string): FhevmContractWrapper {
    const wrapper = wrap(name, address, this.#client);
    this.#byAddress.set(address.toLowerCase(), wrapper);
    this.#byName.set(name, wrapper);
    return wrapper;
  }

  protected registerOptional(name: FhevmContractName, address: string | undefined): FhevmContractWrapper | undefined {
    return address === undefined ? undefined : this.register(name, address);
  }

  getContractFromAddress(address: string): FhevmContractWrapper | undefined {
    return this.#byAddress.get(address.toLowerCase());
  }

  getContractFromName(name: FhevmContractName): FhevmContractWrapper | undefined {
    return this.#byName.get(name);
  }

  addressToContractMap(): ReadonlyMap<string, FhevmContractWrapper> {
    return new Map(this.#byAddress);
  }
}

/** The host contracts plus the two that only exist in cleartext mode; narrow with the guard below. */
export class FhevmCleartextContractsRepository extends FhevmContractsRepository {
  /** Evaluates the operators on-chain — the cleartext stand-in for the coprocessor. */
  readonly cleartextArithmetic: FhevmContractWrapper;
  /** Stores every computed cleartext, keyed by handle. */
  readonly cleartextDb: FhevmContractWrapper;

  constructor(client: PublicClient, addresses: FhevmHostContractsAddresses & FhevmCleartextContractsAddresses) {
    super(client, addresses);
    this.cleartextArithmetic = this.register('CleartextArithmetic', addresses.cleartextArithmeticAddress);
    this.cleartextDb = this.register('CleartextDB', addresses.cleartextDbAddress);
  }
}

export function isCleartextContractsRepository(
  repository: FhevmContractsRepository,
): repository is FhevmCleartextContractsRepository {
  return repository instanceof FhevmCleartextContractsRepository;
}
