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
GW_CRATE_DIR = GW_ROOT_DIR.joinpath("httpz_gateway_rust_bindings")
GW_CONTRACTS_DIR = GW_ROOT_DIR.joinpath("contracts")

ALLOWED_FORGE_VERSIONS = ["1.0.0-v1.0.0", "1.0.0-stable"]


def init_cli() -> ArgumentParser:
    """Inits the CLI of the tool."""
    parser = ArgumentParser(
        description=(
            "A tool to check or update the bindings crate of the Gateway "
            "contracts."
        )
    )
    subparsers = parser.add_subparsers(dest="command", help="Subcommands")

    subparsers.add_parser(
        "check",
        help=(
            "Check if the binding files or the crate version need to be "
            "updated."
        ),
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
        bindings_updater.check_versions()
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

    def _check_forge_installed():
        """Checks if `forge` is installed with the required version."""
        path = shutil.which("forge")
        if path is None:
            log_error("ERROR: forge is not installed.")
            sys.exit(ExitStatus.FORGE_NOT_INSTALLED.value)

        forge_version = (
            subprocess.run(
                ["forge", "--version"], capture_output=True, text=True
            )
            .stdout.splitlines()[0]
            .lstrip("forge Version: ")
        )
        if forge_version not in ALLOWED_FORGE_VERSIONS:
            log_error(
                "ERROR: Required forge version to be one of these: "
                f"`{ALLOWED_FORGE_VERSIONS}` but '{forge_version}' is "
                "currently installed."
            )
            sys.exit(ExitStatus.WRONG_FORGE_VERSION.value)

    def check_bindings_up_to_date(self):
        """Checks that the Gateway contracts' bindings are up-to-date."""
        log_info(
            "Checking that the Gateway contracts' bindings are up-to-date..."
        )
        
        # We need to include the --no-metadata flag to avoid updating many of the contracts' bytecode
        # when only updating one of them (since interfaces are included in many contracts)
        return_code = subprocess.call(
            f"forge bind --root {GW_ROOT_DIR} --module --skip-cargo-toml "
            f"--hh -b {GW_CRATE_DIR}/src  -o {self.tempdir} --skip Example "
            f"--no-metadata",
            shell=True,
            stdout=subprocess.DEVNULL,
        )

        if return_code != 0:
            log_error("ERROR: Some binding files are outdated.")
            log_info(
                "Run `./scripts/bindings_update.py update` to update the "
                "bindings."
            )
            sys.exit(ExitStatus.BINDINGS_NOT_UP_TO_DATE.value)

        log_success("All binding files are up-to-date!")

    def update_bindings(self):
        """Updates the Gateway contracts' bindings."""
        log_info("Updating Gateway contracts' bindings...")

        # We need to include the --no-metadata flag to avoid updating many of the contracts' bytecode
        # when only updating one of them (since interfaces are included in many contracts)
        subprocess.run(
            f"forge bind --root {GW_ROOT_DIR} --hh -b {GW_CRATE_DIR}/src "
            f"--module --overwrite -o {self.tempdir} --skip Example "
            "--no-metadata",
            shell=True,
            check=True,
            stdout=subprocess.DEVNULL,
        )

        log_success("The Gateway contracts' bindings are now up-to-date!")

    def check_versions(self):
        """
        Checks versions of the Gateway and the crate match the latest git tag
        on main.
        """
        log_info(
            "Checking the latest git tag on main match the Gateway version..."
        )
        latest_git_tag = get_latest_tag_on_main()

        if (
            len(latest_git_tag) > 0
            and self.gateway_repo_version != latest_git_tag[1:]
        ):
            log_warning(
                "WARNING: `package.json` version doesn't match main branch's "
                "tag!"
            )
            log_warning(
                f"Main branch tag is '{latest_git_tag}', so `package.json` "
                f"version should be '{latest_git_tag[1:]}' instead"
                f" of '{self.gateway_repo_version}'"
            )

        log_info(
            "Checking that the crate's version match the Gateway version..."
        )
        with open(f"{GW_CRATE_DIR}/Cargo.toml", "r") as cargo_toml_fd:
            cargo_toml_content = cargo_toml_fd.read()
            cargo_toml_version = re.findall(
                'version = "([\\d.]+)"',
                cargo_toml_content,
            )[0]

        if self.gateway_repo_version != cargo_toml_version:
            log_error(
                "ERROR: Cargo.toml version does not match Gateway version!\n"
            )
            log_info(
                "Run `./bindings_update.py update` to update the crate's "
                "version."
            )
            sys.exit(ExitStatus.CRATE_VERSION_NOT_UP_TO_DATE.value)
        log_success(
            "The version of the crate match with the Gateway version!\n"
        )

    def update_crate_version(self):
        """Updates the crate's version to match with the Gateway version."""
        log_info("Updating the crate's version...")

        with open(f"{GW_CRATE_DIR}/Cargo.toml", "r") as cargo_toml_fd:
            cargo_toml_content = cargo_toml_fd.read()

        cargo_toml_content = re.sub(
            'version = "([\\d.]+)"',
            f'version = "{self.gateway_repo_version}"',
            cargo_toml_content,
            count=1,
        )

        with open(f"{GW_CRATE_DIR}/Cargo.toml", "w") as cargo_toml_fd:
            cargo_toml_fd.write(cargo_toml_content)

        log_success("The crate's version has been successfully updated!\n")


def get_latest_tag_on_main():
    """Returns the latest tag on the main branch of the repo."""
    git_tag_result = subprocess.run(
        "git tag --merged main | tail -n 1", shell=True, stdout=subprocess.PIPE
    )
    return git_tag_result.stdout.decode().strip()


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
