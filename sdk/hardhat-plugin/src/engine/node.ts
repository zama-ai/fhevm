import { toBeHex, zeroPadValue } from "ethers";

/**
 * erc7201 storage locations (OpenZeppelin upgradeable). These are layout facts about the contracts we
 * poke, not configuration — they live here because this is the only place that writes them.
 */
const INITIALIZABLE_STORAGE_SLOT = "0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00";
const OWNABLE_STORAGE_SLOT = "0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300";

export interface Eip1193Provider {
  request(args: { method: string; params?: unknown[] }): Promise<unknown>;
}

/**
 * `hardhat node` is deliberately unsupported: it is the only mode where the engine would have to live
 * in a different process from the test, which is what the old plugin's fake-relayer JSON-RPC transport
 * existed to bridge. In-process `hardhat` and `anvil` both embed the engine.
 */
export type NodeKind = "hardhat" | "anvil";

interface NodeMethods {
  readonly setCode: string;
  readonly setBalance: string;
  readonly setStorageAt: string;
  readonly impersonateAccount: string;
}

const METHODS: Record<NodeKind, NodeMethods> = {
  hardhat: {
    setCode: "hardhat_setCode",
    setBalance: "hardhat_setBalance",
    setStorageAt: "hardhat_setStorageAt",
    impersonateAccount: "hardhat_impersonateAccount",
  },
  anvil: {
    setCode: "anvil_setCode",
    setBalance: "anvil_setBalance",
    setStorageAt: "anvil_setStorageAt",
    impersonateAccount: "anvil_impersonateAccount",
  },
};

/**
 * Probes node capabilities rather than matching network names. The old plugin keyed off
 * `network.name === "localhost"` (with an asserted port) and `=== "anvil"`, so renaming a network in
 * hardhat.config.ts silently selected the wrong RPC namespace.
 *
 * Anvil is probed first: it aliases much of the `hardhat_*` namespace for compatibility, so a
 * hardhat-first probe would misidentify it.
 */
export async function detectNodeKind(provider: Eip1193Provider): Promise<NodeKind> {
  try {
    await provider.request({ method: "anvil_nodeInfo", params: [] });
    return "anvil";
  } catch {
    /* not anvil */
  }
  try {
    await provider.request({ method: "hardhat_metadata", params: [] });
    return "hardhat";
  } catch {
    /* not hardhat */
  }
  throw new Error(
    "Unsupported network: neither `anvil_nodeInfo` nor `hardhat_metadata` is available. " +
      "The FHEVM cleartext mock requires an in-process Hardhat network or an anvil node.",
  );
}

export class FhevmNode {
  private constructor(
    public readonly provider: Eip1193Provider,
    public readonly kind: NodeKind,
    public readonly chainId: number,
    private readonly methods: NodeMethods,
  ) {}

  static async create(provider: Eip1193Provider): Promise<FhevmNode> {
    const kind = await detectNodeKind(provider);
    const chainIdHex = (await provider.request({ method: "eth_chainId", params: [] })) as string;
    return new FhevmNode(provider, kind, Number(BigInt(chainIdHex)), METHODS[kind]);
  }

  /** `eth_call` against `to` with ABI-encoded `data`; returns the raw return bytes. */
  async call(to: string, data: string): Promise<string> {
    return (await this.provider.request({ method: "eth_call", params: [{ to, data }, "latest"] })) as string;
  }

  /**
   * Sends a transaction from an already-impersonated account. No signing library is involved: the node
   * signs, because `from` is impersonated. Throws on a reverted transaction — a raw `eth_sendTransaction`
   * would otherwise mine a failed tx (status 0x0) without raising, so the receipt status is checked.
   */
  async sendTransaction(tx: { from: string; to: string; data: string }): Promise<void> {
    const hash = (await this.provider.request({ method: "eth_sendTransaction", params: [tx] })) as string;
    const receipt = await this.waitForReceipt(hash);
    if (receipt === null) {
      throw new Error(`No receipt for tx ${hash} (to=${tx.to}).`);
    }
    const status = (receipt as { status?: string }).status;
    if (status !== "0x1") {
      throw new Error(`Transaction ${hash} reverted (status ${status}, to=${tx.to}).`);
    }
  }

  private async waitForReceipt(hash: string): Promise<unknown> {
    // Both target networks automine, so the receipt is normally ready on the first poll.
    for (let i = 0; i < 50; i++) {
      const receipt = await this.provider.request({ method: "eth_getTransactionReceipt", params: [hash] });
      if (receipt) {
        return receipt;
      }
      await new Promise((resolve) => setTimeout(resolve, 20));
    }
    return null;
  }

  /**
   * Writes runtime code directly. Note this bypasses EIP-170 entirely — the code-size limit applies to
   * CREATE, not to a state write — which is why the 27,758-byte cleartext executor needs neither
   * `allowUnlimitedContractSize` nor `--disable-code-size-limit` here.
   */
  async setCode(address: string, code: string): Promise<void> {
    await this.provider.request({ method: this.methods.setCode, params: [address, code] });
  }

  async getCode(address: string): Promise<string> {
    return (await this.provider.request({ method: "eth_getCode", params: [address, "latest"] })) as string;
  }

  async setBalance(address: string, wei: bigint): Promise<void> {
    await this.provider.request({ method: this.methods.setBalance, params: [address, toBeHex(wei)] });
  }

  async setStorageAt(address: string, slot: string, valueBytes32: string): Promise<void> {
    await this.provider.request({
      method: this.methods.setStorageAt,
      params: [address, toBeHex(BigInt(slot), 32), valueBytes32],
    });
  }

  async impersonate(address: string): Promise<void> {
    await this.provider.request({ method: this.methods.impersonateAccount, params: [address] });
  }

  /**
   * Fakes the post-`EmptyUUPSProxy.initialize()` state. The mock places implementations directly with
   * setCode instead of standing up real ERC1967 proxies, so `initializeFromEmptyProxy` — a
   * `reinitializer(2)` — would revert on its `_initialized < 2` guard without this.
   *
   * Layout (OZ Initializable, one slot, packed from the LSB):
   *   bytes 24..31 = uint64 _initialized
   *   byte  23     = bool   _initializing
   */
  async setInitializableStorage(address: string, initialized: bigint, initializing: boolean): Promise<void> {
    const initializedBytes = toBeHex(initialized, 8);
    const initializingByte = initializing ? "0x01" : "0x00";
    const packed = initializingByte + initializedBytes.slice(2);
    await this.setStorageAt(address, INITIALIZABLE_STORAGE_SLOT, zeroPadValue(packed, 32));
  }

  /** Writes OwnableUpgradeable's `_owner` directly (the mock never runs ACL's initializer). */
  async setOwnableStorage(address: string, owner: string): Promise<void> {
    await this.setStorageAt(address, OWNABLE_STORAGE_SLOT, zeroPadValue(owner, 32));
  }
}
