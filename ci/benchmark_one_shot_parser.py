"""
benchmark_one_shot_parser
-------------------------

Parse reportable Main-baseline one-shot benchmark artifacts and dump them in the
Slab series format produced by ``benchmark_parser.py``.

The reportable one-shot targets
(``make benchmark_erc20_main_block_baseline_{cpu,gpu}``) do not go through
Criterion: each run measures a single logical workload once and writes a JSON
artifact to ``<criterion home>/benchmark-runs/reportable-main-one-shot-*.json``.
``benchmark_parser.py`` only understands Criterion result directories, so the
one-shot runs need this converter to reach Slab.
"""

import argparse
import json
import pathlib
import re
import sys

ONE_SECOND_IN_NANOSECONDS = 1e9
ONE_SECOND_IN_MILLISECONDS = 1e3

# Every point the coprocessor sends to Slab is recorded with this operator type:
# Criterion runs get it from `OperatorType::Atomic`, which serializes under its
# variant name. Keep one-shot points in the same category.
OPERATOR_TYPE = "Atomic"
BENCH_CLASS = "evaluate"
BENCH_NAME = "erc20::transfer::main_block_one_shot"
# Artifact layout this parser understands, written by
# `persist_main_block_one_shot_artifact`. A bump there must be handled here.
SUPPORTED_SCHEMA_VERSION = 2
ARTIFACT_NAME_PATTERN = re.compile(
    r"reportable-main-one-shot-[0-9a-f]+-(?P<timestamp>\d+)\.json"
)

parser = argparse.ArgumentParser()
parser.add_argument(
    "results",
    help="Directory containing reportable-main-one-shot-*.json artifacts",
)
parser.add_argument("output_file", help="File storing parsed results")
parser.add_argument(
    "-d",
    "--database",
    dest="database",
    help="Name of the database used to store results",
)
parser.add_argument(
    "-w",
    "--hardware",
    dest="hardware",
    help="Hardware reference used to perform benchmark",
)
parser.add_argument(
    "-V", "--project-version", dest="project_version", help="Commit hash reference"
)
parser.add_argument(
    "-b",
    "--branch",
    dest="branch",
    help="Git branch name on which benchmark was performed",
)
parser.add_argument(
    "--commit-date",
    dest="commit_date",
    help="Timestamp of commit hash used in project_version",
)
parser.add_argument(
    "--bench-date", dest="bench_date", help="Timestamp when benchmark was run"
)
parser.add_argument(
    "--name-suffix",
    dest="name_suffix",
    default="",
    help="Suffix to append to each of the result test names",
)
parser.add_argument(
    "--backend",
    dest="backend",
    default="cpu",
    help="Backend on which benchmarks have run",
)
parser.add_argument(
    "--append-results",
    dest="append_results",
    action="store_true",
    help="Append parsed results to an existing file",
)
parser.add_argument(
    "--expected-scenarios",
    dest="expected_scenarios",
    default="",
    help="Comma-separated scenarios that must be present, the program will exit"
    " with an error if one of them produced no artifact",
)
parser.add_argument(
    "--expected-revision",
    dest="expected_revision",
    default="",
    help="Commit the artifacts must have been built from, to reject leftover"
    " artifacts of an earlier run on a reused workspace",
)


def artifact_files(directory):
    """
    List one-shot artifacts, oldest first.

    Artifact names end with the run timestamp in milliseconds, which orders runs
    chronologically. A name that does not carry one cannot be placed in that
    order, so it is skipped with a warning rather than taken as the sort key:
    raising here would lose the runs that did report.

    :param directory: directory holding the artifacts as :class:`pathlib.Path`

    :return: :class:`list` of :class:`pathlib.Path`
    """
    timestamped = []
    for path in directory.glob("reportable-main-one-shot-*.json"):
        match = ARTIFACT_NAME_PATTERN.fullmatch(path.name)
        if match is None:
            print(f"Warning: ignoring artifact with unexpected name '{path.name}'")
            continue
        timestamped.append((int(match.group("timestamp")), path))
    return [path for _, path in sorted(timestamped, key=lambda item: item[0])]


