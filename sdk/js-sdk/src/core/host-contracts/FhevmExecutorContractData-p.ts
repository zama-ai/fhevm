import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmExecutorContractData } from '../types/coprocessor.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { HostContractVersion } from '../types/hostContract.js';
import { assertOwnedBy } from '../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('FhevmExecutorContractData.token');
const VERIFY_FUNC = Symbol('FhevmExecutorContractData.verify');

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor (private implementation)
////////////////////////////////////////////////////////////////////////////////

class FhevmExecutorContractDataImpl implements FhevmExecutorContractData {
  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #version: HostContractVersion<'FHEVMExecutor'>;
  readonly #address: ChecksummedAddress;
  readonly #handleVersion: Uint8Number;
  readonly #aclContractAddress: ChecksummedAddress;
  readonly #inputVerifierContractAddress: ChecksummedAddress;
  readonly #hcuLimitContractAddress: ChecksummedAddress;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly version: HostContractVersion<'FHEVMExecutor'>;
      readonly address: ChecksummedAddress;
      readonly handleVersion: Uint8Number;
      readonly aclContractAddress: ChecksummedAddress;
      readonly inputVerifierContractAddress: ChecksummedAddress;
      readonly hcuLimitContractAddress: ChecksummedAddress;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#version = Object.freeze({ ...parameters.version });
    this.#owner = owner;
    this.#address = parameters.address;
    this.#handleVersion = parameters.handleVersion;
    this.#aclContractAddress = parameters.aclContractAddress;
    this.#inputVerifierContractAddress = parameters.inputVerifierContractAddress;
    this.#hcuLimitContractAddress = parameters.hcuLimitContractAddress;
    Object.freeze(this);
  }

  public get version(): HostContractVersion<'FHEVMExecutor'> {
    return this.#version;
  }

  public get address(): ChecksummedAddress {
    return this.#address;
  }

  public get aclContractAddress(): ChecksummedAddress {
    return this.#aclContractAddress;
  }

  public get inputVerifierContractAddress(): ChecksummedAddress {
    return this.#inputVerifierContractAddress;
  }

  public get hcuLimitContractAddress(): ChecksummedAddress {
    return this.#hcuLimitContractAddress;
  }

  public get handleVersion(): Uint8Number {
    return this.#handleVersion;
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof FhevmExecutorContractDataImpl)) {
      throw new Error('Invalid FhevmExecutorContractData instance');
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: 'FhevmExecutorContractData',
    });
  }

  public toJSON(): Record<string, unknown> {
    return {
      version: this.#version,
      address: this.#address,
      aclContractAddress: this.#aclContractAddress,
      inputVerifierContractAddress: this.#inputVerifierContractAddress,
      hcuLimitContractAddress: this.#hcuLimitContractAddress,
      handleVersion: this.#handleVersion,
    };
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(FhevmExecutorContractDataImpl.prototype);
Object.freeze(FhevmExecutorContractDataImpl);

////////////////////////////////////////////////////////////////////////////////

export function createFhevmExecutorContractData(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly version: HostContractVersion<'FHEVMExecutor'>;
    readonly address: ChecksummedAddress;
    readonly aclContractAddress: ChecksummedAddress;
    readonly inputVerifierContractAddress: ChecksummedAddress;
    readonly hcuLimitContractAddress: ChecksummedAddress;
    readonly handleVersion: Uint8Number;
  },
): FhevmExecutorContractData {
  const { version, address, aclContractAddress, inputVerifierContractAddress, hcuLimitContractAddress, handleVersion } =
    parameters;

  return new FhevmExecutorContractDataImpl(PRIVATE_TOKEN, owner, {
    version,
    address,
    aclContractAddress,
    inputVerifierContractAddress,
    hcuLimitContractAddress,
    handleVersion,
  });
}

/**
 * Verifies that the given `FHEVMExecutorContractData` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertFHEVMExecutorContractDataOwnedBy(data: FhevmExecutorContractData, owner: FhevmRuntime): void {
  FhevmExecutorContractDataImpl[VERIFY_FUNC](data, owner);
}
