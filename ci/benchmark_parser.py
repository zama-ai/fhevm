"""
benchmark_parser
----------------

Parse criterion benchmark or keys size results.
"""

import argparse
import csv
import enum
import json
import pathlib
import sys

ONE_HOUR_IN_SECONDS = 3600
ONE_SECOND_IN_NANOSECONDS = 1e9

parser = argparse.ArgumentParser()
parser.add_argument(
    "results",
    help="Location of criterion benchmark results directory."
    "If the --key-size option is used, then the value would have to point to"
    "a CSV file.",
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
    "--append-results",
    dest="append_results",
    action="store_true",
    help="Append parsed results to an existing file",
)
parser.add_argument(
    "--walk-subdirs",
    dest="walk_subdirs",
    action="store_true",
    help="Check for results in subdirectories",
)
parser.add_argument(
    "--object-sizes",
    dest="object_sizes",
    action="store_true",
    help="Parse only the results regarding keys size measurements",
)
parser.add_argument(
    "--key-gen",
    dest="key_gen",
    action="store_true",
    help="Parse only the results regarding keys generation time measurements",
)
parser.add_argument(
    "--bench-type",
    dest="bench_type",
    choices=["latency", "throughput"],
    default="latency",
    help="Compute and append number of operations per second and"
    "operations per dollar",
)
parser.add_argument(
    "--backend",
    dest="backend",
    default="cpu",
    help="Backend on which benchmarks have run",
)
parser.add_argument(
    "--crate",
    dest="crate",
    default="fhevm-engine/coprocessor",
    help="Crate for which benchmarks have run",
)


class BenchType(enum.Enum):
    """
    Type of benchmarks performed
    """

    latency = 1
    throughput = 2


def recursive_parse(
    directory,
    crate,
    bench_type,
    walk_subdirs=False,
    name_suffix="",
    hardware_hourly_cost=None,
):
    """
    Parse all the benchmark results in a directory. It will attempt to parse all the files having a
    .json extension at the top-level of this directory.

    :param directory: path to directory that contains raw results as :class:`pathlib.Path`
    :param crate: the name of the crate that has been benched
    :param bench_type: type of benchmark performed as :class:`BenchType`
    :param walk_subdirs: traverse results subdirectories if parameters changes for benchmark case.
    :param name_suffix: a :class:`str` suffix to apply to each test name found
    :param hardware_hourly_cost: hourly cost of the hardware used in dollar

    :return: tuple of :class:`list` as (data points, parsing failures)
    """
    excluded_directories = ["child_generate", "fork", "parent_generate", "report"]
    result_values = []
    parsing_failures = []
    bench_class = "evaluate"

    for dire in directory.iterdir():
        if dire.name in excluded_directories or not dire.is_dir():
            continue
        for subdir in dire.iterdir():
            if walk_subdirs:
                if subdir.name == "new":
                    pass
                else:
                    subdir = subdir.joinpath("new")
                    if not subdir.exists():
                        continue
            elif subdir.name != "new":
                continue

            full_name, test_name, elements = parse_benchmark_file(subdir)

            if bench_type == BenchType.throughput and elements is None:
                # Current subdir contains only latency measurements
                continue

            if test_name is None:
                parsing_failures.append(
                    (full_name, "'function_id' field is null in report")
                )
                continue

            try:
                params, display_name, operator = get_parameters(test_name, crate)
            except Exception as err:
                parsing_failures.append((full_name, f"failed to get parameters: {err}"))
                continue

            for stat_name, value in parse_estimate_file(subdir).items():
                test_name_parts = list(
                    filter(None, [test_name, stat_name, name_suffix])
                )

                if stat_name == "mean" and bench_type == BenchType.throughput:
                    value = (elements * ONE_SECOND_IN_NANOSECONDS) / value

                result_values.append(
                    _create_point(
                        value,
                        "_".join(test_name_parts),
                        bench_class,
                        bench_type.name,
                        operator,
                        params,
                        display_name=display_name,
                    )
                )

                lowercase_test_name = test_name.lower()
                # This is a special case where PBS are blasted as vector LWE ciphertext with
                # variable length to saturate the machine. To get the actual throughput we need to
                # multiply by the length of the vector.
                if (
                    "pbs_throughput" in lowercase_test_name
                    and lowercase_test_name.endswith("chunk")
                ):
                    try:
                        multiplier = int(
                            lowercase_test_name.strip("chunk").split("::")[-1]
                        )
                    except ValueError:
                        parsing_failures.append(
                            (full_name, "failed to extract throughput multiplier")
                        )
                        continue
                else:
                    multiplier = 1

                if (
                    stat_name == "mean"
                    and bench_type == BenchType.throughput
                    and hardware_hourly_cost is not None
                ):
                    test_suffix = "ops-per-dollar"
                    test_name_parts.append(test_suffix)
                    result_values.append(
                        _create_point(
                            multiplier
                            * compute_ops_per_dollar(value, hardware_hourly_cost),
                            "_".join(test_name_parts),
                            bench_class,
                            bench_type.name,
                            operator,
                            params,
                            display_name="_".join([display_name, test_suffix]),
                        )
                    )

    return result_values, parsing_failures


