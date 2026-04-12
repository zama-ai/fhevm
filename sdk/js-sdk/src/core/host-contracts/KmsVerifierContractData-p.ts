import type {
  ChecksummedAddress,
  Uint64BigInt,
  Uint8Number,
} from '../types/primitives.js';
import type { KmsEIP712Domain, KmsVerifierContractData } from '../types/kms.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertOwnedBy } from '../runtime/CoreFhevmRuntime-p.js';
import type { HostContractVersion } from '../types/hostContract.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('KmsVerifierContractData.token');
const VERIFY_FUNC = Symbol('KmsVerifierContractData.verify');

////////////////////////////////////////////////////////////////////////////////
// KmsVerifierContractData (private implementation)
////////////////////////////////////////////////////////////////////////////////

class KmsVerifierContractDataImpl implements KmsVerifierContractData {
  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #version: HostContractVersion<'KMSVerifier'>;
  readonly #address: ChecksummedAddress;
  readonly #eip712Domain: KmsEIP712Domain;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly version: HostContractVersion<'KMSVerifier'>;
      readonly address: ChecksummedAddress;
      readonly eip712Domain: KmsEIP712Domain;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#owner = owner;
    this.#version = Object.freeze({ ...parameters.version });
    this.#address = parameters.address;
    this.#eip712Domain = { ...parameters.eip712Domain };

    Object.freeze(this.#eip712Domain);
    Object.freeze(this);
  }

  public get version(): HostContractVersion<'KMSVerifier'> {
    return this.#version;
  }

  public get address(): ChecksummedAddress {
    return this.#address;
  }

  public get eip712Domain(): KmsEIP712Domain {
    return this.#eip712Domain;
  }

  public get gatewayChainId(): Uint64BigInt {
    return this.#eip712Domain.chainId;
  }

  public get verifyingContractAddressDecryption(): ChecksummedAddress {
    return this.#eip712Domain.verifyingContract;
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof KmsVerifierContractDataImpl)) {
      throw new Error('Invalid KmsVerifierContractData instance');
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: 'KmsVerifierContractData',
    });
  }

  public toJSON(): Record<string, unknown> {
    return {
      version: this.#version,
      address: this.#address,
      eip712Domain: this.#eip712Domain,
    };
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(KmsVerifierContractDataImpl.prototype);
Object.freeze(KmsVerifierContractDataImpl);

////////////////////////////////////////////////////////////////////////////////

export function createKmsVerifierContractData(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly version: HostContractVersion<'KMSVerifier'>;
    readonly address: ChecksummedAddress;
    readonly eip712Domain: KmsEIP712Domain;
    readonly kmsSigners: readonly ChecksummedAddress[];
    readonly kmsSignerThreshold: Uint8Number;
  },
): KmsVerifierContractData {
  return new KmsVerifierContractDataImpl(PRIVATE_TOKEN, owner, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `KmsVerifierContractData` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertKmsVerifierContractDataOwnedBy(
  data: KmsVerifierContractData,
  owner: FhevmRuntime,
): void {
  KmsVerifierContractDataImpl[VERIFY_FUNC](data, owner);
}
