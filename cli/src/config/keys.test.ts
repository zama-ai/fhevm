import { describe, expect, it } from "bun:test";
import {
  checksumAddress,
  deriveAccount,
  deriveAllKeys,
  mnemonicToSeed,
  publicKeyToAddress,
} from "./keys";
import { DEFAULT_MNEMONIC } from "./model";
import { secp256k1 } from "@noble/curves/secp256k1";
import { hexToBytes } from "@noble/hashes/utils";

describe("keys", () => {
  it("derives known accounts from mnemonic", () => {
    const seed = mnemonicToSeed(DEFAULT_MNEMONIC);
    const deployer = deriveAccount(seed, 1);
    const pauser0 = deriveAccount(seed, 23);
    const txSender = deriveAccount(seed, 25);

    expect(deployer.privateKey).toBe("0x7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f");
    expect(pauser0.address).toBe("0x6591319B97979Acc59b7191A8B4Ec381375bFc92");
    expect(txSender.privateKey).toBe("0x55c7e64a1e153d667b2ce7489b62e0c6d5716fcb016ab6dd324decccf4d2cb2f");
  });

  it("builds complete key sets for topology", () => {
    const keys = deriveAllKeys(DEFAULT_MNEMONIC, 3, 1);
    const uniqueAddresses = new Set(keys.coprocessors.map((item) => item.txSender.address));

    expect(keys.coprocessors).toHaveLength(3);
    expect(uniqueAddresses.size).toBe(3);
    expect(keys.kmsNodes).toHaveLength(1);
    expect(keys.custodians).toHaveLength(3);
    expect(keys.pausers).toHaveLength(2);
  });

  it("checksums addresses using EIP-55 rules", () => {
    expect(checksumAddress("0x6591319b97979acc59b7191a8b4ec381375bfc92")).toBe(
      "0x6591319B97979Acc59b7191A8B4Ec381375bFc92",
    );
  });

  it("derives address from public key", () => {
    const seed = mnemonicToSeed(DEFAULT_MNEMONIC);
    const account = deriveAccount(seed, 1);
    const publicKey = secp256k1.getPublicKey(hexToBytes(account.privateKey.slice(2)), false);

    expect(publicKeyToAddress(publicKey)).toBe(account.address);
  });

  it("rejects invalid mnemonic", () => {
    expect(() => mnemonicToSeed("not a mnemonic")).toThrow("invalid mnemonic");
  });
});
