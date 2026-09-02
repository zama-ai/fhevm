import { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';
import constants from './constants';
import type { FhevmContractName } from './migration/placeholders';

/**
 * The FHEVM host contracts, by ABI.
 *
 * Replaces `@fhevm/mock-utils`' `FhevmContractsRepository`, and is deliberately far smaller. The old
 * one also carried the KMS/coprocessor signer sets, gateway addresses and EIP-712 domains, because
 * the JavaScript mock engine needed them to forge relayer responses. Nothing does any more: the
 * cleartext stack runs on-chain and `@fhevm/sdk` owns the signing. What is left is exactly one job —
 * decoding a revert into the custom error that produced it, which needs nothing but ABIs.
 *
 * ABIs come from `@fhevm/host-contracts-cleartext`'s `./abi/*.json` export, so they track the
 * deployed contracts by construction rather than being vendored here.
 */

// `require` rather than `import`: these are JSON in a CommonJS build, and loading them lazily keeps
// them out of the module graph for consumers that never decode an error.
// eslint-disable-next-line @typescript-eslint/naming-convention
function __loadAbi(file: string): EthersT.InterfaceAbi {
  try {
    return require(`@fhevm/host-contracts-cleartext/abi/${file}.json`) as EthersT.InterfaceAbi;
  } catch (e) {
    throw new HardhatFhevmError(
      `Unable to load the '${file}' ABI from ${constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE.name}. ${String(e)}`,
    );
  }
}

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

/**
 * The host-contract addresses the repository is built from.
 *
 * The four core contracts exist on every network the plugin supports, so they are required.
 * `HCULimit`, `ProtocolConfig`, `KMSGeneration` and `PauserSet` are optional on purpose: their
 * addresses are only known for the local stack. Passing `undefined` leaves the contract unregistered
 * rather than registering a local address against a public chain — which is what the previous version
 * did for `HCULimit`, putting a localhost address into the Sepolia lookup table.
 */
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

/**
 * The cleartext-only addresses. Both are required: `deploy()` returns them as a pair
 * (`CleartextAddresses`), so one never exists without the other.
 */
export type FhevmCleartextContractsAddresses = {
  readonly cleartextArithmeticAddress: string;
  readonly cleartextDbAddress: string;
};

export type FhevmContractWrapper = {
  readonly name: FhevmContractName;
  readonly address: string;
  readonly package: string;
  readonly interface: EthersT.Interface;
  readonly readonlyContract: EthersT.Contract;
  readonly properties: {
    contractName: FhevmContractName;
    address: string;
    contract: EthersT.Contract;
    package: string;
  };
};

// eslint-disable-next-line @typescript-eslint/naming-convention
function __wrap(name: FhevmContractName, address: string, provider: EthersT.Provider): FhevmContractWrapper {
  const abi = __loadAbi(ABI_FILE[name]);
  const contract = new EthersT.Contract(address, abi, provider);
  const pkg = constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE.name;
  return {
    name,
    address,
    package: pkg,
    interface: contract.interface,
    readonlyContract: contract,
    properties: { contractName: name, address, contract, package: pkg },
  };
}

/**
 * The real host contracts — the ones present in every FHEVM deployment.
 *
 * This is what a public network gets. It knows nothing about cleartext mode; for that, see
 * {@link FhevmCleartextContractsRepository}.
 */
export class FhevmContractsRepository {
  /** Always present: these four exist on every supported network. */
  readonly acl: FhevmContractWrapper;
  readonly fhevmExecutor: FhevmContractWrapper;
  readonly inputVerifier: FhevmContractWrapper;
  readonly kmsVerifier: FhevmContractWrapper;

  /** Present only when the address is known — see {@link FhevmHostContractsAddresses}. */
  readonly hcuLimit: FhevmContractWrapper | undefined;
  readonly protocolConfig: FhevmContractWrapper | undefined;
  readonly kmsGeneration: FhevmContractWrapper | undefined;
  readonly pauserSet: FhevmContractWrapper | undefined;

  readonly #provider: EthersT.Provider;
  readonly #byAddress: Record<string, FhevmContractWrapper>;
  readonly #byName: Map<FhevmContractName, FhevmContractWrapper>;

  constructor(provider: EthersT.Provider, addresses: FhevmHostContractsAddresses) {
    this.#provider = provider;
    this.#byAddress = {};
    this.#byName = new Map();

    this.acl = this._register('ACL', addresses.aclAddress);
    this.fhevmExecutor = this._register('FHEVMExecutor', addresses.fhevmExecutorAddress);
    this.inputVerifier = this._register('InputVerifier', addresses.inputVerifierAddress);
    this.kmsVerifier = this._register('KMSVerifier', addresses.kmsVerifierAddress);

    this.hcuLimit = this._registerOptional('HCULimit', addresses.hcuLimitAddress);
    this.protocolConfig = this._registerOptional('ProtocolConfig', addresses.protocolConfigAddress);
    this.kmsGeneration = this._registerOptional('KMSGeneration', addresses.kmsGenerationAddress);
    this.pauserSet = this._registerOptional('PauserSet', addresses.pauserSetAddress);
  }

  /**
   * Wraps a contract and adds it to both lookup tables.
   *
   * `protected` so {@link FhevmCleartextContractsRepository} can add its own contracts: the tables
   * themselves stay `#`-private, since a revert must resolve against a name/address that was actually
   * registered.
   */
  protected _register(name: FhevmContractName, address: string): FhevmContractWrapper {
    const w = __wrap(name, address, this.#provider);
    // Addresses are compared case-insensitively: a revert reports whatever casing the node used.
    this.#byAddress[w.address.toLowerCase()] = w;
    this.#byName.set(w.name, w);
    return w;
  }

  protected _registerOptional(name: FhevmContractName, address: string | undefined): FhevmContractWrapper | undefined {
    return address === undefined ? undefined : this._register(name, address);
  }

  public addressToContractMap(): Record<string, FhevmContractWrapper> {
    return { ...this.#byAddress };
  }

  public getContractFromAddress(address: string): FhevmContractWrapper | undefined {
    return this.#byAddress[address.toLowerCase()];
  }

  public getContractFromName(name: FhevmContractName): FhevmContractWrapper | undefined {
    return this.#byName.get(name);
  }
}

/**
 * The host contracts *plus* the two that only exist in cleartext mode.
 *
 * Separating this from {@link FhevmContractsRepository} is what makes "cleartext-only" structural
 * rather than a convention: `cleartextArithmetic` and `cleartextDb` are non-optional here — no `?.`
 * at the call sites that legitimately have them — and simply unreachable on the base class, which is
 * what a public network is given. Narrow with {@link isCleartextContractsRepository}.
 */
export class FhevmCleartextContractsRepository extends FhevmContractsRepository {
  /** Evaluates the operators on-chain — the cleartext stand-in for the coprocessor. */
  readonly cleartextArithmetic: FhevmContractWrapper;
  /** Stores every computed cleartext, keyed by handle. Backs `fhevm.debugger`. */
  readonly cleartextDb: FhevmContractWrapper;

  constructor(provider: EthersT.Provider, addresses: FhevmHostContractsAddresses & FhevmCleartextContractsAddresses) {
    super(provider, addresses);
    this.cleartextArithmetic = this._register('CleartextArithmetic', addresses.cleartextArithmeticAddress);
    this.cleartextDb = this._register('CleartextDB', addresses.cleartextDbAddress);
  }
}

/** Narrows a repository to the cleartext one, for the call sites that need `CleartextDB`. */
export function isCleartextContractsRepository(
  repository: FhevmContractsRepository,
): repository is FhevmCleartextContractsRepository {
  return repository instanceof FhevmCleartextContractsRepository;
}
