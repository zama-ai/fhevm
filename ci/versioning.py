#!/usr/bin/env python3

import json
import os
import re
import subprocess
from argparse import ArgumentParser
from pathlib import Path

FHEVM_ROOT_DIR = Path(os.path.dirname(__file__)).parent
COPRO_DIR = FHEVM_ROOT_DIR.joinpath("coprocessor")
GW_CONTRACTS_DIR = FHEVM_ROOT_DIR.joinpath("gateway-contracts")
HOST_CONTRACTS_DIR = FHEVM_ROOT_DIR.joinpath("host-contracts")
KMS_CONNECTOR_DIR = FHEVM_ROOT_DIR.joinpath("kms-connector")
LIB_SOLIDITY_DIR = FHEVM_ROOT_DIR.joinpath("library-solidity")
RUST_SDK_DIR = FHEVM_ROOT_DIR.joinpath("sdk").joinpath("rust-sdk")
TEST_SUITE_DIR = FHEVM_ROOT_DIR.joinpath("test-suite")

FHEVM_SUBPROJECTS_VERSION_FILES = [
    COPRO_DIR.joinpath("fhevm-engine").joinpath("Cargo.toml"),
    GW_CONTRACTS_DIR.joinpath("package.json"),
    GW_CONTRACTS_DIR.joinpath("rust_bindings").joinpath("Cargo.toml"),
    HOST_CONTRACTS_DIR.joinpath("package.json"),
    KMS_CONNECTOR_DIR.joinpath("Cargo.toml"),
    LIB_SOLIDITY_DIR.joinpath("package.json"),
    RUST_SDK_DIR.joinpath("Cargo.toml"),
    TEST_SUITE_DIR.joinpath("gateway-stress").joinpath("Cargo.toml"),
    TEST_SUITE_DIR.joinpath("e2e").joinpath("package.json"),
]


def init_cli() -> ArgumentParser:
    """Inits the CLI of the tool."""
    parser = ArgumentParser(
        description=(
            "A tool to check or update the versions within the FHEVM monorepo."
        )
    )
    subparsers = parser.add_subparsers(dest="command", help="Subcommands")

    check_subparser = subparsers.add_parser(
        "check",
        help=(
            "Check if all projects' versions within the FHEVM monorepo matches a given version."
        ),
    )
    check_subparser.add_argument(
        "version", type=str, help="The version that all FHEVM sub-projects should have."
    )

    update_subparser = subparsers.add_parser(
        "update", help="Update the projects' versions within the FHEVM monorepo."
    )
    update_subparser.add_argument(
        "version", type=str, help="The version to set for all FHEVM sub-projects."
    )

    return parser


def main():
    cli = init_cli()
    args = cli.parse_args()

    if args.command not in ["check", "update"]:
        return cli.print_help()

    version = args.version.strip("v")
    if args.command == "check":
        check_semver_string(version, False)
        check_projects_versions(version)
    elif args.command == "update":
        check_semver_string(version, True)
        update_projects_versions(version)


def check_projects_versions(fhevm_version: str):
    """
    Checks that the version of all the projects of the monorepo matches the FHEVM version.
    """
    check_results = [
        check_project_version(version_file, fhevm_version)
        for version_file in FHEVM_SUBPROJECTS_VERSION_FILES
    ]
    if all(check_results):
        log_success("All versions are up-to-date!")
    else:
        log_info(
            f"Run `./ci/versioning.py update {fhevm_version}` to update all FHEVM sub-projects versions."
        )
        exit(1)


def check_project_version(project_version_file: Path, fhevm_version: str) -> bool:
    """
    Checks that the version of a project of the monorepo matches the FHEVM version.
    """
    log_info(
        f"Checking that {project_version_file} version matches the FHEVM version..."
    )
    if project_version_file.name == "Cargo.toml":
        project_version = get_cargo_toml_version(project_version_file)
    elif project_version_file.name == "package.json":
        project_version = get_package_json_version(project_version_file)
    else:
        log_error(f"Unsupported version file: {project_version_file}!")
        return False

    if project_version == fhevm_version:
        log_success(
            f"{project_version_file}'s version matches with the FHEVM version: {fhevm_version}!\n"
        )
        return True

    log_error(
        f"ERROR: {project_version_file} version does not match FHEVM version!\n"
        f"FHEVM version: {fhevm_version}\n"
        f"{project_version_file} version: {project_version}\n"
    )
    return False