def latest_run_per_scenario(directory, expected_revision=""):
    """
    Keep the last usable run of each scenario found in ``directory``.

    An artifact this parser cannot trust is announced and dropped: an unreadable
    file, an unknown schema version, or a run built from another commit (a
    leftover in a reused workspace or a restored target cache). Dropping is not
    silent tolerance — because a dropped artifact no longer stands in for a
    scenario, ``--expected-scenarios`` still fails a run whose scenario has no
    fresh artifact, while a stale file alongside complete fresh results no longer
    fails a run that measured everything it was asked to.

    :param directory: directory holding the artifacts as :class:`pathlib.Path`
    :param expected_revision: commit the artifacts must be built from, if known

    :return: :class:`dict` of scenario name to parsed artifact content
    """
    runs = {}
    for path in artifact_files(directory):
        try:
            content = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as err:
            print(f"Warning: ignoring unreadable artifact '{path.name}': {err}")
            continue
        if content.get("run_mode") != "reportable":
            continue
        schema_version = content.get("schema_version")
        if schema_version != SUPPORTED_SCHEMA_VERSION:
            print(
                f"Warning: ignoring artifact '{path.name}' with schema version"
                f" {schema_version}, expected {SUPPORTED_SCHEMA_VERSION}"
            )
            continue
        revision = content.get("build", {}).get("revision")
        if expected_revision and revision != expected_revision:
            print(
                f"Warning: ignoring artifact '{path.name}' built from revision"
                f" {revision}, expected {expected_revision}"
            )
            continue
        try:
            scenario = content["result"]["scenario"]
        except (KeyError, TypeError) as err:
            print(f"Warning: ignoring artifact '{path.name}' with no scenario: {err}")
            continue
        runs[scenario] = content
    return runs


def _create_point(value, test_name, bench_type, params, display_name):
    return {
        "value": value,
        "test": test_name,
        "name": display_name,
        "class": BENCH_CLASS,
        "type": bench_type,
        "operator": OPERATOR_TYPE,
        "params": params,
    }


def scenario_parameters(content):
    """
    Flatten the artifact facts that qualify a measurement into Slab parameters.

    :param content: parsed artifact content as :class:`dict`

    :return: :class:`dict` of parameters
    """
    result = content["result"]
    topology = result["transfer_topology"]
    dispatch = result["dispatch"]
    build = content["build"]
    return {
        "scenario": result["scenario"],
        "topology": result["topology"],
        "worker_semantics": result["worker_semantics"],
        "primary_metric": result["measurement_methodology"]["primary_metric"],
        "transfer_count": result["logical_block"]["transfer_count"],
        "l1_block_count": result["blocks_committed"],
        "operations_per_transfer": topology["operations_per_transfer_count"],
        "dependence_chain_count": result["dependence_chain_count"],
        "dependent_transfers_per_chain": topology["dependent_transfers_per_chain"],
        "transaction_identity": topology["transaction_identity"],
        "transaction_id_count": topology["transaction_id_count"],
        "cross_transaction_balance_dependency_edge_count": topology[
            "same_block_cross_transaction_balance_dependency_edge_count"
        ],
        "dependence_lag_l1_blocks": topology["dependence_lag_l1_blocks"],
        "computation_count": result["computation_count"],
        "terminal_handle_count": result["terminal_handle_count"],
        "work_items_batch_size": dispatch["work_items_batch_size"],
        "dependence_chains_per_batch": dispatch["dependence_chains_per_batch"],
        "dcid_batch_execution": dispatch["dcid_batch_execution"],
        # Recorded from the run that introduced the adaptive window; artifacts
        # written before it carry no such key.
        "dcid_adaptive_batch_execution": dispatch.get("dcid_adaptive_batch_execution"),
        "backend": build["backend"],
        "features": ",".join(build["features"]),
        "bench_lto": build["bench_lto"],
    }


def parse_scenario(content, name_suffix):
    """
    Turn one artifact into its Slab data points.

    The primary metric is reported as a latency in nanoseconds, and both FHE
    operation and transfer rates are derived from it as throughputs.

    :param content: parsed artifact content as :class:`dict`
    :param name_suffix: a :class:`str` suffix to apply to each test name

    :return: tuple of :class:`list` as (data points, parsing failures)
    """
    result = content["result"]
    scenario = result["scenario"]
    params = scenario_parameters(content)
    primary_ms = result["post_commit_worker_visible_to_terminal_outputs_ms"]
    upper_bound_ms = result["commit_start_to_terminal_outputs_upper_bound_ms"]

    for metric_name, value_ms in (
        ("primary metric", primary_ms),
        ("commit-start upper bound", upper_bound_ms),
    ):
        if value_ms <= 0:
            return [], [(scenario, f"non-positive {metric_name}: {value_ms} ms")]

    primary_seconds = primary_ms / ONE_SECOND_IN_MILLISECONDS
    measurements = [
        (
            result["measurement_methodology"]["primary_metric"],
            "latency",
            primary_ms * ONE_SECOND_IN_NANOSECONDS / ONE_SECOND_IN_MILLISECONDS,
        ),
        (
            "commit_start_to_terminal_outputs_upper_bound",
            "latency",
            upper_bound_ms * ONE_SECOND_IN_NANOSECONDS / ONE_SECOND_IN_MILLISECONDS,
        ),
        (
            "fhe_operations_per_second",
            "throughput",
            result["computation_count"] / primary_seconds,
        ),
        (
            "transfers_per_second",
            "throughput",
            result["logical_block"]["transfer_count"] / primary_seconds,
        ),
    ]

    points = []
    for metric_name, bench_type, value in measurements:
        display_name = "::".join([BENCH_NAME, scenario, metric_name])
        test_name = "_".join(filter(None, [display_name, name_suffix]))
        points.append(
            _create_point(value, test_name, bench_type, params, display_name)
        )
    return points, []


