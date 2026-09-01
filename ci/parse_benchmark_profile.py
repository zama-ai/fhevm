#!/usr/bin/env python3

import argparse
import sys
import tomllib


BACKEND_MAP = {
    "hyperstack": "hyperstack",
    "scaleway": "terraform",
}


def parse_profile(profile_string, output_file, slab_toml):
    parts = profile_string.split("::", 1)
    if len(parts) != 2:
        sys.exit(
            f"Invalid profile format '{profile_string}'; expected "
            "'provider::profile-name (hardware-name)'"
        )

    provider, profile_and_hardware = parts
    split_profile = profile_and_hardware.split()
    if len(split_profile) != 2:
        sys.exit(
            f"Invalid profile format '{profile_and_hardware}'; expected "
            "'provider::profile-name (hardware-name)'"
        )

    profile_name = split_profile[0]
    hardware_name = split_profile[1].removeprefix("(").removesuffix(")")

    if provider not in BACKEND_MAP:
        known_providers = ", ".join(sorted(BACKEND_MAP))
        sys.exit(f"Unknown provider '{provider}'; known providers: {known_providers}")

    slab_backend = BACKEND_MAP[provider]
    profile = f"{provider}-{profile_name}" if slab_backend == "terraform" else profile_name

    with open(slab_toml, "rb") as slab_file:
        slab = tomllib.load(slab_file)

    available_profiles = slab.get("backend", {}).get(slab_backend, {})
    if profile not in available_profiles:
        known_profiles = ", ".join(sorted(available_profiles))
        sys.exit(
            f"Profile '{profile}' not found under [backend.{slab_backend}] in "
            f"{slab_toml}; known profiles: {known_profiles}"
        )

    entry = available_profiles[profile]
    expected_hardware = entry.get("flavor_name") or entry.get("instance_type")
    if expected_hardware is None:
        sys.exit(
            f"Profile '{profile}' in {slab_toml} has neither flavor_name nor instance_type"
        )
    if hardware_name != expected_hardware:
        sys.exit(
            f"Hardware '{hardware_name}' does not match expected '{expected_hardware}' "
            f"for profile '{profile}' in {slab_toml}"
        )

    with open(output_file, "a", encoding="utf-8") as output:
        for name, value in (
            ("backend", slab_backend),
            ("profile", profile),
            ("hardware", hardware_name),
            ("cloud_provider", provider),
        ):
            output.write(f"{name}={value}\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Parse and validate a GPU benchmark provider/profile string"
    )
    parser.add_argument("profile_string")
    parser.add_argument("output_file")
    parser.add_argument("--slab-toml", default="ci/slab.toml")
    arguments = parser.parse_args()
    parse_profile(arguments.profile_string, arguments.output_file, arguments.slab_toml)