def get_cargo_toml_version(cargo_toml_path: Path) -> str | None:
    """Gets the version within a given Cargo.toml file."""
    with open(cargo_toml_path, "r") as cargo_toml_fd:
        cargo_toml_content = cargo_toml_fd.read()

        # Find the version in the Cargo.toml
        # Here, we want to find the version in the [package] section to avoid catching versions
        # from dependencies. The `re.DOTALL` flag is used to allow the dot to match newlines.
        # There is only one captured group: the version found within the quotes
        matches = re.search(
            r'\[.*package\].*?version\s*=\s*"([^"]+)"',
            cargo_toml_content,
            flags=re.DOTALL,
        )

        if not matches:
            log_error(f"Could not find version in {cargo_toml_path}")
            return None

        # Extract the version from the matches: the first (and only) captured group from the regex.
        cargo_toml_version = matches.group(1)
        return cargo_toml_version


def get_package_json_version(package_json_path: Path) -> str | None:
    """Gets the version within a given package.json file."""
    with open(package_json_path, "r") as package_json_fd:
        return json.load(package_json_fd)["version"]


def update_projects_versions(fhevm_version: str):
    """Updates the version of all the monorepo projects to `fhevm_version`."""
    update_results = [
        update_project_version(version_file, fhevm_version)
        for version_file in FHEVM_SUBPROJECTS_VERSION_FILES
    ]
    if all(update_results):
        log_success("All versions were successfully updated!")
    else:
        exit(1)


def update_project_version(project_version_file: Path, fhevm_version: str) -> bool:
    """Updates the version in the project version file."""
    log_info(f"Updating the {project_version_file}'s version...")

    if project_version_file.name == "Cargo.toml":
        update_cargo_toml_version(project_version_file, fhevm_version)
        # Update version in lockfile
        subprocess.run(
            ["cargo", "generate-lockfile", "--offline"],
            capture_output=True,
            cwd=project_version_file.parent,
        )
    elif project_version_file.name == "package.json":
        update_package_json_version(project_version_file, fhevm_version)
        lock_file = project_version_file.parent.joinpath("package-lock.json")
        if lock_file.exists():
            update_package_json_version(lock_file, fhevm_version)
    else:
        log_error(f"Unsupported version file: {project_version_file}!")
        return False

    log_success(
        f"The {project_version_file}'s version has been successfully updated to {fhevm_version}!\n"
    )
    return True


def update_cargo_toml_version(cargo_toml_path: Path, fhevm_version: str):
    """Updates the version in the Cargo.toml file."""
    with open(cargo_toml_path, "r") as cargo_toml_fd:
        cargo_toml_content = cargo_toml_fd.read()

    # Replace the version in the Cargo.toml
    # Similar to the check_version function, we use a regex to find the version in the [package]
    # section to avoid changing the version of any dependency. The `count=1` argument ensures that
    # only the first occurrence is replaced as we only expect one version. The `re.DOTALL` flag is
    # used to allow the dot to match newlines. There are two captured groups:
    # - The first one is the [package] section up until the first quote of the version.
    # - The second one is the ending quote of the version.
    # We then only replace the version by inserting it between both captured groups. This is to
    # make sure we do not alter the original format of the Cargo.toml.
    cargo_toml_content = re.sub(
        r'(\[.*package\].*?version\s*=\s*")[^"]+(")',
        lambda m: m.group(1) + fhevm_version + m.group(2),
        cargo_toml_content,
        count=1,
        flags=re.DOTALL,
    )

    with open(cargo_toml_path, "w") as cargo_toml_fd:
        cargo_toml_fd.write(cargo_toml_content)


def update_package_json_version(package_json_path: Path, fhevm_version: str):
    """Updates the version in the package.json file."""
    with open(package_json_path, "r") as package_json_fd:
        package_json_content = json.load(package_json_fd)
        package_json_content["version"] = fhevm_version

    with open(package_json_path, "w") as package_json_fd:
        json.dump(package_json_content, package_json_fd, indent=2)


def check_semver_string(string: str, strict: bool):
    """Checks the given string is a valid semver expression.

    If `strict` is set and the string is not a valid semver expression, it exits the program.
    Else, it just raises a warning.
    """
    # Official semver regex: https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
    match = re.match(
        r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
        string,
    )

    if match:
        log_success(f"{string} is a valid semver expression\n")
    elif not match and strict:
        log_error(f"{string} is not a valid semver expression!")
        exit(1)
    else:
        log_warning(f"{string} is not a valid semver expression!\n")


BRED = "\033[91m\033[1m"
BGREEN = "\033[92m\033[1m"
BYELLOW = "\033[93m\033[1m"
BBLUE = "\033[94m\033[1m"
NC = "\033[0m"


def log_info(msg: str):
    print(f"{BBLUE}[*]{NC} {msg}")


def log_success(msg: str):
    print(f"{BGREEN}[+]{NC} {msg}")


def log_error(msg: str):
    print(f"{BRED}[-]{NC} {msg}")


def log_warning(msg: str):
    print(f"{BYELLOW}[!]{NC} {msg}")


if __name__ == "__main__":
    main()
