#!/usr/bin/env python3
"""Check that the e2e scripts and scenarios still read text and JSON that something produces.

Two couplings, both across a language boundary that nothing typechecks:

1. The e2e scripts grep labeled lines out of a live client's stdout (`scan`, below).
2. The scenario suite requires an *exact* JSON key set from a probe it runs (`probe_key_failures`).

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

The second check has the same shape, one level up: `two-holder-transfer.ts` requires the balance-state
probe's JSON key set *exactly*, and a renamed Rust field changed the emitted key while the consumer
went on demanding the old one. Its own unit test could not catch that, because a hand-written fixture
agrees with the parser it ships beside and has no way to disagree with the producer — so the required
key sets are compared against the producer instead.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT_DIR = REPO_ROOT / "solana" / "scripts" / "e2e"

# Where a printed label can legitimately come from. Enumerated through `git ls-files`, so build output
# and dependencies cannot get in: a first version globbed the working tree and passed locally on
# `@types/node`'s prose and a vendored `zstd.h`, while CI — with no target/ and no node_modules —
# reported the miss.
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
EXCLUDED_PRODUCERS = ("dead-surface-check.sh",)

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

# A prefix this short is not a label; matching it proves nothing. Five, because the shortest real
# label these scripts grep for is five characters.
MIN_LITERAL = 5

# The pattern is a POSIX ERE; everything from the first of these onward is not literal text.
METACHARACTERS = re.compile(r"[\\\[\](){}.*+?^$|]")

# Captures the flags too, so a `-i` pattern is matched case-insensitively here as well. Fixed-string
# and basic-regexp greps are read the same way as EREs: reducing to the literal prefix is
# conservative for every dialect, since none of them treat plain text as a metacharacter.
#
# Matches one flag group and a single- or double-quoted pattern. Only the single-quoted one-flag form
# appears in the scripts today; the rest is slack, so a reformatted grep does not become invisible.
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
    # A leading `^` anchors the match without changing the text being matched, so strip it first —
    # otherwise the literal comes out empty and the whole pattern is skipped as too short.
    pattern = pattern.lstrip("^")
    match = METACHARACTERS.search(pattern)
    return (pattern if match is None else pattern[: match.start()]).strip()


def producer_text(files: list[Path], exclude: Path) -> str:
    """Everything that could print a grepped label, minus the script doing the grepping.

    A script is excluded from its own haystack because the line that greps for a label is usually
    followed by a `fail "no <label>: $out"` that repeats it, and that error message would vouch for
    the very pattern under test. Labels printed by a *different* script still count.

    Grep lines are dropped from every other file too: two e2e scripts grepping for the same label
    used to satisfy each other's claim, so removing the only thing that actually printed it changed
    nothing. A grep is a consumer; only an `echo`, a `println!` or a real string constant is evidence
    that the label still exists.
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
    """Tracked, non-test source files — see PRODUCER_PATHSPECS for why tracked matters.

    Test files are dropped here rather than at each call site. Check 1 used to read them, so a
    renamed producer label whose old spelling survived in any `.test.ts` or `.test.sh` still looked
    present and the rename passed. Filename-level only: a `#[cfg(test)]` module inside a production
    `.rs` is still part of the haystack.
    """
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
        if is_test_file(path):
            continue
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


# --- Check 2: the probe JSON key sets -------------------------------------------------------------

# A consumer requiring an exact key set. The balance-state probe's call spans ten lines, so this has
# to read across newlines — which it does without `re.DOTALL`, because a negated class and `\s` both
# match a newline and there is no `.` in the pattern. The first argument is anything up to the comma,
# so `hasExactKeys(value.data, [...])` and `hasExactKeys(parse(out), [...])` are read too.
HAS_EXACT_KEYS = re.compile(r"hasExactKeys\(\s*[^,()]*\(?[^,]*\)?\s*,\s*\[([^\]]*)\]")
QUOTED = re.compile(r"""['"]([^'"]+)['"]""")
# What may appear in a required list besides the quoted keys themselves. A spread (`...BASE`) or a
# named constant means the real set is larger than what is written here, and silently comparing the
# visible part would reach a verdict about a set the consumer does not actually require.
LIST_NOISE = re.compile(r"""(['"][^'"]*['"]|[\s,])+""")

