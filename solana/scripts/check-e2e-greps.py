#!/usr/bin/env python3
"""Check that every string the e2e scripts grep for is still printed by something.

The e2e scripts drive the live clients by reading their stdout: a phase runs a client, greps a
labeled line out of the output, and feeds the captured field to the next phase. Nothing ties the
grep pattern to the `println!` that produces it, so renaming a printed label leaves the script
grepping for text no producer emits any more.

That failure is silent twice over. `full-vertical.sh` runs under `bash -e`, so a `grep` that matches
nothing inside `VAR="$(...)"` aborts the script with status 1 and no message; and the full vertical
takes ~45 minutes of bring-up to reach the phase where it happens, so a red run says almost nothing
about which rename broke it.

This check runs in seconds instead. For every `grep` pattern in the e2e scripts it takes the
pattern's literal prefix — the part before the first regex metacharacter — and requires some producer
to contain that text: a live client, a script's own `echo`, or the test SDK. A label that no longer
exists anywhere is reported with the file and line that still expects it.

Patterns matching output this repo does not produce (the Solana runtime's error text, the gateway
container) are listed in EXTERNAL_PATTERNS with the reason, so the check stays exhaustive by default.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT_DIR = REPO_ROOT / "solana" / "scripts" / "e2e"

# Where a printed label can legitimately come from. Enumerated through `git ls-files`, so build
# output and dependencies are structurally out of reach: a first version globbed the working tree and
# passed locally on `@types/node`'s prose and a vendored `zstd.h`, while CI — with no target/ and no
# node_modules — reported the miss. A haystack that grows arbitrary English silently stops checking.
PRODUCER_PATHSPECS = (
    "solana/scripts",
    "solana/programs",
    "solana-proof-service",
    "test-suite/fhevm",
)
PRODUCER_SUFFIXES = (".rs", ".sh", ".ts")
# Extensionless executables that print labels the scripts read.
PRODUCER_NAMES = ("fhevm-cli",)

# These files describe patterns for a living instead of printing them, so their prose must never
# count as a producer. `dead-surface-check.sh` quoting `::InvalidKmsContext` in a comment was the
# only thing keeping this check green for that label — the real producer, `zama-host/src/errors.rs`,
# was not even in the haystack until `solana/programs` joined PRODUCER_PATHSPECS above.
EXCLUDED_PRODUCERS = ("dead-surface-check.sh", "check-e2e-greps.py")

# Literal prefixes whose producer is outside this repository. Each entry says which one, so an
# unexplained miss cannot hide behind a blanket skip.
EXTERNAL_PATTERNS = {
    "custom program error": "the Solana runtime's transaction error text",
    "insufficient funds": "the Solana runtime's transaction error text",
    "error occurred": "the gateway addHostChain container (hardhat/ethers)",
    "reverted": "the gateway addHostChain container (hardhat/ethers)",
    "creating token": "the spl-token CLI's own progress output",
    "0x96a56828": "a Solidity error selector, printed by the gateway container",
}

# A prefix this short is not a label; matching it proves nothing. Five, not six: `full-vertical.sh`
# greps `PUB H`, a real label printed by the live client, and a threshold of six skipped it.
MIN_LITERAL = 5

# The pattern is a POSIX ERE; everything from the first of these onward is not literal text.
METACHARACTERS = re.compile(r"[\\\[\](){}.*+?^$|]")

# Captures the flags too, so a `-i` pattern is matched case-insensitively here as well. Fixed-string
# and basic-regexp greps are read the same way as EREs: reducing to the literal prefix is
# conservative for every dialect, since none of them treat plain text as a metacharacter.
#
# Deliberately tolerant of the forms that were previously invisible, each of which occurs in these
# scripts today: double-quoted patterns (`grep -Fqx "..."`), several flag groups (`grep -m1 -oE`),
# `egrep`, and a bare `grep 'pattern'` with no flags at all. A grep this check cannot see is a
# claim it silently stops testing.
GREP_CALL = re.compile(
    r"\b(?:egrep|grep)\s+((?:-[A-Za-z0-9-]+\s+)*)(?:'([^']*)'|\"([^\"]*)\")"
)


def grep_calls(line: str) -> list[tuple[str, str]]:
    """(flags, pattern) for every grep on the line, whichever quoting and flag form it uses."""
    return [
        (flags, single if single else double)
        for flags, single, double in GREP_CALL.findall(line)
    ]


def literal_prefix(pattern: str) -> str:
    match = METACHARACTERS.search(pattern)
    return (pattern if match is None else pattern[: match.start()]).strip()


def producer_text(files: list[Path], exclude: Path) -> str:
    """Everything that could print a grepped label, minus the script doing the grepping.

    A script is excluded from its own haystack because the line that greps for a label is usually
    followed by a `fail "no <label>: $out"` that repeats it, and that error message would vouch for
    the very pattern under test. Labels printed by a *different* script still count.

    Grep lines are dropped from every other file too. Two e2e scripts greping for the same label
    used to satisfy each other's claim — 29 of 56 label uses (52%) survived renaming the only thing
    that actually printed them. A grep is a consumer; only an `echo`, a `println!` or a real string
    constant is evidence that the label still exists.
    """
    chunks = []
    for path in files:
        if path == exclude or path.name in EXCLUDED_PRODUCERS:
            continue
        text = path.read_text(errors="replace")
        if path.suffix == ".sh":
            text = "\n".join(
                line for line in text.splitlines() if not grep_calls(line)
            )
        chunks.append(text)
    return "\n".join(chunks)


def producer_files() -> list[Path]:
    """Tracked source files only — see PRODUCER_PATHSPECS for why that matters."""
    listed = subprocess.run(
        ["git", "ls-files", "-z", "--", *PRODUCER_PATHSPECS],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    files = []
    for name in filter(None, listed.split("\0")):
        path = REPO_ROOT / name
        if (path.suffix in PRODUCER_SUFFIXES or path.name in PRODUCER_NAMES) and path.is_file():
            files.append(path)
    return files


def scan(script_dir: Path) -> tuple[list[str], int, int]:
    """Returns the misses, how many labels were checked, and how many scripts were read."""
    # `*.test.sh` are unit tests of the helper functions: every grep in them asserts against a mock
    # call log the test writes itself, and one of them asserts a string is *absent*. Neither is a
    # claim that some producer still prints a label, which is the only thing this check can judge.
    scripts = sorted(p for p in script_dir.glob("*.sh") if not p.name.endswith(".test.sh"))
    files = producer_files()
    failures = []
    checked = 0
    for script in scripts:
        haystack = producer_text(files, exclude=script)
        if not haystack:
            raise SystemExit("check-e2e-greps: found no producer files to read")
        lowered = haystack.lower()
        for number, line in enumerate(script.read_text().splitlines(), start=1):
            for flags, pattern in grep_calls(line):
                # An alternation greps for several labels at once; each branch is its own claim.
                for branch in pattern.split("|"):
                    literal = literal_prefix(branch)
                    if len(literal) < MIN_LITERAL or literal in EXTERNAL_PATTERNS:
                        continue
                    checked += 1
                    found = (
                        literal.lower() in lowered if "i" in flags else literal in haystack
                    )
                    if not found:
                        failures.append(
                            f"{script}:{number}: nothing prints "
                            f"{literal!r} (pattern {branch!r})"
                        )
    return failures, checked, len(scripts)


MISSING_LABEL = "label no producer prints"


def self_test() -> int:
    """Proves the check can fail, and that its haystack is still the tracked-files one.

    The second half matters as much as the first. The original bug this check was written against
    was a haystack built by globbing the working tree, which passed locally on `@types/node`'s prose
    while CI reported the miss. A probe whose text appears nowhere on disk cannot detect that
    regression — widening the haystack never flips its verdict. So the probe label is also planted
    in an *untracked* file inside a producer pathspec: `git ls-files` must not see it, and if the
    haystack ever goes back to globbing, this file vouches for the probe and the self-test fails.
    """
    decoy = REPO_ROOT / "solana" / "scripts" / ".e2e-greps-selftest-untracked.sh"
    tracked = subprocess.run(
        ["git", "ls-files", "--error-unmatch", str(decoy.relative_to(REPO_ROOT))],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    if tracked.returncode == 0:
        print(f"self-test: {decoy} must stay untracked", file=sys.stderr)
        return 1
    decoy.write_text(f"echo '{MISSING_LABEL} 0xdeadbeef'\n")
    try:
        with tempfile.TemporaryDirectory() as directory:
            probe = Path(directory) / "probe.sh"
            probe.write_text(
                "#!/usr/bin/env bash\n"
                "X=\"$(echo \"$out\" | grep -oE 'result handle 0x[0-9a-f]+')\"\n"
                f"Y=\"$(echo \"$out\" | grep -oE '{MISSING_LABEL} 0x[0-9a-f]+')\"\n"
            )
            failures, checked, _ = scan(Path(directory))
    finally:
        decoy.unlink(missing_ok=True)
    if checked != 2:
        print(f"self-test: expected 2 checked labels, got {checked}", file=sys.stderr)
        return 1
    if len(failures) != 1 or MISSING_LABEL not in failures[0]:
        print(
            "self-test: expected exactly one miss — an untracked file must never vouch for a "
            f"label. Got {failures}",
            file=sys.stderr,
        )
        return 1
    print("check-e2e-greps: self-test OK (a missing label is reported, a live one is not)")
    return 0


def main() -> int:
    if "--self-test" in sys.argv[1:]:
        return self_test()

    failures, checked, script_count = scan(SCRIPT_DIR)
    if script_count == 0:
        print(f"check-e2e-greps: found no scripts under {SCRIPT_DIR}", file=sys.stderr)
        return 2

    if failures:
        print("check-e2e-greps: the e2e scripts grep for text no producer prints:\n")
        for failure in failures:
            print(f"  {failure}")
        print(
            "\nEither restore the printed label or update the script that reads it. A miss here is "
            "a silent `bash -e` abort ~45 minutes into the full vertical."
        )
        return 1

    print(f"check-e2e-greps: OK ({checked} grepped labels, {script_count} scripts)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
