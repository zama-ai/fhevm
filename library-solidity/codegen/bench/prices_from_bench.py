#!/usr/bin/env python3
"""Writes the work column of operatorsPrices.json from the coprocessors' CPU benchmark, and the casts' latency.

Usage: python3 prices_from_bench.py [cpu-results.csv] [operatorsPrices.json]

Work is 1,000,000 divided by the operations per second (µs of one machine's time). Latency is left as it is, except
that a cast's latency is raised to its measured latency, and then so it never falls as the width grows. The script
then enforces RFC 040's invariants on work: a ciphertext operation's work is at least its scalar form's; work never
falls as the width or the array grows; work never exceeds latency. It fails if a cap at latency breaks another
invariant. Running it again on its own output changes nothing.

Mapping:
- A cast takes the slowest and the most expensive source widths for its result type, for latency and work. Casts to
  Bool have no benchmark row and keep their price.
- FheSum and FheIsIn bucket leN uses the benchmark's array of N elements.
"""
import csv
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "cpu-results.csv")
PRICES = sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "..", "src", "operatorsPrices.json")

NAME = {
    "fheAdd": "add", "fheSub": "sub", "fheMul": "mul", "fheDiv": "div", "fheRem": "rem", "fheBitAnd": "and",
    "fheBitOr": "or", "fheBitXor": "xor", "fheShl": "shl", "fheShr": "shr", "fheRotl": "rotl", "fheRotr": "rotr",
    "fheEq": "eq", "fheNe": "ne", "fheGe": "ge", "fheGt": "gt", "fheLe": "le", "fheLt": "lt", "fheMin": "min",
    "fheMax": "max", "fheNeg": "neg", "fheNot": "not", "cast": "cast", "trivialEncrypt": "trivial_encrypt",
    "ifThenElse": "select", "fheRand": "rand", "fheRandBounded": "rand_bounded", "fheSum": "sum", "fheIsIn": "is_in",
    "fheMulDiv": "mul_div",
}
UNARY_FORM = {"neg": "unary", "not": "unary", "cast": "unary", "trivial_encrypt": "scalar", "select": "ct_ct",
              "rand": "-", "rand_bounded": "-"}
WIDTHS = ["Uint8", "Uint16", "Uint32", "Uint64", "Uint128", "Uint160", "Uint256"]
BUCKETS = ["le10", "le30", "le60", "le100"]


def load_bench(path):
    """Returns (latency µs, work µs) per (operation, form, width, from_width, array_len)."""
    with open(path) as f:
        rows = csv.DictReader(line for line in f if not line.startswith("#"))
        return {
            (r["operation"], r["form"], r["width"], r["from_width"], r["array_len"]):
            (float(r["latency_us"]), 1e6 / float(r["throughput_ops_s"]))
            for r in rows
        }


def measure(bench, op, section, fhe_type, bucket):
    width = "bool" if fhe_type == "Bool" else fhe_type[4:]
    if op == "cast":
        sources = [v for (o, _, w, _, _), v in bench.items() if o == "cast" and w == width]
        return (max(v[0] for v in sources), max(v[1] for v in sources)) if sources else None
    if bucket:
        return bench.get((op, "array", width, "NA", bucket[2:]))
    form = UNARY_FORM.get(op) or ("ct_scalar" if section == "scalar" else "ct_ct")
    return bench.get((op, form, width, "NA", "NA"))


def cells(spec):
    for section in ("scalar", "nonScalar", "types"):
        for fhe_type, cell in (spec.get(section) or {}).items():
            yield section, fhe_type, None, cell
    for fhe_type, buckets in (spec.get("nBucketed") or {}).items():
        for bucket, cell in buckets.items():
            yield "nBucketed", fhe_type, bucket, cell


def raise_monotone(values, column="work"):
    run = 0
    for cell in values:
        run = cell[column] = max(run, cell[column])


def width_columns(spec):
    """Yields each sequence of cells that must not fall as the width grows."""
    for section in ("scalar", "nonScalar", "types"):
        by_type = spec.get(section) or {}
        yield [by_type[t] for t in WIDTHS if t in by_type]
    buckets = spec.get("nBucketed") or {}
    for bucket in BUCKETS:
        yield [buckets[t][bucket] for t in WIDTHS if t in buckets and bucket in buckets[t]]


def array_rows(spec):
    """Yields each sequence of cells that must not fall as the array grows."""
    for by_bucket in (spec.get("nBucketed") or {}).values():
        yield [by_bucket[b] for b in BUCKETS if b in by_bucket]


def scalar_pairs(spec):
    """Yields (scalar, ciphertext) cells of the same type."""
    for fhe_type, cell in (spec.get("scalar") or {}).items():
        ciphertext = (spec.get("nonScalar") or {}).get(fhe_type)
        if ciphertext:
            yield cell, ciphertext


def enforce(spec):
    """Applies the invariants to work; returns the cells whose work was capped at latency."""
    # Raising within a width column keeps it monotone, and the max of two monotone columns is monotone, so one pass in
    # this order suffices; check() below proves it on the output.
    for column in width_columns(spec):
        raise_monotone(column)
    for scalar, ciphertext in scalar_pairs(spec):
        ciphertext["work"] = max(ciphertext["work"], scalar["work"])
    for row in array_rows(spec):
        raise_monotone(row)
    capped = []
    for section, fhe_type, bucket, cell in cells(spec):
        if cell["work"] > cell["latency"]:
            capped.append((section, fhe_type, bucket, cell["work"], cell["latency"]))
            cell["work"] = cell["latency"]
    return capped


def check(name, spec):
    """Fails if any invariant does not hold, for instance after a cap at latency."""
    for sequence in [*width_columns(spec), *array_rows(spec)]:
        works = [cell["work"] for cell in sequence]
        assert works == sorted(works), f"{name}: work falls as the width or the array grows: {works}"
    for scalar, ciphertext in scalar_pairs(spec):
        assert ciphertext["work"] >= scalar["work"], f"{name}: ciphertext work below scalar work"
    for _, fhe_type, bucket, cell in cells(spec):
        assert cell["work"] <= cell["latency"], f"{name} {fhe_type} {bucket}: work above latency"


def dump(value, indent=0):
    """Formats the JSON as the repository keeps it: one {work, latency} price per line."""
    pad = "  " * indent
    if isinstance(value, dict):
        if set(value) == {"work", "latency"}:
            return '{ "work": %d, "latency": %d }' % (value["work"], value["latency"])
        items = [f"{pad}  {json.dumps(k)}: {dump(v, indent + 1)}" for k, v in value.items()]
        return "{\n" + ",\n".join(items) + "\n" + pad + "}"
    return json.dumps(value)


def main():
    bench = load_bench(BENCH)
    with open(PRICES) as f:
        prices = json.load(f)
    for name, spec in prices.items():
        op = NAME[name]
        for section, fhe_type, bucket, cell in cells(spec):
            measured = measure(bench, op, section, fhe_type, bucket)
            if measured is None:
                continue
            latency, work = measured
            if op == "cast":
                cell["latency"] = max(cell["latency"], math.ceil(latency))
            cell["work"] = max(1, round(work))
        if op == "cast":
            raise_monotone((spec["types"][t] for t in WIDTHS if t in spec["types"]), "latency")
        for capped in enforce(spec):
            print(f"{name}: work capped at latency", capped)
        check(name, spec)
    with open(PRICES, "w") as f:
        f.write(dump(prices) + "\n")


if __name__ == "__main__":
    main()
