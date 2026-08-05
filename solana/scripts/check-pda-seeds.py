#!/usr/bin/env python3
"""Check explicit handwritten TypeScript/Rust PDA-seed counterparts."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import pathlib
import re
import sys


SOLANA_ROOT = pathlib.Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class SeedPair:
    ts_path: pathlib.Path
    ts_symbol: str
    rust_path: pathlib.Path
    rust_symbol: str


# Keep this list explicit. Global literal-set membership is unsafe: a wrong TypeScript seed could
# otherwise be vouched for by an unrelated Rust program that happens to use the same bytes.
SEED_PAIRS = (
    SeedPair(
        pathlib.Path("demo-dapp/src/vault/internal/batcherPdas.ts"),
        "PENDING_BURN_SEED",
        pathlib.Path("programs/confidential-token/src/constants.rs"),
        "PENDING_BURN_SEED",
    ),
)
RUST_USAGE_PATHS = (
    pathlib.Path("programs/confidential-token/src/state/mod.rs"),
    pathlib.Path("programs/confidential-token/src/instructions/confidential_burn.rs"),
    pathlib.Path("programs/confidential-token/src/instructions/redeem_burned_amount.rs"),
    pathlib.Path("programs/confidential-token/src/instructions/cancel_pending_burn.rs"),
)


def exactly_one(pattern: re.Pattern[str], source: str, description: str) -> re.Match[str]:
    matches = list(pattern.finditer(source))
    if len(matches) != 1:
        raise ValueError(f"expected exactly one {description}, found {len(matches)}")
    return matches[0]


def typescript_seed(source: str, symbol: str) -> str:
    match = exactly_one(
        re.compile(
            rf"\bconst\s+{re.escape(symbol)}\s*=\s*"
            rf"[A-Za-z_$][\w$]*\.encode\(\s*(['\"])([^'\"]+)\1\s*\)\s*;"
        ),
        source,
        f"TypeScript constant {symbol}",
    )
    return match.group(2)


def rust_seed(source: str, symbol: str) -> str:
    match = exactly_one(
        re.compile(
            rf"\bpub(?:\(crate\))?\s+const\s+{re.escape(symbol)}\s*:\s*"
            rf"&\s*\[u8\]\s*=\s*b\"([^\"\\]+)\"\s*;"
        ),
        source,
        f"Rust constant {symbol}",
    )
    return match.group(1)


def pair_error(pair: SeedPair, ts_source: str, rust_source: str) -> str | None:
    try:
        ts_value = typescript_seed(ts_source, pair.ts_symbol)
        rust_value = rust_seed(rust_source, pair.rust_symbol)
    except ValueError as error:
        return str(error)
    if ts_value == rust_value:
        return None
    return (
        f"{pair.ts_path}:{pair.ts_symbol} is {ts_value!r}, but "
        f"{pair.rust_path}:{pair.rust_symbol} uses {rust_value!r}"
    )


def usage_errors(sources: dict[pathlib.Path, str]) -> list[str]:
    errors: list[str] = []
    raw_sites = [
        path
        for path, source in sources.items()
        for _ in re.finditer(r'b"pending-burn"', source)
    ]
    expected_raw_site = pathlib.Path("programs/confidential-token/src/constants.rs")
    if raw_sites != [expected_raw_site]:
        errors.append(f"raw pending-burn literal sites are {raw_sites!r}, expected only {expected_raw_site}")
    for path in RUST_USAGE_PATHS:
        count = len(re.findall(r"\bPENDING_BURN_SEED\b", sources.get(path, "")))
        if count != 1:
            errors.append(f"{path} uses PENDING_BURN_SEED {count} times, expected 1")
    return errors


def self_test() -> int:
    pair = SEED_PAIRS[0]
    ts = "const PENDING_BURN_SEED = encoder.encode('pending-burn');"
    rust = 'pub const PENDING_BURN_SEED: &[u8] = b"pending-burn";'
    cases = (
        (ts, rust, False),
        (ts.replace("pending-burn", "batch"), rust, True),
        (ts, rust.replace("pending-burn", "pending_burn"), True),
        (ts.replace("const ", "const X = encoder.encode('batch');\nconst "), rust, False),
        (ts + "\n" + ts, rust, True),
    )
    for index, (ts_source, rust_source, should_fail) in enumerate(cases, 1):
        failed = pair_error(pair, ts_source, rust_source) is not None
        if failed != should_fail:
            print(f"check-pda-seeds: self-test case {index} failed", file=sys.stderr)
            return 1
    usage_fixture = {
        pathlib.Path("programs/confidential-token/src/constants.rs"): rust,
        **{path: "derive(PENDING_BURN_SEED);" for path in RUST_USAGE_PATHS},
    }
    if usage_errors(usage_fixture):
        print("check-pda-seeds: self-test failed to accept canonical Rust usage", file=sys.stderr)
        return 1
    mutated = dict(usage_fixture)
    mutated[RUST_USAGE_PATHS[0]] = 'derive(b"pending_burn");'
    if not usage_errors(mutated):
        print("check-pda-seeds: self-test failed to reject divergent Rust usage", file=sys.stderr)
        return 1
    print("check-pda-seeds: self-test OK")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()

    errors: list[str] = []
    for pair in SEED_PAIRS:
        error = pair_error(
            pair,
            (SOLANA_ROOT / pair.ts_path).read_text(),
            (SOLANA_ROOT / pair.rust_path).read_text(),
        )
        if error is not None:
            errors.append(error)
    rust_sources = {
        path.relative_to(SOLANA_ROOT): path.read_text()
        for path in (SOLANA_ROOT / "programs/confidential-token/src").rglob("*.rs")
    }
    errors.extend(usage_errors(rust_sources))
    if errors:
        for error in errors:
            print(f"check-pda-seeds: {error}", file=sys.stderr)
        return 1

    print(f"check-pda-seeds: OK ({len(SEED_PAIRS)} explicit seed pair checked)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
