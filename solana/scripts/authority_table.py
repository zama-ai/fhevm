#!/usr/bin/env python3
"""Generate and check the zama-host authority table (docs/AUTHORITY.md).

For every zama-host instruction the table lists who signs and what that signature stands for,
which accounts it writes and reads, which programs it can invoke, and what it takes as remaining
accounts. Accounts come from the committed IDL, which check_solana_abi.py pins to the build. What
the IDL cannot express is declared in DECLARATIONS below, and every IDL instruction and signer
must be declared. Three claims of the table rest on zama-host's source, so the check reads it
too: every CPI goes to the System program or to zama-host itself, every handler without declared
remaining accounts rejects them, and FEATURE_GATED lists exactly the feature-gated instructions.
Either way, a change to who may write what lands as a diff of the table or a failed check, and
security is asked to review both (.github/CODEOWNERS).
"""

from __future__ import annotations

import argparse
import difflib
import json
import pathlib
import re
import sys
from typing import Any

from check_solana_abi import PROGRAMS

TABLE = "docs/AUTHORITY.md"
HOST_SRC = "programs/zama-host/src"

# The capability boundaries of fhevm-internal#2083. Instructions are grouped by the capability
# that owns them.
CAPABILITIES = {
    "Governance": {
        "owns": "`HostConfig`, pause flags and pauser records, deny list, HCU limits and trust records",
        "changed_by": "admin; an enabled pauser can also set pause flags",
        "consumers": "host listener (`HostConfig.chain_id`, read at startup)",
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
        "changed_by": "the user: a wallet, or a PDA its program signs for",
        "consumers": "KMS connector",
    },
}

ADMIN = "`HostConfig.admin`"
PAYER = "pays rent; no authority"

