import { assertOwnedBy } from "../runtime/CoreFhevmRuntime-p.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import type {
  GlobalFheCrs,
  GlobalFhePkeParams,
  GlobalFhePublicKey,
} from "../types/globalFhePkeParams.js";

const PRIVATE_TOKEN = Symbol("GlobalFhePkeParams.token");
const VERIFY_FUNC = Symbol("GlobalFhePkeParams.verify");

class GlobalFhePkeParamsImpl implements GlobalFhePkeParams {
  readonly #owner: WeakRef<FhevmRuntime>;
  readonly #crs: GlobalFheCrs;
  readonly #publicKey: GlobalFhePublicKey;

  constructor(
    privateToken: symbol,
    owner: WeakRef<FhevmRuntime>,
    parameters: {
      readonly publicKey: GlobalFhePublicKey;
      readonly crs: GlobalFheCrs;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error("Unauthorized");
    }
    this.#owner = owner;
    this.#publicKey = parameters.publicKey;
    this.#crs = parameters.crs;

    Object.freeze(this);
  }

  public get publicKey(): GlobalFhePublicKey {
    return this.#publicKey;
  }

  public get crs(): GlobalFheCrs {
    return this.#crs;
  }

  public static [VERIFY_FUNC](instance: unknown, owner: FhevmRuntime): void {
    if (!(instance instanceof GlobalFhePkeParamsImpl)) {
      throw new Error("Invalid GlobalFhePkeParams instance");
    }
    assertOwnedBy({
      actualOwner: instance.#owner,
      expectedOwner: owner,
      name: "GlobalFhePkeParams",
    });
  }
}

// Prevent prototype pollution and constructor access
Object.freeze(GlobalFhePkeParamsImpl.prototype);
Object.freeze(GlobalFhePkeParamsImpl);

////////////////////////////////////////////////////////////////////////////////

export function createGlobalFhePkeParams(
  owner: WeakRef<FhevmRuntime>,
  parameters: {
    readonly publicKey: GlobalFhePublicKey;
    readonly crs: GlobalFheCrs;
  },
): GlobalFhePkeParams {
  return new GlobalFhePkeParamsImpl(PRIVATE_TOKEN, owner, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Verifies that the given `GlobalFhePkeParams` instance is owned
 * by the given runtime. Throws if not.
 */
export function assertGlobalFhePkeParamsOwnedBy(
  data: GlobalFhePkeParams,
  owner: FhevmRuntime,
): void {
  GlobalFhePkeParamsImpl[VERIFY_FUNC](data, owner);
}