def _create_point(
    value, test_name, bench_class, bench_type, operator, params, display_name=None
):
    return {
        "value": value,
        "test": test_name,
        "name": display_name,
        "class": bench_class,
        "type": bench_type,
        "operator": operator,
        "params": params,
    }


def parse_benchmark_file(directory):
    """
    Parse file containing details of the parameters used for a benchmark.

    :param directory: directory where a benchmark case results are located as :class:`pathlib.Path`

    :return: names of the test and throughput elements as :class:`tuple` formatted as
    (:class:`str`, :class:`str`, :class:`int`)
    """
    raw_res = _parse_file_to_json(directory, "benchmark.json")
    throughput = raw_res["throughput"]
    elements = throughput.get("Elements", None) if throughput else None
    return raw_res["full_id"], raw_res["function_id"], elements


def parse_estimate_file(directory):
    """
    Parse file containing timing results for a benchmark.

    :param directory: directory where a benchmark case results are located as :class:`pathlib.Path`

    :return: :class:`dict` of data points
    """
    raw_res = _parse_file_to_json(directory, "estimates.json")
    return {
        stat_name: raw_res[stat_name]["point_estimate"]
        for stat_name in ("mean", "std_dev")
    }


def _parse_key_results(result_file, crate, bench_type):
    """
    Parse file containing results about operation on keys. The file must be formatted as CSV.

    :param result_file: results file as :class:`pathlib.Path`
    :param crate: crate for which benchmarks have run
    :param bench_type: type of benchmark as :class:`str`

    :return: tuple of :class:`list` as (data points, parsing failures)
    """
    result_values = []
    parsing_failures = []

    with result_file.open() as csv_file:
        reader = csv.reader(csv_file)
        for test_name, value in reader:
            try:
                params, display_name, operator = get_parameters(test_name, crate)
            except Exception as err:
                parsing_failures.append((test_name, f"failed to get parameters: {err}"))
                continue

            result_values.append(
                _create_point(
                    value,
                    test_name,
                    display_name,
                    "keygen",
                    bench_type,
                    operator,
                    params,
                )
            )

    return result_values, parsing_failures


def parse_object_sizes(result_file, crate):
    """
    Parse file containing key sizes results. The file must be formatted as CSV.

    :param result_file: results file as :class:`pathlib.Path`
    :param crate: crate for which benchmarks have run

    :return: tuple of :class:`list` as (data points, parsing failures)
    """
    return _parse_key_results(result_file, crate, "keysize")


def parse_key_gen_time(result_file, crate):
    """
    Parse file containing key generation time results. The file must be formatted as CSV.

    :param result_file: results file as :class:`pathlib.Path`
    :param crate: crate for which benchmarks have run

    :return: tuple of :class:`list` as (data points, parsing failures)
    """
    return _parse_key_results(result_file, crate, "latency")


