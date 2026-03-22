import type {
  ChecksummedAddress,
  Uint64BigInt,
  Uint8Number,
} from "../types/primitives.js";
import type { KmsEIP712Domain, KmsVerifierContractData } from "../types/kms.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import { assertOwnedBy } from "../runtime/CoreFhevmRuntime-p.js";

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol("KmsVerifierContractData.token");
const VERIFY_FUNC = Symbol("KmsVerifierContractData.verify");

////////////////////////////////////////////////////////////////////////////////
// KmsVerifierContractData (private implementation)
////////////////////////////////////////////////////////////////////////////////

class KmsVerifierContractDataImpl implements KmsVerifierContractData {
  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #address: ChecksummedAddress;
  readonly #eip712Domain: KmsEIP712Domain;
  readonly #kmsSigners: ChecksummedAddress[];
  readonly #kmsSignersSet: Set<string>;
  readonly #kmsSignerThreshold: Uint8Number;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      address: ChecksummedAddress;
      eip712Domain: KmsEIP712Domain;
      kmsSigners: ChecksummedAddress[];
      kmsSignerThreshold: Uint8Number;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error("Unauthorized");
    }
    this.#owner = owner;
    this.#address = parameters.address;
    this.#eip712Domain = { ...parameters.eip712Domain };
    this.#kmsSigners = [...parameters.kmsSigners];
    this.#kmsSignerThreshold = parameters.kmsSignerThreshold;
    this.#kmsSignersSet = new Set(
      this.#kmsSigners.map((addr) => addr.toLowerCase()),
    );

    Object.freeze(this.#eip712Domain);
    Object.freeze(this.#kmsSigners);
    Object.freeze(this);
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

  public get kmsSigners(): ChecksummedAddress[] {
    return this.#kmsSigners;
  }

  public get kmsSignerThreshold(): Uint8Number {
    return this.#kmsSignerThreshold;
  }

  public get verifyingContractAddressDecryption(): ChecksummedAddress {
    return this.#eip712Domain.verifyingContract;
  }

  public has(signer: string): boolean {
    return this.#kmsSignersSet.has(signer);
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof KmsVerifierContractDataImpl)) {
      throw new Error("Invalid KmsVerifierContractData instance");
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: "KmsVerifierContractData",
    });
  }

  public toJson(): Record<string, unknown> {
    return {
      address: this.#address,
      eip712Domain: this.#eip712Domain,
      kmsSigners: this.#kmsSigners,
      kmsSignerThreshold: this.#kmsSignerThreshold,
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
    readonly address: ChecksummedAddress;
    readonly eip712Domain: KmsEIP712Domain;
    readonly kmsSigners: ChecksummedAddress[];
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
