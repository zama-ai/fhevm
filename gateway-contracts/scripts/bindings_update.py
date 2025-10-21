#!/usr/bin/env python3

import os
import json
import re
import shutil
import subprocess
import sys
import tempfile
from argparse import ArgumentParser
from enum import Enum
from pathlib import Path

GW_ROOT_DIR = Path(os.path.dirname(__file__)).parent
GW_CRATE_DIR = GW_ROOT_DIR.joinpath("rust_bindings")
GW_CONTRACTS_DIR = GW_ROOT_DIR.joinpath("contracts")
GW_MOCKS_DIR = GW_CONTRACTS_DIR.joinpath("mocks")

# To update forge to the latest version locally, run `foundryup`
ALLOWED_FORGE_VERSIONS = ["1.3.1-v1.3.1", "1.3.1-stable", "1.3.2-stable", "1.4.2-stable"]


def init_cli() -> ArgumentParser:
    """Inits the CLI of the tool."""
    parser = ArgumentParser(
        description=(
            "A tool to check or update the bindings crate of the Gateway contracts."
        )
    )
    subparsers = parser.add_subparsers(dest="command", help="Subcommands")

    subparsers.add_parser(
        "check",
        help=("Check if the binding files or the crate version need to be updated."),
    )
    subparsers.add_parser(
        "update", help="Update the binding files and the crate version."
    )

    return parser


def main():
    cli = init_cli()
    args = cli.parse_args()

    if args.command not in ["check", "update"]:
        return cli.print_help()

    bindings_updater = BindingsUpdater()

    if args.command == "check":
        bindings_updater.check_version()
        bindings_updater.check_bindings_up_to_date()
    elif args.command == "update":
        bindings_updater.update_crate_version()
        bindings_updater.update_bindings()


class ExitStatus(Enum):
    """An enum representing the different exit status of the tool."""

    FORGE_NOT_INSTALLED = 1
    WRONG_FORGE_VERSION = 2
    CRATE_VERSION_NOT_UP_TO_DATE = 3
    BINDINGS_NOT_UP_TO_DATE = 4


class BindingsUpdater:
    """
    An object used to check if the binding crate of the Gateway contracts is
    up-to-date.

    Also takes care of updating this crate if requested.
    """

    tempdir: str
    gateway_repo_version: str

    def __init__(self):
        self.tempdir = tempfile.mkdtemp()
        BindingsUpdater._check_forge_installed()
        with open(f"{GW_ROOT_DIR}/package.json", "r") as package_json_fd:
            package_json_content = json.load(package_json_fd)
            self.gateway_repo_version = package_json_content["version"]

    def __del__(self):
        shutil.rmtree(self.tempdir)

    @staticmethod
    def _check_forge_installed():
        """Checks if `forge` is installed with the required version."""
        path = shutil.which("forge")
        if path is None:
            log_error("ERROR: forge is not installed.")
            sys.exit(ExitStatus.FORGE_NOT_INSTALLED.value)

        print("Check forge is installed, waiting for response...")
        forge_version = (
            subprocess.run(
                ["forge", "--version"],
                capture_output=True,
                text=True,
            )
            .stdout.splitlines()[0]
            .lstrip("forge Version: ")
        )
        print("Forge version:", forge_version)
        if forge_version not in ALLOWED_FORGE_VERSIONS:
            log_error(
                "ERROR: Required forge version to be one of these: "
                f"`{ALLOWED_FORGE_VERSIONS}` but '{forge_version}' is "
                "currently installed."
            )
            sys.exit(ExitStatus.WRONG_FORGE_VERSION.value)

    def check_bindings_up_to_date(self):
        """Checks that the Gateway contracts' bindings are up-to-date."""
        log_info("Checking that the Gateway contracts' bindings are up-to-date...")

        # We need to include the --no-metadata flag to avoid updating many of the contracts' bytecode
        # when only updating one of them (since interfaces are included in many contracts)
        return_code = subprocess.call(
            f"forge bind --root {GW_ROOT_DIR} --module --skip-cargo-toml "
            f"--hh -b {GW_CRATE_DIR}/src  -o {self.tempdir} --skip Example --skip {GW_MOCKS_DIR}/* "
            f"--no-metadata",
            shell=True,
            stdout=subprocess.DEVNULL,
        )

        if return_code != 0:
            log_error("ERROR: Some binding files are outdated.")
            log_info("Run `make update-bindings` to update the bindings.")
            sys.exit(ExitStatus.BINDINGS_NOT_UP_TO_DATE.value)

        log_success("All binding files are up-to-date!")

    def update_bindings(self):
        """Updates the Gateway contracts' bindings."""
        log_info("Updating Gateway contracts' bindings...")

        # We need to include the --no-metadata flag to avoid updating many of the contracts' bytecode
        # when only updating one of them (since interfaces are included in many contracts)
        subprocess.run(
            f"forge bind --root {GW_ROOT_DIR} --hh -b {GW_CRATE_DIR}/src "
            f"--module --overwrite -o {self.tempdir} --skip Example --skip {GW_MOCKS_DIR}/* "
            "--no-metadata",
            shell=True,
            check=True,
            stdout=subprocess.DEVNULL,
        )

        log_success("The Gateway contracts' bindings are now up-to-date!")

    def check_version(self):
        """
        Checks that the version of the crate matches the version of the Gateway.
        """
        log_info("Checking that the crate's version match the Gateway version...")
        with open(f"{GW_CRATE_DIR}/Cargo.toml", "r") as cargo_toml_fd:
            cargo_toml_content = cargo_toml_fd.read()

            # Find the version in the Cargo.toml
            # Here, we want to find the version in the [package] section to avoid catching versions
            # from dependencies. The `re.DOTALL` flag is used to allow the dot to match newlines.
            # There is only one captured group: the version found within the quotes
            matches = re.search(
                r'\[package\].*?version\s*=\s*"([^"]+)"',
                cargo_toml_content,
                flags=re.DOTALL,
            )

            if not matches:
                log_error("Could not find version in Cargo.toml")
                sys.exit(1)

            # Extract the version from the matches: the first (and only) captured group from the regex.
            cargo_toml_version = matches.group(1)

        if self.gateway_repo_version != cargo_toml_version:
            log_error(
                "ERROR: Cargo.toml version does not match Gateway version!\n"
                f"Gateway version: {self.gateway_repo_version}\n"
                f"Cargo.toml version: {cargo_toml_version}\n"
            )
            log_info("Run `make update-bindings` to update the crate's version.")
            sys.exit(ExitStatus.CRATE_VERSION_NOT_UP_TO_DATE.value)
        log_success(
            f"The version of the crate match with the Gateway version: {self.gateway_repo_version}!\n"
        )

    def update_crate_version(self):
        """Updates the crate's version to match with the Gateway version."""
        log_info("Updating the crate's version...")

        with open(f"{GW_CRATE_DIR}/Cargo.toml", "r") as cargo_toml_fd:
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
            r'(\[package\].*?version\s*=\s*")[^"]+(")',
            lambda m: m.group(1) + self.gateway_repo_version + m.group(2),
            cargo_toml_content,
            count=1,
            flags=re.DOTALL,
        )

        with open(f"{GW_CRATE_DIR}/Cargo.toml", "w") as cargo_toml_fd:
            cargo_toml_fd.write(cargo_toml_content)

        log_success(
            f"The crate's version has been successfully updated to "
            f"{self.gateway_repo_version}!\n"
        )


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