def get_parameters(bench_id, directory):
    """
    Get benchmarks parameters recorded for a given benchmark case.

    :param bench_id: function name used for the benchmark case
    :param directory: directory where the parameters are stored

    :return: :class:`tuple` as ``(benchmark parameters, display name, operator type)``
    """
    params_dir = pathlib.Path(directory, "benchmarks_parameters", bench_id)
    params = _parse_file_to_json(params_dir, "parameters.json")

    display_name = params.pop("display_name")
    operator = params.pop("operator_type")

    # Put cryptographic parameters at the same level as the others parameters
    crypto_params = params.pop("crypto_parameters")
    params.update(crypto_params)

    return params, display_name, operator


def compute_ops_per_dollar(data_point, product_hourly_cost):
    """
    Compute numbers of operations per dollar for a given ``data_point``.

    :param data_point: throughput value measured during benchmark in elements per second
    :param product_hourly_cost: cost in dollar per hour of hardware used

    :return: number of operations per dollar
    """
    return ONE_HOUR_IN_SECONDS * data_point / product_hourly_cost


def compute_ops_per_second(data_point):
    """
    Compute numbers of operations per second for a given ``data_point``.

    :param data_point: timing value measured during benchmark in nanoseconds

    :return: number of operations per second
    """
    return 1e9 / data_point


def _parse_file_to_json(directory, filename):
    result_file = directory.joinpath(filename)
    return json.loads(result_file.read_text())


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
        parsed_content = json.loads(filename.read_text())
        parsed_content["points"].extend(parsed_results)
        filename.write_text(json.dumps(parsed_content))
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
        filename.write_text(json.dumps(series))


def check_mandatory_args(input_args):
    """
    Check for availability of required input arguments, the program will exit if one of them is
    not present. If `append_results` flag is set, all the required arguments will be ignored.

    :param input_args: CLI input arguments
    """
    if input_args.append_results:
        return

    missing_args = []
    for arg_name in vars(input_args):
        if arg_name in [
            "results_dir",
            "output_file",
            "name_suffix",
            "append_results",
            "walk_subdirs",
            "object_sizes",
            "key_gen",
            "bench_type",
        ]:
            continue
        if not getattr(input_args, arg_name):
            missing_args.append(arg_name)

    if missing_args:
        for arg_name in missing_args:
            print(f"Missing required argument: --{arg_name.replace('_', '-')}")
        sys.exit(1)


if __name__ == "__main__":
    args = parser.parse_args()
    check_mandatory_args(args)

    bench_type = BenchType[args.bench_type]

    failures = []
    raw_results = pathlib.Path(args.results)
    if args.object_sizes or args.key_gen:
        if args.object_sizes:
            print("Parsing key sizes results... ")
            results, failures = parse_object_sizes(raw_results, args.crate)

        if args.key_gen:
            print("Parsing key generation time results... ")
            results, failures = parse_key_gen_time(raw_results, args.crate)
    else:
        print("Parsing benchmark results... ")
        hardware_cost = None
        if bench_type == BenchType.throughput:
            print("Throughput computation enabled")
            ec2_costs = json.loads(
                pathlib.Path("ci/ec2_products_cost.json").read_text(encoding="utf-8")
            )
            try:
                hardware_cost = abs(ec2_costs[args.hardware])
                print(f"Hardware hourly cost: {hardware_cost} $/h")
            except KeyError:
                print(f"Cannot find hardware hourly cost for '{args.hardware}'")
                sys.exit(1)

        results, failures = recursive_parse(
            raw_results,
            args.crate,
            bench_type,
            args.walk_subdirs,
            args.name_suffix,
            hardware_cost,
        )

    print("Parsing results done")

    output_file = pathlib.Path(args.output_file)
    print(f"Dump parsed results into '{output_file.resolve()}' ... ", end="")
    dump_results(results, output_file, args)

    print("Done")

    if failures:
        print("\nParsing failed for some results")
        print("-------------------------------")
        for name, error in failures:
            print(f"[{name}] {error}")
        sys.exit(1)