def dump_results(parsed_results, filename, input_args):
    """
    Dump parsed results formatted as JSON to file.

    :param parsed_results: :class:`list` of data points
    :param filename: filename for dump file as :class:`pathlib.Path`
    :param input_args: CLI input arguments
    """
    for point in parsed_results:
        point["backend"] = input_args.backend

    if input_args.append_results:
        parsed_content = json.loads(filename.read_text(encoding="utf-8"))
        parsed_content["points"].extend(parsed_results)
        filename.write_text(json.dumps(parsed_content), encoding="utf-8")
    else:
        filename.parent.mkdir(parents=True, exist_ok=True)
        series = {
            "database": input_args.database,
            "hardware": input_args.hardware,
            "project_version": input_args.project_version,
            "branch": input_args.branch,
            "insert_date": input_args.bench_date,
            "commit_date": input_args.commit_date,
            "points": parsed_results,
        }
        filename.write_text(json.dumps(series), encoding="utf-8")


def check_mandatory_args(input_args):
    """
    Check for availability of required input arguments, the program will exit if
    one of them is not present. If `append_results` flag is set, all the
    required arguments will be ignored.

    :param input_args: CLI input arguments
    """
    if input_args.append_results:
        return

    missing_args = [
        arg_name
        for arg_name in (
            "database",
            "hardware",
            "project_version",
            "branch",
            "commit_date",
            "bench_date",
        )
        if not getattr(input_args, arg_name)
    ]
    if missing_args:
        for arg_name in missing_args:
            print(f"Missing required argument: --{arg_name.replace('_', '-')}")
        sys.exit(1)


def main(input_args):
    results_dir = pathlib.Path(input_args.results)
    if not results_dir.is_dir():
        print(f"No one-shot artifact directory at '{results_dir}'")
        sys.exit(1)

    runs = latest_run_per_scenario(results_dir, input_args.expected_revision)
    if not runs:
        print(f"No usable reportable one-shot artifact found in '{results_dir}'")
        sys.exit(1)

    parsed_results = []
    failures = []
    for scenario, content in sorted(runs.items()):
        print(f"Parsing one-shot scenario '{scenario}'... ")
        # One unparsable artifact must not cost the scenarios that did report:
        # this step publishes what it could parse and fails afterwards.
        try:
            points, scenario_failures = parse_scenario(content, input_args.name_suffix)
        except (KeyError, TypeError, ZeroDivisionError) as err:
            failures.append((scenario, f"failed to parse artifact: {err!r}"))
            continue
        parsed_results.extend(points)
        failures.extend(scenario_failures)

    expected = [
        scenario.strip()
        for scenario in input_args.expected_scenarios.split(",")
        if scenario.strip()
    ]
    for scenario in expected:
        if scenario not in runs:
            failures.append((scenario, "no reportable artifact for expected scenario"))

    # Writing an empty series would let the workflow publish "results" holding no
    # measurement at all; leave no file behind so the upload and send steps skip.
    if not parsed_results:
        print("\nNo data point could be parsed")
        print("-----------------------------")
        for name, error in failures:
            print(f"[{name}] {error}")
        sys.exit(1)

    output_file = pathlib.Path(input_args.output_file)
    print(f"Dump parsed results into '{output_file.resolve()}' ... ", end="")
    dump_results(parsed_results, output_file, input_args)
    print("Done")

    if failures:
        print("\nParsing failed for some results")
        print("-------------------------------")
        for name, error in failures:
            print(f"[{name}] {error}")
        sys.exit(1)


if __name__ == "__main__":
    args = parser.parse_args()
    check_mandatory_args(args)
    main(args)