# Per instruction: its capability, what each IDL signer's signature stands for, and the
# remaining accounts it accepts. The handler of an instruction without `remaining` must call
# `assert_no_remaining_accounts`.
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
        "signers": {"admin": f"{ADMIN}; `new_admin` co-signs unless it is a program-owned PDA"},
    },
    "pause": {
        "capability": "Governance",
        "signers": {
            "pauser": "a key with an enabled `PauserRecord`; it can set pause flags, not clear them; "
            "a wallet must call at the top level, a PDA may call through CPI",
        },
    },
    "unpause": {"capability": "Governance", "signers": {"admin": ADMIN}},
    "set_pauser": {"capability": "Governance", "signers": {"payer": PAYER, "admin": ADMIN}},
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
            "authority": "the authority of the execution store (`execution_store_index`), "
            "whose application is metered and seeds the randomness",
        },
        "remaining": (
            "the `EncryptedStore`s it reads or writes, each admitted by its authority's signature "
            "(as `authority` or a signing remaining account) and writable when written; the "
            "stores that receive grants, read-only and unsigned; the other signing store "
            "authorities; and, when the deny list is enabled, the deny record of each application "
            "whose store it reads, writes or grants from"
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
        "signers": {
            "payer": PAYER,
            "delegator": "the user granting the delegation; a wallet must call at the top "
            "level, a PDA may call through CPI",
        },
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

# Accounts the IDL pins to a fixed address: the program each one is, or None for a sysvar.
FIXED_ADDRESSES = {
    "11111111111111111111111111111111": "System",
    "Sysvar1nstructions1111111111111111111111111": None,
}

# Anchor's event CPI appends these two accounts; the program invokes itself to log the event.
EVENT_CPI_ACCOUNTS = ("event_authority", "program")

# The only instruction builders zama-host's hand-written CPIs may pass: the System program's and
# its own event CPI. Anchor's `init` and `realloc` constraints can only invoke System.
CPI_BUILDERS = ("&system_instruction::", "&event_cpi_instruction(")


def flat_accounts(accounts: list[dict[str, Any]]) -> list[dict[str, Any]]:
    out = []
    for account in accounts:
        out.extend(flat_accounts(account["accounts"]) if "accounts" in account else [account])
    return out


def cell(items: list[str]) -> str:
    return ", ".join(items) if items else "—"


def shown(account: dict[str, Any]) -> str:
    return f"`{account['name']}`" + ("?" if account.get("optional") else "")


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

    names = [account["name"] for account in accounts]
    event_cpi = names[-len(EVENT_CPI_ACCOUNTS):] == list(EVENT_CPI_ACCOUNTS)
    if event_cpi:
        accounts = accounts[: -len(EVENT_CPI_ACCOUNTS)]
    writes = [shown(account) for account in accounts if account.get("writable")]
    reads, calls = [], []
    for account in accounts:
        if account.get("signer") or account.get("writable"):
            continue
        address = account.get("address")
        if address is not None and address not in FIXED_ADDRESSES:
            errors.append(f"{name}: `{account['name']}` has unknown fixed address {address}")
        elif address is not None and FIXED_ADDRESSES[address] is not None:
            calls.append(FIXED_ADDRESSES[address])
        else:
            reads.append(shown(account))
    if event_cpi:
        calls.append("self (event CPI)")

    roles = [f"`{signer}`: {declared['signers'].get(signer, '')}" for signer in signers]
    return (
        f"| `{name}` | {'<br>'.join(roles) or '—'} | {cell(writes)} | {cell(reads)} "
        f"| {cell(calls)} | {declared.get('remaining', '—')} |"
    )


def render(idl: dict[str, Any], errors: list[str]) -> str:
    instructions = {instruction["name"]: instruction for instruction in idl["instructions"]}
    for name in instructions:
        if name not in DECLARATIONS:
            errors.append(f"{name}: not declared in DECLARATIONS; add its capability and signer roles")
    for name in DECLARATIONS:
        if name not in instructions:
            errors.append(f"{name}: declared but not in the IDL; remove its declaration")

    lines = [
        "# zama-host authority table",
        "",
        "Generated by `scripts/authority_table.py` from the committed zama-host IDL and the",
        "declarations in that script. Do not edit it by hand: change the program or the declarations,",
        "then run `python3 scripts/authority_table.py --root . --write` from `solana/`. CI fails when",
        "this file differs from what the script produces, and security is asked to review changes to",
        "both (`.github/CODEOWNERS`).",
        "",
        "Signers, writes, reads and calls come from the IDL, and every account an instruction takes",
        "appears in one of them. Calls lists the programs an instruction passes. The check fails on a",
        "CPI in zama-host's source to any program other than System or zama-host itself, so no other",
        "program can be called. Capabilities, signer roles and remaining accounts are declared, because",
        "the IDL cannot express them. \"—\" under remaining accounts means the handler rejects any; the",
        "check requires its `assert_no_remaining_accounts` call. `?` marks an optional account.",
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
            "| Instruction | Signers | Writes | Reads | Calls | Remaining accounts |",
            "|---|---|---|---|---|---|",
        ]
        for name, declared in DECLARATIONS.items():
            if declared["capability"] == capability and name in instructions:
                lines.append(instruction_row(instructions[name], errors))
    unknown = {d["capability"] for d in DECLARATIONS.values()} - set(CAPABILITIES)
    errors.extend(f"unknown capability {capability!r}" for capability in sorted(unknown))

    lines += ["", "## Not in the default build", ""]
    lines += [f"- `{name}`: {text}" for name, text in FEATURE_GATED.items()]
    return "\n".join(lines) + "\n"


def code_lines(path: pathlib.Path) -> str:
    """The file with comment lines blanked, keeping line numbers."""
    return "\n".join(
        "" if line.lstrip().startswith("//") else line for line in path.read_text().splitlines()
    )


def line_of(code: str, offset: int) -> int:
    return code.count("\n", 0, offset) + 1


def source_errors(src: pathlib.Path, idl: dict[str, Any]) -> list[str]:
    errors = []
    handlers = []
    for path in sorted(src.rglob("*.rs")):
        code = code_lines(path)
        where = f"{HOST_SRC}/{path.relative_to(src)}"
        for match in re.finditer(r"\binvoke\w*\s*\(\s*", code):
            if not code.startswith(CPI_BUILDERS, match.end()):
                errors.append(
                    f"{where}:{line_of(code, match.start())}: CPI whose instruction is not built by "
                    f"{' or '.join(CPI_BUILDERS)}; zama-host may call only System and itself"
                )
        for match in re.finditer(r"\bCpiContext\b|\binvoke\w* as\b", code):
            errors.append(
                f"{where}:{line_of(code, match.start())}: `{match.group(0)}` hides a CPI target "
                "from this check"
            )
        if "instructions" in path.relative_to(src).parts:
            handlers.append(code)

    handler_code = "\n".join(handlers)
    for instruction in idl["instructions"]:
        name = instruction["name"]
        if name not in DECLARATIONS or "remaining" in DECLARATIONS[name]:
            continue
        body = re.search(rf"^pub fn {name}\b.*?^}}", handler_code, re.M | re.S)
        if body is None:
            errors.append(f"{name}: no `pub fn {name}` handler under {HOST_SRC}/instructions")
        elif "assert_no_remaining_accounts(ctx.remaining_accounts)" not in body.group(0):
            errors.append(
                f"{name}: its handler does not call assert_no_remaining_accounts; reject remaining "
                "accounts or declare the ones it takes"
            )

    lib = code_lines(src / "lib.rs")
    program = re.search(r"^#\[program\]\npub mod \w+ \{\n(.*?)^\}", lib, re.M | re.S)
    if program is None:
        return errors + [f"{HOST_SRC}/lib.rs: no #[program] module"]
    gated = set(
        re.findall(r"#\[cfg\([^\n]*\)\]\s*(?:#\[[^\n]*\]\s*)*pub fn (\w+)", program.group(1))
    )
    for name in sorted(gated - set(FEATURE_GATED)):
        errors.append(f"{name}: feature-gated in lib.rs; describe it in FEATURE_GATED")
    for name in sorted(set(FEATURE_GATED) - gated):
        errors.append(f"{name}: in FEATURE_GATED but not feature-gated in lib.rs")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True, help="Solana workspace root")
    parser.add_argument("--write", action="store_true", help=f"rewrite {TABLE}")
    args = parser.parse_args()

    root = pathlib.Path(args.root).resolve()
    idl_path = (root / PROGRAMS["zama_host"]["vendored_idl"]).resolve()
    idl = json.loads(idl_path.read_text())
    errors: list[str] = []
    table = render(idl, errors)
    errors += source_errors(root / HOST_SRC, idl)
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
