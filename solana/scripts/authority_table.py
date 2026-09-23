#!/usr/bin/env python3
"""Generate and check the zama-host authority table (docs/AUTHORITY.md).

For every zama-host instruction the table lists who signs and what that signature stands for,
which accounts it writes, which programs it can invoke, and what it takes as remaining accounts.
Signers, writes and program accounts come from the committed IDL, which check_solana_abi.py pins
to the build. What the IDL cannot express is declared in DECLARATIONS below, and every IDL
instruction and signer must be declared. Either way, a change to who may write what lands as a
diff of the table, which security reviews (.github/CODEOWNERS).
"""

from __future__ import annotations

import argparse
import difflib
import json
import pathlib
import sys
from typing import Any

from check_solana_abi import PROGRAMS

TABLE = "docs/AUTHORITY.md"

# The capability boundaries of fhevm-internal#2083. Instructions are grouped by the capability
# whose state they change.
CAPABILITIES = {
    "Governance": {
        "owns": "`HostConfig`, pause, deny list, HCU limits and trust records",
        "changed_by": "admin",
        "consumers": "none",
    },
    "Trust roots": {
        "owns": "KMS contexts, coprocessor signers, EIP-712 domain, `verify_public_decrypt`",
        "changed_by": "admin",
        "consumers": "confidential-token redemption, KMS connector",
    },
    "Execution": {
        "owns": "`fhe_execute`, HCU metering, type gate, handle derivation, transient store",
        "changed_by": "no role",
        "consumers": "host listener, `zama-fhe`",
    },
    "Stores and ACL": {
        "owns": "`EncryptedStore`, MMR, public release",
        "changed_by": "the store's authority, a PDA of its program",
        "consumers": "host listener, KMS connector (`zama-solana-acl`)",
    },
    "User rights": {
        "owns": "delegation, permit revocation",
        "changed_by": "the user's wallet",
        "consumers": "KMS connector",
    },
}

ADMIN = "`HostConfig.admin`"
PAYER = "pays rent; no authority"

# Per instruction: its capability, what each IDL signer's signature stands for, and the
# remaining accounts it accepts. An instruction without `remaining` rejects any
# (`assert_no_remaining_accounts`). `note` records a signing rule the IDL cannot show.
DECLARATIONS: dict[str, dict[str, Any]] = {
    "initialize_host_config": {
        "capability": "Governance",
        "signers": {
            "payer": PAYER,
            "admin": "the program's upgrade authority (`ProgramData`); becomes `HostConfig.admin`",
        },
    },
    "set_admin": {
        "capability": "Governance",
        "signers": {"admin": ADMIN},
        "note": "`new_admin` co-signs unless it is a program-owned PDA.",
    },
    "set_host_pause": {"capability": "Governance", "signers": {"admin": ADMIN}},
    "set_grant_deny_list_enabled": {"capability": "Governance", "signers": {"admin": ADMIN}},
    "set_deny_scope": {"capability": "Governance", "signers": {"payer": PAYER, "admin": ADMIN}},
    "set_max_hcu_per_tx": {"capability": "Governance", "signers": {"admin": ADMIN}},
    "set_max_hcu_depth_per_tx": {"capability": "Governance", "signers": {"admin": ADMIN}},
    "set_hcu_block_cap_per_app": {"capability": "Governance", "signers": {"admin": ADMIN}},
    "set_hcu_app_trusted": {
        "capability": "Governance",
        "signers": {"payer": PAYER, "admin": ADMIN},
    },
    "define_kms_context": {
        "capability": "Trust roots",
        "signers": {"admin": f"{ADMIN}; also pays rent"},
    },
    "destroy_kms_context": {"capability": "Trust roots", "signers": {"admin": ADMIN}},
    "set_coprocessor_signers": {"capability": "Trust roots", "signers": {"admin": ADMIN}},
    "set_eip712_domain": {"capability": "Trust roots", "signers": {"admin": ADMIN}},
    "verify_public_decrypt": {"capability": "Trust roots", "signers": {}},
    "open_transient_store": {"capability": "Execution", "signers": {"payer": PAYER}},
    "close_transient_store": {"capability": "Execution", "signers": {}},
    "fhe_execute": {
        "capability": "Execution",
        "signers": {
            "payer": PAYER,
            "authority": "the authority of the stores it reads and writes",
        },
        "remaining": (
            "the `EncryptedStore`s it reads and writes (writable; each store's authority signs, "
            "as `authority` or among these accounts), the other store authorities as signers, "
            "and the application deny records"
        ),
    },
    "create_encrypted_store": {
        "capability": "Stores and ACL",
        "signers": {
            "payer": PAYER,
            "authority": "a PDA of the calling program, proven from `authority_seeds`; "
            "becomes the store's authority",
        },
    },
    "make_store_handle_public": {
        "capability": "Stores and ACL",
        "signers": {"payer": PAYER, "authority": "`EncryptedStore.authority`"},
    },
    "delegate_for_user_decryption": {
        "capability": "User rights",
        "signers": {"payer": PAYER, "delegator": "the user granting the delegation"},
    },
    "revoke_delegation_for_user_decryption": {
        "capability": "User rights",
        "signers": {"delegator": "the user who granted the delegation"},
    },
    "revoke_permits": {
        "capability": "User rights",
        "signers": {"user": "the user whose permits are revoked; also pays rent"},
    },
}

# Instructions compiled only under a cargo feature, so absent from the committed IDL.
FEATURE_GATED = {
    "close_owned_accounts": (
        "`admin-sweep` builds only (enabled by `environments/preview-env.json`). The upgrade "
        "authority (`ProgramData`) signs; closes every zama-host-owned account passed as a "
        "remaining account and takes its rent."
    ),
}