# A Rust producer: a serde struct that renames its fields to camelCase on the way out. `rename_all` is
# what makes the mapping mechanical enough to check by case conversion; a struct that also renames
# individual fields is refused below rather than converted wrongly.
RUST_CAMEL_STRUCT = re.compile(
    r'#\[serde\(rename_all\s*=\s*"camelCase"\)\]\s*(?:pub\s+)?struct\s+\w+\s*\{(.*?)\n\}', re.S
)
RUST_FIELD = re.compile(r"^\s*(?:pub\s+)?([a-z_][a-z_0-9]*)\s*:", re.M)
# A per-field `rename`, `skip` or `flatten` makes the emitted key set something other than the
# case-converted field names, so such a struct declares nothing rather than declaring it wrongly.
RUST_UNCONVERTIBLE = re.compile(r"#\[serde\((rename\s*=|skip|flatten)")

# A TypeScript producer: one object literal handed to JSON.stringify, no nesting. Shorthand
# properties count, which is the form the transfer worker uses.
TS_STRINGIFY = re.compile(r"JSON\.stringify\(\s*\{([^{}]*)\}\s*\)")
TS_IDENTIFIER = re.compile(r"[A-Za-z_$][\w$]*")


def camel(field: str) -> str:
    head, *rest = field.split("_")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def is_test_file(path: Path) -> bool:
    """A test file is a consumer's own fixture, never evidence that a producer emits something.

    This is the whole point of the check: a hand-written fixture agrees with the parser it ships
    beside, so letting one vouch for a key set restores exactly the blindness the check exists to
    remove. One `JSON.stringify({ ... })` in a `.test.ts` would do it.
    """
    return path.name.endswith((".test.ts", ".test.tsx", ".test.sh")) or path.name == "tests.rs"


def declared_key_sets(files: list[Path]) -> list[frozenset[str]]:
    """Every JSON key set some producer in `files` emits, from either language."""
    declared = []
    for path in files:
        if is_test_file(path):
            continue
        text = path.read_text(errors="replace")
        if path.suffix == ".rs":
            for body in RUST_CAMEL_STRUCT.findall(text):
                if RUST_UNCONVERTIBLE.search(body):
                    continue
                declared.append(frozenset(camel(name) for name in RUST_FIELD.findall(body)))
        elif path.suffix == ".ts":
            for body in TS_STRINGIFY.findall(text):
                keys = set()
                for part in filter(None, (piece.strip() for piece in body.split(","))):
                    name = part.split(":")[0].strip()
                    # A spread, a computed key, or a value containing a comma: not a shape this
                    # check can read, so it vouches for nothing rather than for the wrong set.
                    if not TS_IDENTIFIER.fullmatch(name):
                        keys = None
                        break
                    keys.add(name)
                if keys:
                    declared.append(frozenset(keys))
    return declared


def probe_key_failures(files: list[Path]) -> tuple[list[str], int]:
    """Every required key set that no producer emits, and how many were checked."""
    declared = declared_key_sets(files)
    failures = []
    checked = 0
    for path in files:
        if path.suffix != ".ts" or is_test_file(path):
            continue
        text = path.read_text(errors="replace")
        for match in HAS_EXACT_KEYS.finditer(text):
            listed = match.group(1)
            required = frozenset(QUOTED.findall(listed))
            if not required:
                continue
            checked += 1
            line = text.count("\n", 0, match.start()) + 1
            if not LIST_NOISE.fullmatch(listed):
                failures.append(
                    f"{path}:{line}: required key set is not written out as quoted keys "
                    f"({listed.strip()!r}) — the real set is larger than what is visible here, so "
                    f"this check cannot compare it against a producer"
                )
                continue
            if required in declared:
                continue
            closest = min(
                declared, key=lambda other: len(other ^ required), default=frozenset()
            )
            failures.append(
                f"{path}:{line}: no probe emits the required key set "
                f"{sorted(required)}; closest producer emits {sorted(closest)}"
            )
    return failures, checked


MISSING_LABEL = "label no producer prints"
MISSING_KEY = "keyNoProbeEmits"


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
    if (code := probe_key_self_test()) != 0:
        return code
    print("check-e2e-greps: self-test OK (a missing label is reported, a live one is not)")
    return 0


# Keys that appear only in a producer shape this check must refuse to read, or only in a test
# fixture. If any of them ever shows up in a declared key set, a parser has become over-permissive and
# is vouching for something no probe emits.
POISON_KEYS = ("renamedOnly", "plainOnly", "spreadOnly", "onlyInTestFixture")


