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
    "test-suite/fhevm",
)
PRODUCER_SUFFIXES = (".rs", ".sh", ".ts")
# Extensionless executables that print labels the scripts read.
PRODUCER_NAMES = ("fhevm-cli",)

# Literal prefixes whose producer is outside this repository. Each entry says which one, so an
# unexplained miss cannot hide behind a blanket skip.
EXTERNAL_PATTERNS = {
    "custom program error": "the Solana runtime's transaction error text",
    "insufficient funds": "the Solana runtime's transaction error text",
    "error occurred": "the gateway addHostChain container (hardhat/ethers)",
    "reverted": "the gateway addHostChain container (hardhat/ethers)",
}

# A prefix this short is not a label; matching it proves nothing.
MIN_LITERAL = 6

# The pattern is a POSIX ERE; everything from the first of these onward is not literal text.
METACHARACTERS = re.compile(r"[\\\[\](){}.*+?^$|]")

# Captures the flags too, so a `-i` pattern is matched case-insensitively here as well. Fixed-string
# and basic-regexp greps are read the same way as EREs: reducing to the literal prefix is
# conservative for every dialect, since none of them treat plain text as a metacharacter.
GREP_CALL = re.compile(r"grep\s+(-[A-Za-z]+)\s+'([^']*)'")


def literal_prefix(pattern: str) -> str:
    match = METACHARACTERS.search(pattern)
    return (pattern if match is None else pattern[: match.start()]).strip()


def producer_text(files: list[Path], exclude: Path) -> str:
    """Everything that could print a grepped label, minus the script doing the grepping.

    A script is excluded from its own haystack because the line that greps for a label is usually
    followed by a `fail "no <label>: $out"` that repeats it, and that error message would vouch for
    the very pattern under test. Labels printed by a *different* script still count.
    """
    return "\n".join(path.read_text(errors="replace") for path in files if path != exclude)


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
    scripts = sorted(script_dir.glob("*.sh"))
    files = producer_files()
    failures = []
    checked = 0
    for script in scripts:
        haystack = producer_text(files, exclude=script)
        if not haystack:
            raise SystemExit("check-e2e-greps: found no producer files to read")
        lowered = haystack.lower()
        for number, line in enumerate(script.read_text().splitlines(), start=1):
            for flags, pattern in GREP_CALL.findall(line):
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


def self_test() -> int:
    """Proves the check can fail: a script grepping for a label nobody prints must be reported."""
    with tempfile.TemporaryDirectory() as directory:
        probe = Path(directory) / "probe.sh"
        probe.write_text(
            "#!/usr/bin/env bash\n"
            "X=\"$(echo \"$out\" | grep -oE 'result handle 0x[0-9a-f]+')\"\n"
            "Y=\"$(echo \"$out\" | grep -oE 'label no producer prints 0x[0-9a-f]+')\"\n"
        )
        failures, checked, _ = scan(Path(directory))
    if checked != 2:
        print(f"self-test: expected 2 checked labels, got {checked}", file=sys.stderr)
        return 1
    if len(failures) != 1 or "label no producer prints" not in failures[0]:
        print(f"self-test: expected one miss, got {failures}", file=sys.stderr)
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
