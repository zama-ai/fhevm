#!/usr/bin/env python3
"""Pin solana-proof-store's ingest surface to the vendored zama-host IDL.

The proof-store hand-decodes a lifecycle subset of zama-host instructions
(`discriminator("…")` / `event_discriminator("…")` in decode.rs). Path deps keep
arg types honest; this script keeps the *instruction catalog* honest:

1. Every name decode.rs matches must exist in the host IDL.
2. Every host instruction must be either decoded or explicitly ignored here.
3. Required lifecycle events must be referenced from decode.rs.

When the host adds an instruction, CI fails until proof-store either decodes it
or lists it under INTENTIONALLY_IGNORED_INSTRUCTIONS.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

DISC_RE = re.compile(r'discriminator\("([a-z0-9_]+)"\)')
EVENT_DISC_RE = re.compile(r'event_discriminator\("([A-Za-z0-9_]+)"\)')

# Host instructions the proof-store does not ingest. New host IDL instructions
# must land in decode.rs or here — never silently.
INTENTIONALLY_IGNORED_INSTRUCTIONS = frozenset(
    {
        "define_kms_context",
        "delegate_for_user_decryption",
        "destroy_kms_context",
        "initialize_host_config",
        "revoke_delegation_for_user_decryption",
        "set_coprocessor_signers",
        "set_deny_subject",
        "set_grant_deny_list_enabled",
        "set_hcu_app_trusted",
        "set_hcu_block_cap_per_app",
        "set_host_pause",
        "set_max_hcu_depth_per_tx",
        "set_max_hcu_per_tx",
        "verify_public_decrypt",
    }
)

# Lifecycle events decode.rs must keep wired (born-public binding).
REQUIRED_EVENTS = frozenset({"PublicOutputsProducedEvent"})


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--repo-root",
        required=True,
        help="fhevm repository root (parent of solana/ and solana-proof-service/)",
    )
    args = parser.parse_args()

    root = pathlib.Path(args.repo_root).resolve()
    decode_rs = (
        root
        / "solana-proof-service"
        / "crates"
        / "solana-proof-store"
        / "src"
        / "decode.rs"
    )
    idl_path = (
        root
        / "coprocessor"
        / "fhevm-engine"
        / "host-listener"
        / "idl"
        / "zama_host.json"
    )

    if not decode_rs.is_file():
        print(f"skip: proof-store decode.rs not present at {decode_rs}")
        return 0
    if not idl_path.is_file():
        print(f"error: missing vendored host IDL {idl_path}", file=sys.stderr)
        return 1

    source = decode_rs.read_text(encoding="utf-8")
    decoded_instructions = frozenset(DISC_RE.findall(source))
    decoded_events = frozenset(EVENT_DISC_RE.findall(source))

    idl = json.loads(idl_path.read_text(encoding="utf-8"))
    idl_instructions = frozenset(ix["name"] for ix in idl.get("instructions", []))
    idl_events = frozenset(ev["name"] for ev in idl.get("events", []))

    errors: list[str] = []

    unknown_ix = sorted(decoded_instructions - idl_instructions)
    if unknown_ix:
        errors.append(
            "decode.rs references instruction names absent from zama_host IDL: "
            + ", ".join(unknown_ix)
        )

    unknown_ev = sorted(decoded_events - idl_events)
    if unknown_ev:
        errors.append(
            "decode.rs references event names absent from zama_host IDL: "
            + ", ".join(unknown_ev)
        )

    overlap = sorted(decoded_instructions & INTENTIONALLY_IGNORED_INSTRUCTIONS)
    if overlap:
        errors.append(
            "instruction(s) listed as ignored but still decoded in decode.rs: "
            + ", ".join(overlap)
        )

    unclassified = sorted(
        idl_instructions - decoded_instructions - INTENTIONALLY_IGNORED_INSTRUCTIONS
    )
    if unclassified:
        errors.append(
            "zama_host IDL instruction(s) neither decoded by proof-store nor "
            "listed in INTENTIONALLY_IGNORED_INSTRUCTIONS: "
            + ", ".join(unclassified)
            + " — add decode arms or classify as ignored"
        )

    stale_ignored = sorted(INTENTIONALLY_IGNORED_INSTRUCTIONS - idl_instructions)
    if stale_ignored:
        errors.append(
            "INTENTIONALLY_IGNORED_INSTRUCTIONS entries absent from IDL: "
            + ", ".join(stale_ignored)
        )

    missing_required = sorted(REQUIRED_EVENTS - decoded_events)
    if missing_required:
        errors.append(
            "decode.rs missing required lifecycle event_discriminator refs: "
            + ", ".join(missing_required)
        )

    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print(
        "proof-store ingest surface matches zama_host IDL "
        f"(decoded_ix={len(decoded_instructions)} "
        f"ignored_ix={len(INTENTIONALLY_IGNORED_INSTRUCTIONS)} "
        f"decoded_events={sorted(decoded_events)})"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
