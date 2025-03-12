#!/usr/bin/env python3

import os
import json
import re
import shutil
import subprocess
import tempfile
import sys
from argparse import ArgumentParser
from enum import Enum
from pathlib import Path

GWL2_CRATE_DIR = Path(os.path.dirname(__file__))
GWL2_ROOT_DIR = GWL2_CRATE_DIR.parent
GWL2_CONTRACTS_DIR = GWL2_ROOT_DIR.joinpath("contracts")


def init_cli() -> ArgumentParser:
    """Inits the CLI of the tool."""
    parser = ArgumentParser(
        description=(
            "A tool to check or update the ABI files and version of the crate."
        )
    )
    subparsers = parser.add_subparsers(dest="command", help="Subcommands")

    subparsers.add_parser(
        "check",
        help="Check if the ABI files or the crate version need to be updated.",
    )
    subparsers.add_parser(
        "update", help="Update the ABI files and crate version."
    )

    return parser


def main():
    cli = init_cli()
    args = cli.parse_args()

    if args.command not in ["check", "update"]:
        return cli.print_help()

    abi_updater = AbiUpdater()
    abi_updater.compile_abi()

    if args.command == "check":
        check_versions()
        abi_updater.check_abi_up_to_date()
    elif args.command == "update":
        update_crate_version()
        abi_updater.update_abi_files()


class ExitStatus(Enum):
    """An enum representing the different exit status of the tool."""

    SOLC_COMPILER_NOT_INSTALLED = 1
    CRATE_VERSION_NOT_UP_TO_DATE = 2
    ABI_FILES_NOT_UP_TO_DATE = 3


class AbiUpdater:
    """
    An object used to check if ABI of the Gateway L2 contracts are up to date.

    Also takes care of updating these ABI files if requested.
    """

    tempdir: str

    def __init__(self):
        self.tempdir = tempfile.mkdtemp()

    def _check_solc_installed():
        """Checks if the solc compiler is installed."""
        path = shutil.which("solc")
        if path is None:
            log_error("ERROR: solc is not installed.")
            sys.exit(ExitStatus.SOLC_COMPILER_NOT_INSTALLED.value)

    def compile_abi(self) -> subprocess.CompletedProcess:
        """Compiles the ABI of the Gateway L2 contracts."""
        AbiUpdater._check_solc_installed()

        log_info("Compiling ABI of Gateway L2 contracts...")
        solc_command = (
            f"solc --base-path '{GWL2_ROOT_DIR}' "
            f"--include-path '{GWL2_ROOT_DIR}/node_modules' "
            f"--abi '{GWL2_CONTRACTS_DIR}/'*.sol -o '{self.tempdir}'"
        )
        result = subprocess.run(
            solc_command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            shell=True,
            text=True,
        )
        if result.returncode != 0:
            log_error(f"solc command '{solc_command}' failed:")
            log_error(f"stdout:\n{result.stdout}")
            log_error(f"stderr:\n{result.stderr}")
            sys.exit(result.returncode)

        log_success("ABI files successfully compiled!\n")

    def _is_abi_up_to_date(self, temp_abi_file: Path):
        """
        Checks if an ABI file is up-to-date with the newly compiled ABI file.
        """
        abi_file = f"{GWL2_CRATE_DIR}/abi/{temp_abi_file.name}"
        if os.path.isfile(abi_file):
            return_code = subprocess.call(
                f"diff {temp_abi_file} {abi_file}",
                shell=True,
                stdout=subprocess.DEVNULL,
            )
            return return_code == 0
        return False

    def check_abi_up_to_date(self):
        """Checks that the ABI of the Gateway L2 contracts are up-to-date."""
        log_info("Checking that the Gateway L2 ABI files are up-to-date...")
        temp_abi_files = Path(self.tempdir).glob("*.abi")
        current_abi_files = Path(f"{GWL2_CRATE_DIR}/abi").glob("*.abi")

        # Check for updated ABI files
        outdated_abi_files = list(
            filter(
                lambda file: not self._is_abi_up_to_date(file), temp_abi_files
            )
        )
        unused_abi_files = list(
            filter(
                lambda file: not os.path.isfile(f"{self.tempdir}/{file.name}"),
                current_abi_files,
            )
        )

        if len(outdated_abi_files) > 0 or len(unused_abi_files) > 0:
            log_error("ERROR: Some ABI files are outdated:")
            for file in outdated_abi_files:
                log_error(f"- {GWL2_CRATE_DIR}/abi/{file.name}")
            for file in unused_abi_files:
                log_error(f"- {file}")
            print()
            log_info("Run `./abi_update.py update` to update these files.")
            sys.exit(ExitStatus.ABI_FILES_NOT_UP_TO_DATE.value)

        log_success("All ABI files are up-to-date!")

    def update_abi_files(self):
        """Updates the ABI files of the Gateway L2 contracts."""
        log_info("Updating ABI files when required...")
        temp_abi_files = Path(self.tempdir).glob("*.abi")

        # Update modified ABI files
        for temp_abi_file in temp_abi_files:
            if not self._is_abi_up_to_date(temp_abi_file):
                current_abi_file = f"{GWL2_CRATE_DIR}/abi/{temp_abi_file.name}"
                shutil.copy(temp_abi_file, current_abi_file)
                log_success(f"- ABI file {current_abi_file} has been updated!")

        # Remove ABI files that are no longer used
        current_abi_files = Path(f"{GWL2_CRATE_DIR}/abi").glob("*.abi")
        unused_abi_files = filter(
            lambda file: not os.path.isfile(f"{self.tempdir}/{file.name}"),
            current_abi_files,
        )
        for unused_abi_file in unused_abi_files:
            os.remove(unused_abi_file)
            log_success(f"- ABI file {unused_abi_file.name} has been deleted!")
        log_success("All ABI files are now up-to-date!")

    def __del__(self):
        shutil.rmtree(self.tempdir)


