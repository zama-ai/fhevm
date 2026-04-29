import type { ChecksummedAddress, Uint64BigInt, Uint8Number } from '../types/primitives.js';
import type { CoprocessorEip712Domain, InputVerifierContractData } from '../types/coprocessor.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { HostContractVersion } from '../types/hostContract.js';
import { assertOwnedBy } from '../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('InputVerifierContractData.token');
const VERIFY_FUNC = Symbol('InputVerifierContractData.verify');

////////////////////////////////////////////////////////////////////////////////
// InputVerifier (private implementation)
////////////////////////////////////////////////////////////////////////////////

class InputVerifierContractDataImpl implements InputVerifierContractData {
  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #version: HostContractVersion<'InputVerifier'>;
  readonly #address: ChecksummedAddress;
  readonly #eip712Domain: CoprocessorEip712Domain;
  readonly #coprocessorSigners: ChecksummedAddress[];
  readonly #coprocessorSignerThreshold: Uint8Number;
  readonly #coprocessorSignersSet: Set<string>;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly version: HostContractVersion<'InputVerifier'>;
      readonly address: ChecksummedAddress;
      readonly eip712Domain: CoprocessorEip712Domain;
      readonly coprocessorSigners: readonly ChecksummedAddress[];
      readonly coprocessorSignerThreshold: Uint8Number;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#owner = owner;
    this.#version = Object.freeze({ ...parameters.version });
    this.#address = parameters.address;
    this.#eip712Domain = { ...parameters.eip712Domain };
    this.#coprocessorSigners = [...parameters.coprocessorSigners];
    this.#coprocessorSignerThreshold = parameters.coprocessorSignerThreshold;
    this.#coprocessorSignersSet = new Set(this.#coprocessorSigners.map((addr) => addr.toLowerCase()));

    Object.freeze(this.#eip712Domain);
    Object.freeze(this.#coprocessorSigners);
    Object.freeze(this);
  }

  public get version(): HostContractVersion<'InputVerifier'> {
    return this.#version;
  }

  public get address(): ChecksummedAddress {
    return this.#address;
  }

  public get eip712Domain(): CoprocessorEip712Domain {
    return this.#eip712Domain;
  }

  public get gatewayChainId(): Uint64BigInt {
    return this.#eip712Domain.chainId;
  }

  public get coprocessorSigners(): ChecksummedAddress[] {
    return this.#coprocessorSigners;
  }

  public get coprocessorSignerThreshold(): Uint8Number {
    return this.#coprocessorSignerThreshold;
  }

  public get verifyingContractAddressInputVerification(): ChecksummedAddress {
    return this.#eip712Domain.verifyingContract;
  }

  public has(signer: string): boolean {
    return this.#coprocessorSignersSet.has(signer);
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof InputVerifierContractDataImpl)) {
      throw new Error('Invalid InputVerifierContractData instance');
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: 'InputVerifierContractData',
    });
  }

  public toJSON(): Record<string, unknown> {
    return {
      version: this.#version,
      address: this.#address,
      eip712Domain: this.#eip712Domain,
      coprocessorSigners: this.#coprocessorSigners,
      coprocessorSignerThreshold: this.#coprocessorSignerThreshold,
    };
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(InputVerifierContractDataImpl.prototype);
Object.freeze(InputVerifierContractDataImpl);

////////////////////////////////////////////////////////////////////////////////

export function createInputVerifierContractData(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly version: HostContractVersion<'InputVerifier'>;
    readonly address: ChecksummedAddress;
    readonly eip712Domain: CoprocessorEip712Domain;
    readonly coprocessorSigners: readonly ChecksummedAddress[];
    readonly coprocessorSignerThreshold: Uint8Number;
  },
): InputVerifierContractData {
  return new InputVerifierContractDataImpl(PRIVATE_TOKEN, owner, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `InputVerifierContractData` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertInputVerifierContractDataOwnedBy(data: InputVerifierContractData, owner: FhevmRuntime): void {
  InputVerifierContractDataImpl[VERIFY_FUNC](data, owner);
}