# Accounts the IDL pins to a fixed address, and whether an instruction can invoke them.
FIXED_ADDRESSES = {
    "11111111111111111111111111111111": ("System", True),
    "Sysvar1nstructions1111111111111111111111111": ("instructions sysvar", False),
}

# Anchor's event CPI appends these two accounts; the program invokes itself to log the event.
EVENT_CPI_ACCOUNTS = ("event_authority", "program")


def flat_accounts(accounts: list[dict[str, Any]]) -> list[dict[str, Any]]:
    out = []
    for account in accounts:
        out.extend(flat_accounts(account["accounts"]) if "accounts" in account else [account])
    return out


def cell(items: list[str]) -> str:
    return ", ".join(items) if items else "—"


def instruction_row(instruction: dict[str, Any], errors: list[str]) -> str:
    name = instruction["name"]
    declared = DECLARATIONS[name]
    accounts = flat_accounts(instruction["accounts"])
    signers = [account["name"] for account in accounts if account.get("signer")]
    for signer in signers:
        if signer not in declared["signers"]:
            errors.append(f"{name}: signer `{signer}` has no declared role")
    for signer in declared["signers"]:
        if signer not in signers:
            errors.append(f"{name}: declares a role for `{signer}`, which does not sign")

    writes = [
        f"`{account['name']}`" + ("?" if account.get("optional") else "")
        for account in accounts
        if account.get("writable")
    ]
    calls = []
    for account in accounts:
        address = account.get("address")
        if address is None:
            continue
        if address not in FIXED_ADDRESSES:
            errors.append(f"{name}: `{account['name']}` has unknown fixed address {address}")
            continue
        label, invocable = FIXED_ADDRESSES[address]
        if invocable:
            calls.append(label)
    names = [account["name"] for account in accounts]
    if names[-len(EVENT_CPI_ACCOUNTS):] == list(EVENT_CPI_ACCOUNTS):
        calls.append("self (event CPI)")

    roles = [f"`{signer}`: {declared['signers'].get(signer, '?')}" for signer in signers]
    if "note" in declared:
        roles.append(declared["note"])
    return (
        f"| `{name}` | {'<br>'.join(roles) or '—'} | {cell(writes)} | {cell(calls)} "
        f"| {declared.get('remaining', '—')} |"
    )


def render(idl: dict[str, Any], errors: list[str]) -> str:
    instructions = {instruction["name"]: instruction for instruction in idl["instructions"]}
    for name in instructions:
        if name not in DECLARATIONS:
            errors.append(f"{name}: not declared in DECLARATIONS; add its capability and signer roles")
    for name in DECLARATIONS:
        if name not in instructions:
            errors.append(f"{name}: declared but not in the IDL; remove its declaration")
    for name in FEATURE_GATED:
        if name in instructions:
            errors.append(f"{name}: in the default IDL; move it from FEATURE_GATED to DECLARATIONS")

    lines = [
        "# zama-host authority table",
        "",
        "Generated by `scripts/authority_table.py` from the committed zama-host IDL and the",
        "declarations in that script. Do not edit it by hand: change the program or the declarations,",
        "then run `python3 scripts/authority_table.py --root . --write` from `solana/`. CI fails when",
        "this file differs from what the script produces, and security review owns both",
        "(`.github/CODEOWNERS`).",
        "",
        "Signers, writes and program calls come from the IDL. Capabilities, signer roles and remaining",
        "accounts are declared, because the IDL cannot express them. An instruction with no remaining",
        "accounts listed rejects any. `?` marks an optional account.",
        "",
        "## Capabilities",
        "",
        "| Capability | Owns | Who may change it | Other consumers |",
        "|---|---|---|---|",
    ]
    for capability, spec in CAPABILITIES.items():
        lines.append(
            f"| {capability} | {spec['owns']} | {spec['changed_by']} | {spec['consumers']} |"
        )
    for capability in CAPABILITIES:
        lines += [
            "",
            f"## {capability}",
            "",
            "| Instruction | Signers | Writes | Calls | Remaining accounts |",
            "|---|---|---|---|---|",
        ]
        for name, declared in DECLARATIONS.items():
            if declared["capability"] == capability and name in instructions:
                lines.append(instruction_row(instructions[name], errors))
    unknown = {d["capability"] for d in DECLARATIONS.values()} - set(CAPABILITIES)
    errors.extend(f"unknown capability {capability!r}" for capability in sorted(unknown))

    lines += ["", "## Not in the default build", ""]
    lines += [f"- `{name}`: {text}" for name, text in FEATURE_GATED.items()]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True, help="Solana workspace root")
    parser.add_argument("--write", action="store_true", help=f"rewrite {TABLE}")
    args = parser.parse_args()

    root = pathlib.Path(args.root).resolve()
    idl_path = (root / PROGRAMS["zama_host"]["vendored_idl"]).resolve()
    errors: list[str] = []
    table = render(json.loads(idl_path.read_text()), errors)
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    table_path = root / TABLE
    if args.write:
        table_path.write_text(table)
        print(f"wrote {table_path}")
        return 0
    committed = table_path.read_text() if table_path.exists() else ""
    if committed != table:
        sys.stderr.writelines(
            difflib.unified_diff(
                committed.splitlines(keepends=True),
                table.splitlines(keepends=True),
                f"{TABLE} (committed)",
                f"{TABLE} (from the IDL)",
            )
        )
        print(
            f"error: {TABLE} is out of date; run python3 scripts/authority_table.py "
            "--root . --write from solana/ and commit the result",
            file=sys.stderr,
        )
        return 1
    print(f"{TABLE} matches the IDL")
    return 0


if __name__ == "__main__":
    sys.exit(main())