def get_latest_tag_on_main():
    """Returns the latest tag on the main branch of the repo."""
    git_tag_result = subprocess.run(
        "git tag --merged main | tail -n 1", shell=True, stdout=subprocess.PIPE
    )
    return git_tag_result.stdout.decode().strip()


def check_versions():
    """
    Checks versions of the Gateway L2 and the crate match the latest git tag
    on main.
    """
    log_info(
        "Checking the latest git tag on main match the Gateway L2 version..."
    )
    latest_git_tag = get_latest_tag_on_main()

    with open(f"{GWL2_ROOT_DIR}/package.json", "r") as package_json_fd:
        package_json_content = json.load(package_json_fd)
        package_json_version = package_json_content["version"]

    if len(latest_git_tag) > 0 and package_json_version != latest_git_tag[1:]:
        log_warning(
            "WARNING: `package.json` version doesn't match main branch's tag!"
        )
        log_warning(
            f"Main branch tag is '{latest_git_tag}', so "
            f"`package.json` version should be '{latest_git_tag[1:]}' instead"
            f" of '{package_json_version}'"
        )

    log_info(
        "Checking that the crate's version match the Gateway L2 version..."
    )
    with open(f"{GWL2_CRATE_DIR}/Cargo.toml", "r") as cargo_toml_fd:
        cargo_toml_content = cargo_toml_fd.read()
        cargo_toml_version = re.findall(
            'version = "([\\d.]+)"',
            cargo_toml_content,
        )[0]

    if package_json_version != cargo_toml_version:
        log_error(
            "ERROR: Cargo.toml version does not match Gateway L2 version!\n"
        )
        log_info("Run `./abi_update.py update` to update the crate's version.")
        sys.exit(ExitStatus.CRATE_VERSION_NOT_UP_TO_DATE.value)
    log_success(
        "The version of the crate match with the Gateway L2 version!\n"
    )


def update_crate_version():
    """Updates the crate's version to match with the Gateway L2 version."""
    log_info("Updating the crate's version...")

    with open(f"{GWL2_ROOT_DIR}/package.json", "r") as package_json_fd:
        package_json_content = json.load(package_json_fd)

    with open(f"{GWL2_CRATE_DIR}/Cargo.toml", "r") as cargo_toml_fd:
        cargo_toml_content = cargo_toml_fd.read()

    cargo_toml_content = re.sub(
        'version = "([\\d.]+)"',
        'version = "{}"'.format(package_json_content["version"]),
        cargo_toml_content,
        count=1,
    )

    with open(f"{GWL2_CRATE_DIR}/Cargo.toml", "w") as cargo_toml_fd:
        cargo_toml_fd.write(cargo_toml_content)

    log_success("The crate's version has been successfully updated!\n")


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
