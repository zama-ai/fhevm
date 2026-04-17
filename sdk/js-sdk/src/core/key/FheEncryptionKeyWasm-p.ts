import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type {
  FheEncryptionCrs,
  FheEncryptionKeyMetadata,
  FheEncryptionKeyWasm,
  FheEncryptionPublicKey,
} from '../types/fheEncryptionKey.js';
import { assertOwnedBy } from '../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('FheEncryptionKeyWasm.token');
const VERIFY_FUNC = Symbol('FheEncryptionKeyWasm.verify');

////////////////////////////////////////////////////////////////////////////////
// FheEncryptionKeyWasmImpl (private implementation)
////////////////////////////////////////////////////////////////////////////////

class FheEncryptionKeyWasmImpl implements FheEncryptionKeyWasm {
  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #crs: FheEncryptionCrs;
  readonly #publicKey: FheEncryptionPublicKey;
  readonly #metadata: FheEncryptionKeyMetadata;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly publicKey: FheEncryptionPublicKey;
      readonly crs: FheEncryptionCrs;
      readonly metadata: FheEncryptionKeyMetadata;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#owner = owner;
    this.#publicKey = parameters.publicKey;
    this.#crs = parameters.crs;
    this.#metadata = Object.freeze({ ...parameters.metadata });

    Object.freeze(this);
  }

  public get publicKey(): FheEncryptionPublicKey {
    return this.#publicKey;
  }

  public get crs(): FheEncryptionCrs {
    return this.#crs;
  }

  public get metadata(): FheEncryptionKeyMetadata {
    return this.#metadata;
  }

  public static [VERIFY_FUNC](privateToken: symbol, instance: unknown, owner: FhevmRuntime): void {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    if (!(instance instanceof FheEncryptionKeyWasmImpl)) {
      throw new Error('Invalid FheEncryptionKey instance');
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: 'FheEncryptionKey',
    });
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(FheEncryptionKeyWasmImpl.prototype);
Object.freeze(FheEncryptionKeyWasmImpl);

////////////////////////////////////////////////////////////////////////////////

export function createFheEncryptionKeyWasm(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly publicKey: FheEncryptionPublicKey;
    readonly crs: FheEncryptionCrs;
    readonly metadata: FheEncryptionKeyMetadata;
  },
): FheEncryptionKeyWasm {
  return new FheEncryptionKeyWasmImpl(PRIVATE_TOKEN, owner, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `FheEncryptionKeyWasm` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertFheEncryptionKeyWasmOwnedBy(data: FheEncryptionKeyWasm, owner: FhevmRuntime): void {
  FheEncryptionKeyWasmImpl[VERIFY_FUNC](PRIVATE_TOKEN, data, owner);
}
