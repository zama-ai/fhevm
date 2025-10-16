#!/usr/bin/env python3
from dataclasses import dataclass
import os
import tomllib

SCRIPT_DIR_PATH = os.path.dirname(os.path.abspath(__file__))
COPRO_TOOL_DIR_PATH = (
    f"{SCRIPT_DIR_PATH}/../../../coprocessor/fhevm-engine/stress-test-generator"
)


@dataclass
class CiphertextHandle:
    handle: str
    digest64: str
    digest128: str

    @classmethod
    def from_line(cls, line: str):
        return cls(*line.strip().split(" "))


def generate_handle_with_copro_tool():
    _ = os.system(f"""
        cd {COPRO_TOOL_DIR_PATH} &&
        rm -f data/handles*;
        EVGEN_SCENARIO=data/minitest_003_generate_handles_for_decryption.csv make run;
        cd -
    """)


def edit_handles_in_config():
    with open(f"{COPRO_TOOL_DIR_PATH}/data/handles_for_usr_decryption") as usr_ct_file:
        usr_ct_handles = [
            CiphertextHandle.from_line(line) for line in usr_ct_file.readlines()
        ]
        euint64_pub_ct = usr_ct_handles[5]
    with open(f"{COPRO_TOOL_DIR_PATH}/data/handles_for_pub_decryption") as pub_ct_file:
        pub_ct_handles = [
            CiphertextHandle.from_line(line) for line in pub_ct_file.readlines()
        ]
        euint64_usr_ct = pub_ct_handles[5]
    print("euint64_pub_ct:", euint64_pub_ct)
    print("euint64_usr_ct:", euint64_usr_ct)

    with open(f"{SCRIPT_DIR_PATH}/../config/config.toml", "rb") as config_file:
        parsed_config = tomllib.load(config_file)

    with open(f"{SCRIPT_DIR_PATH}/../config/config.toml") as config_file:
        new_config = (
            config_file.read()
            .replace(parsed_config["user_ct"][0]["handle"], euint64_usr_ct.handle)
            .replace(parsed_config["user_ct"][0]["digest"], euint64_usr_ct.digest128)
            .replace(parsed_config["public_ct"][0]["handle"], euint64_pub_ct.handle)
            .replace(parsed_config["public_ct"][0]["digest"], euint64_pub_ct.digest128)
        )

    with open(f"{SCRIPT_DIR_PATH}/../config/config.toml", "w+") as config_file:
        _ = config_file.write(new_config)


def main():
    generate_handle_with_copro_tool()
    edit_handles_in_config()


if __name__ == "__main__":
    main()