def probe_key_self_test() -> int:
    """Proves the key-set check fires on a renamed key, and that nothing over-permissive vouches.

    Two halves. The first asserts what the producer parsers read and — more importantly — what they
    refuse to read, because every parser bug found while writing this made a parser *more* permissive,
    and an over-permissive producer parser vouches for a key set nothing emits. The second runs the
    whole check on fixtures whose verdicts are known.

    The `.test.ts` fixture is the one that matters most: a hand-written fixture agrees with the parser
    it ships beside, so if one can stand in for a producer, the check is back to comparing a consumer
    against itself, which is the blindness it exists to remove.
    """
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        (root / "producer.rs").write_text(
            "#[derive(Serialize)]\n"
            '#[serde(rename_all = "camelCase")]\n'
            "struct Probe {\n    version: u8,\n    token_account: String,\n}\n"
        )
        (root / "producer_renamed.rs").write_text(
            "#[derive(Serialize)]\n"
            '#[serde(rename_all = "camelCase")]\n'
            "struct Renamed {\n    version: u8,\n"
            '    #[serde(rename = "other")]\n    renamed_only: String,\n}\n'
        )
        (root / "producer_plain.rs").write_text(
            "#[derive(Serialize)]\nstruct Plain {\n    version: u8,\n    plain_only: String,\n}\n"
        )
        (root / "worker.ts").write_text(
            "process.stdout.write(JSON.stringify({ version: 1, signature }));\n"
        )
        (root / "worker_spread.ts").write_text(
            "process.stdout.write(JSON.stringify({ ...base, spreadOnly }));\n"
        )
        (root / "fixture.test.ts").write_text(
            "const fixture = JSON.stringify({ version: 1, onlyInTestFixture: 2 });\n"
        )
        (root / "consumer.ts").write_text(
            'hasExactKeys(value, ["version", "tokenAccount"]);\n'
            # Deliberately split across lines: the real balance-state call spans ten of them.
            'hasExactKeys(parseJsonLine(out), [\n  "version",\n  "signature",\n]);\n'
            f'hasExactKeys(value, ["version", "{MISSING_KEY}"]);\n'
            'hasExactKeys(value, [...BASE, "spreadOnly"]);\n'
            'hasExactKeys(value, ["version", "onlyInTestFixture"]);\n'
        )
        files = sorted(root.iterdir())
        declared = declared_key_sets(files)
        failures, checked = probe_key_failures(files)

    for wanted in (frozenset({"version", "tokenAccount"}), frozenset({"version", "signature"})):
        if wanted not in declared:
            print(
                f"self-test: {sorted(wanted)} is emitted by a fixture producer but was not read "
                f"(declared: {[sorted(s) for s in declared]})",
                file=sys.stderr,
            )
            return 1
    leaked = [key for key in POISON_KEYS if any(key in keys for keys in declared)]
    if leaked:
        print(
            f"self-test: a producer parser read a shape it must refuse — {leaked} appeared in "
            f"{[sorted(s) for s in declared]}",
            file=sys.stderr,
        )
        return 1
    if checked != 5:
        print(f"self-test: expected 5 checked key sets, got {checked}", file=sys.stderr)
        return 1
    expected = (MISSING_KEY, "not written out as quoted keys", "onlyInTestFixture")
    missing = [want for want in expected if not any(want in failure for failure in failures)]
    if len(failures) != 3 or missing:
        print(
            f"self-test: expected exactly these three key-set misses {expected}, got {failures}",
            file=sys.stderr,
        )
        return 1
    return 0


def main() -> int:
    if "--self-test" in sys.argv[1:]:
        return self_test()

    failures, checked, script_count = scan(SCRIPT_DIR)
    if script_count == 0:
        print(f"check-e2e-greps: found no scripts under {SCRIPT_DIR}", file=sys.stderr)
        return 2

    if checked == 0:
        print(
            "check-e2e-greps: read 6 scripts and found no grep to check — the grep matcher has "
            "stopped seeing the forms these scripts use",
            file=sys.stderr,
        )
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

    key_failures, key_sets = probe_key_failures(producer_files())
    if key_sets == 0:
        print("check-e2e-greps: found no probe key sets to check", file=sys.stderr)
        return 2
    if key_failures:
        print("check-e2e-greps: a scenario requires a JSON key set no probe emits:\n")
        for failure in key_failures:
            print(f"  {failure}")
        print(
            "\nThe consumer follows the producer. A probe emits its keys either from a Rust serde "
            "struct with `rename_all = \"camelCase\"` or from one `JSON.stringify({ ... })` object "
            "literal; if the producer is a shape this check cannot read, it counts as absent."
        )
        return 1

    print(
        f"check-e2e-greps: OK ({checked} grepped labels, {script_count} scripts, "
        f"{key_sets} probe key sets)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
