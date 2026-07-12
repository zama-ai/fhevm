#!/usr/bin/env python3
"""
Generate the Hardhat-shaped cleartext artifact bundle from the Foundry `out/` build, for consumers
that deploy the cleartext contracts from TypeScript (the @fhevm/hardhat-plugin).

Run from the package root after `forge build`:
    forge build && python3 scripts/gen-hardhat-artifacts.py

Writes {contractName, abi, bytecode, deployedBytecode} JSON per contract to `artifacts-hardhat/`,
which the package exposes to npm consumers as the `./artifacts/*` subpath export. Consumers
(e.g. @fhevm/hardhat-plugin) resolve these directly — there is no vendored copy to keep in sync.
"""
import json, os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "out")
DST = os.path.join(ROOT, "artifacts-hardhat")

# Cleartext split-set + immutable + empty proxies (as the plugin deploys them).
CONTRACTS = [
    "CleartextACL", "CleartextFHEVMExecutor", "CleartextInputVerifier", "CleartextKMSVerifier",
    "CleartextHCULimit", "CleartextProtocolConfig", "CleartextKMSGeneration",
    "PauserSet", "EmptyUUPSProxy", "EmptyUUPSProxyACL",
]


def read_artifact(name):
    src = os.path.join(OUT, f"{name}.sol", f"{name}.json")
    if not os.path.exists(src):
        sys.exit(f"missing forge artifact: {src} (did you run `forge build`?)")
    d = json.load(open(src))
    return {
        "contractName": name,
        "abi": d.get("abi", []),
        "bytecode": d.get("bytecode", {}).get("object", "0x"),
        "deployedBytecode": d.get("deployedBytecode", {}).get("object", "0x"),
    }


def main():
    os.makedirs(DST, exist_ok=True)
    for name in CONTRACTS:
        art = read_artifact(name)
        json.dump(art, open(os.path.join(DST, f"{name}.json"), "w"))
    print(f"wrote {len(CONTRACTS)} artifacts to artifacts-hardhat/")


if __name__ == "__main__":
    main()
