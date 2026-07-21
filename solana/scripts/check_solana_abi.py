#!/usr/bin/env python3
"""Check Solana Anchor IDLs, schema hashes, and Borsh golden fixtures.

The host-listener and KMS connector intentionally mirror Solana account and
event layouts instead of linking against the on-chain crates. This script pins
the mirrored contract surface to the generated Anchor IDLs and makes field
reorders visible even when serialized sizes do not change.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import pathlib
import re
import shutil
import sys
from typing import Any


PROGRAMS = {
    "zama_host": {
        "target_idl": "target/idl/zama_host.json",
        "vendored_idl": "../coprocessor/fhevm-engine/host-listener/idl/zama_host.json",
        "constants": "programs/zama-host/src/constants.rs",
        "event_version_consts": {
            "zama_host": "EVENT_VERSION",
            "zama_host_public_outputs_produced": "PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION",
        },
    },
    "confidential_token": {
        "target_idl": "target/idl/confidential_token.json",
        "vendored_idl": "../coprocessor/fhevm-engine/host-listener/idl/confidential_token.json",
        "constants": "programs/confidential-token/src/constants.rs",
        "event_version_consts": {
            "confidential_token": "APP_EVENT_VERSION",
        },
    },
}

PINNED_SCHEMAS = [
    ("zama_host", "account", "HostConfig", True),
    ("zama_host", "account", "KmsContext", True),
    ("zama_host", "type", "InitializeHostConfigArgs", True),
    ("zama_host", "type", "FheEvalArgs", True),
    ("zama_host", "event", "PublicOutputsProducedEvent", True),
    ("zama_host", "type", "EncryptedValueSubjectGrant", True),
    ("zama_host", "instruction_args", "initialize_host_config", True),
    ("zama_host", "instruction_args", "fhe_eval", True),
    # EncryptedValue itself is intentionally not an Anchor `Account<'info, T>`
    # (see solana/programs/zama-host/src/instructions/encrypted_value.rs) —
    # every instruction takes it as `UncheckedAccount` and hand-rolls the
    # discriminator+borsh codec via `zama_solana_acl`, so Anchor's IDL builder
    # never registers it as an `account`/`type` entry. Its wire layout is
    # instead pinned by `zama-host`'s own
    # `state::encrypted_value::tests::discriminator_matches_shared_crate` and
    # `zama-solana-acl`'s codec tests, not by this golden file.
    ("zama_host", "instruction_args", "create_encrypted_value", True),
    ("zama_host", "instruction_args", "allow_subjects", True),
    ("zama_host", "instruction_args", "remove_subject", True),
    ("zama_host", "instruction_args", "update_encrypted_value", True),
    ("zama_host", "instruction_args", "make_handle_public", True),
    ("zama_host", "instruction_args", "define_kms_context", True),
    ("zama_host", "instruction_args", "delegate_for_user_decryption", True),
    ("zama_host", "instruction_args", "destroy_kms_context", True),
    ("zama_host", "instruction_args", "revoke_delegation_for_user_decryption", True),
    ("zama_host", "instruction_args", "set_coprocessor_signers", True),
    ("zama_host", "instruction_args", "set_deny_subject", True),
    ("zama_host", "instruction_args", "set_grant_deny_list_enabled", True),
    ("zama_host", "instruction_args", "set_hcu_app_trusted", True),
    ("zama_host", "instruction_args", "set_hcu_block_cap_per_app", True),
    ("zama_host", "instruction_args", "set_host_pause", True),
    ("zama_host", "instruction_args", "set_max_hcu_depth_per_tx", True),
    ("zama_host", "instruction_args", "set_max_hcu_per_tx", True),
    ("zama_host", "instruction_args", "verify_public_decrypt", True),
    ("confidential_token", "account", "ConfidentialMint", True),
    ("confidential_token", "account", "ConfidentialTokenAccount", True),
    # The DisclosureRequest account and its request/disclose/close instruction lifecycle were
    # dissolved (fhevm-internal#1704), and the BurnRedemptionRequest witness lifecycle
    # (request_burn_redemption + both close_* instructions) was dissolved onto the stateless host
    # verifier (fhevm-internal#1763). Token disclosure is the thin `disclose_secp` consumer and
    # redemption is the thin `redeem_burned_amount` consumer of `verify_public_decrypt`. Only the
    # permanent per-handle `BurnRedemption` replay marker remains as durable token state.
    ("confidential_token", "account", "BurnRedemption", True),
    ("confidential_token", "instruction_args", "confidential_burn", True),
    ("confidential_token", "instruction_args", "confidential_burn_from_value", True),
    ("confidential_token", "instruction_args", "confidential_transfer", True),
    ("confidential_token", "instruction_args", "confidential_transfer_from_value", True),
    # create_random_amount / create_random_bounded_amount are `poc`-gated demo helpers and are
    # intentionally absent from the production IDL, so they are not pinned here.
    ("confidential_token", "instruction_args", "disclose_secp", True),
    ("confidential_token", "instruction_args", "initialize_mint", True),
    ("confidential_token", "instruction_args", "initialize_token_account", True),
    ("confidential_token", "instruction_args", "redeem_burned_amount", True),
    ("confidential_token", "instruction_args", "wrap_usdc", True),
]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True, help="Solana workspace root")
    parser.add_argument(
        "--write",
        action="store_true",
        help="rewrite vendored IDLs and the ABI golden manifest",
    )
    args = parser.parse_args()

    root = pathlib.Path(args.root).resolve()
    manifest_path = (
        root
        / "../coprocessor/fhevm-engine/host-listener/idl/solana_abi_golden.json"
    ).resolve()

    if args.write:
        for spec in PROGRAMS.values():
            shutil.copyfile(
                root / spec["target_idl"],
                (root / spec["vendored_idl"]).resolve(),
            )
        current = build_manifest(root)
        pending_errors = pending_schema_errors(current)
        if pending_errors:
            for error in pending_errors:
                print(f"error: {error}", file=sys.stderr)
            return 1
        write_json(manifest_path, current)
        print(f"wrote {manifest_path}")
        return 0

    errors: list[str] = []
    for program, spec in PROGRAMS.items():
        target = root / spec["target_idl"]
        vendored = (root / spec["vendored_idl"]).resolve()
        if not vendored.exists():
            errors.append(f"{program}: missing vendored IDL {vendored}")
            continue
        if load_json(target) != load_json(vendored):
            errors.append(
                f"{program}: vendored IDL is out of sync with {target}; "
                "run solana/scripts/sync-zama-host-idl.sh"
            )

    if not manifest_path.exists():
        errors.append(f"missing ABI golden manifest {manifest_path}")
    else:
        expected = load_json(manifest_path)
        current = build_manifest(root)
        errors.extend(pending_schema_errors(current))
        if expected != current:
            errors.extend(manifest_diff(expected, current))

    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print("Solana IDLs, schema hashes, and ABI golden fixtures are in sync")
    return 0


def pending_schema_errors(manifest: dict[str, Any]) -> list[str]:
    errors = []
    for item in manifest.get("pending_schemas", []):
        errors.append(
            "pending Solana ABI schema remains for "
            f"{item.get('program')}/{item.get('category')}/{item.get('name')}: "
            f"{item.get('reason')}"
        )
    return errors


def build_manifest(root: pathlib.Path) -> dict[str, Any]:
    idls = {
        program: load_json((root / spec["vendored_idl"]).resolve())
        for program, spec in PROGRAMS.items()
    }
    schemas: list[dict[str, Any]] = []
    pending: list[dict[str, str]] = []

    requested = list(PINNED_SCHEMAS)
    for program, idl in idls.items():
        for event in idl.get("events", []):
            requested.append((program, "event", event["name"], True))

    # Every production instruction must be pinned in PINNED_SCHEMAS. Report any IDL instruction
    # that has no "instruction_args" entry through the pending channel so it surfaces as an error.
    pinned_instructions: dict[str, set[str]] = {program: set() for program in idls}
    for program, category, name, _ in PINNED_SCHEMAS:
        if category == "instruction_args":
            pinned_instructions[program].add(name)
    for program, idl in idls.items():
        for instruction in idl.get("instructions", []):
            if instruction["name"] not in pinned_instructions[program]:
                pending.append(
                    {
                        "program": program,
                        "category": "instruction_args",
                        "name": instruction["name"],
                        "reason": "production instruction is not pinned in PINNED_SCHEMAS",
                    }
                )

    seen: set[tuple[str, str, str]] = set()
    for program, category, name, needs_fixture in requested:
        key = (program, category, name)
        if key in seen:
            continue
        seen.add(key)
        idl = idls[program]
        schema = schema_for(idl, category, name)
        if schema is None:
            pending.append(
                {
                    "program": program,
                    "category": category,
                    "name": name,
                    "reason": "planned schema is not present in the current Anchor IDL",
                }
            )
            continue

        entry: dict[str, Any] = {
            "program": program,
            "category": category,
            "name": name,
            "schema_hash": sha256_hex(canonical_json(strip_docs(schema))),
        }
        if needs_fixture:
            fixture = golden_fixture(idl, category, name)
            if fixture is not None:
                entry["fixture_hex"] = fixture.hex()
                entry["fixture_len"] = len(fixture)
        schemas.append(entry)

    schemas.sort(key=lambda item: (item["program"], item["category"], item["name"]))
    pending.sort(key=lambda item: (item["program"], item["category"], item["name"]))

    return {
        "version": 1,
        "description": (
            "Pinned Solana Anchor schema hashes and deterministic Borsh golden "
            "fixtures for off-chain listener/KMS layout mirrors."
        ),
        "event_versions": event_versions(root),
        "schemas": schemas,
        "pending_schemas": pending,
    }


def schema_for(idl: dict[str, Any], category: str, name: str) -> dict[str, Any] | None:
    if category == "account":
        account = find_by_name(idl.get("accounts", []), name)
        ty = find_by_name(idl.get("types", []), name)
        if account is None or ty is None:
            return None
        return {"account": account, "type": ty}
    if category == "event":
        event = find_by_name(idl.get("events", []), name)
        ty = find_by_name(idl.get("types", []), name)
        if event is None or ty is None:
            return None
        return {"event": event, "type": ty}
    if category == "type":
        return find_by_name(idl.get("types", []), name)
    if category == "instruction_args":
        instruction = find_by_name(idl.get("instructions", []), name)
        if instruction is None:
            return None
        return {
            "instruction": {
                "name": instruction["name"],
                "discriminator": instruction.get("discriminator"),
                "accounts": instruction.get("accounts", []),
                "args": instruction.get("args", []),
            }
        }
    raise ValueError(f"unsupported category {category}")


def golden_fixture(idl: dict[str, Any], category: str, name: str) -> bytes | None:
    types = {ty["name"]: ty for ty in idl.get("types", [])}
    try:
        if category == "account":
            account = find_by_name(idl.get("accounts", []), name)
            ty = types[name]
            return bytes(account["discriminator"]) + encode_defined(ty, types, 1)
        if category == "event":
            event = find_by_name(idl.get("events", []), name)
            ty = types[name]
            return bytes(event["discriminator"]) + encode_defined(ty, types, 1)
        if category == "type":
            return encode_defined(types[name], types, 1)
        if category == "instruction_args":
            instruction = find_by_name(idl.get("instructions", []), name)
            out = bytearray(instruction.get("discriminator", []))
            for index, arg in enumerate(instruction.get("args", []), start=1):
                out.extend(encode_type(arg["type"], types, index))
            return bytes(out)
    except UnsupportedFixtureType:
        return None
    return None


class UnsupportedFixtureType(Exception):
    pass


def encode_defined(ty: dict[str, Any], types: dict[str, Any], seed: int) -> bytes:
    body = ty["type"]
    kind = body["kind"]
    if kind == "struct":
        out = bytearray()
        for index, field in enumerate(body.get("fields", []), start=seed):
            out.extend(encode_type(field["type"], types, index))
        return bytes(out)
    if kind == "enum":
        variants = body.get("variants", [])
        if not variants:
            raise UnsupportedFixtureType()
        out = bytearray([0])
        fields = variants[0].get("fields", [])
        if isinstance(fields, list):
            for index, field in enumerate(fields, start=seed):
                field_type = field["type"] if isinstance(field, dict) else field
                out.extend(encode_type(field_type, types, index))
        return bytes(out)
    raise UnsupportedFixtureType()


def encode_type(idl_type: Any, types: dict[str, Any], seed: int) -> bytes:
    if isinstance(idl_type, str):
        if idl_type == "u8":
            return bytes([seed & 0xFF])
        if idl_type == "u16":
            return (seed + 16).to_bytes(2, "little")
        if idl_type == "u32":
            return (seed + 32).to_bytes(4, "little")
        if idl_type == "u64":
            return (seed + 64).to_bytes(8, "little")
        if idl_type == "bool":
            return b"\x01"
        if idl_type == "pubkey":
            return bytes([seed & 0xFF]) * 32
        if idl_type == "bytes":
            return (3).to_bytes(4, "little") + bytes(
                [(seed + 0) & 0xFF, (seed + 1) & 0xFF, (seed + 2) & 0xFF]
            )
        raise UnsupportedFixtureType()

    if "array" in idl_type:
        element_type, length = idl_type["array"]
        out = bytearray()
        for index in range(length):
            out.extend(encode_type(element_type, types, seed + index))
        return bytes(out)

    if "vec" in idl_type:
        return (1).to_bytes(4, "little") + encode_type(idl_type["vec"], types, seed)

    if "option" in idl_type:
        return b"\x01" + encode_type(idl_type["option"], types, seed)

    defined = idl_type.get("defined")
    if isinstance(defined, dict):
        defined = defined["name"]
    if isinstance(defined, str):
        return encode_defined(types[defined], types, seed)

    raise UnsupportedFixtureType()


def event_versions(root: pathlib.Path) -> dict[str, int]:
    versions = {}
    for spec in PROGRAMS.values():
        constants = root / spec["constants"]
        source = constants.read_text(encoding="utf-8")
        for version_name, const_name in spec["event_version_consts"].items():
            pattern = re.compile(
                rf"pub const {re.escape(const_name)}: u8 = ([0-9]+);"
            )
            match = pattern.search(source)
            if not match:
                raise RuntimeError(f"could not find {const_name} in {constants}")
            versions[version_name] = int(match.group(1))
    return versions


def manifest_diff(expected: dict[str, Any], current: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    expected_entries = {
        (item["program"], item["category"], item["name"]): item
        for item in expected.get("schemas", [])
    }
    current_entries = {
        (item["program"], item["category"], item["name"]): item
        for item in current.get("schemas", [])
    }
    for key in sorted(set(expected_entries) | set(current_entries)):
        if key not in expected_entries:
            errors.append(f"ABI schema {key} is missing from solana_abi_golden.json")
            continue
        if key not in current_entries:
            errors.append(f"ABI schema {key} is no longer present in the vendored IDLs")
            continue
        if expected_entries[key] != current_entries[key]:
            errors.append(
                f"ABI schema {key} drifted; run solana/scripts/sync-zama-host-idl.sh"
            )

    if expected.get("event_versions") != current.get("event_versions"):
        errors.append("Solana event version constants drifted from solana_abi_golden.json")

    if expected.get("pending_schemas") != current.get("pending_schemas"):
        errors.append(
            "pending Solana ABI schema list changed; run solana/scripts/sync-zama-host-idl.sh"
        )
    return errors


def find_by_name(items: list[dict[str, Any]], name: str) -> dict[str, Any] | None:
    return next((item for item in items if item.get("name") == name), None)


def strip_docs(value: Any) -> Any:
    if isinstance(value, dict):
        return {
            key: strip_docs(val)
            for key, val in value.items()
            if key not in {"docs"}
        }
    if isinstance(value, list):
        return [strip_docs(item) for item in value]
    return value


def canonical_json(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_json(path: pathlib.Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def write_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as fh:
        json.dump(value, fh, indent=2, sort_keys=True)
        fh.write("\n")


if __name__ == "__main__":
    raise SystemExit(main())
