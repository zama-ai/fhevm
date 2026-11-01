"""Render preview listeners and verify per-chain executor proxy wiring."""

import os
import subprocess
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parent.parent
CHART = ROOT / "charts/listener"
PREVIEW_LISTENERS = ROOT / "ci/preview-env/listener"
PROXY_ENV_NAME = "APP_BLOCKCHAIN__STRATEGY__PUBLISH__TFHE_EXECUTOR_PROXY_ADDRESS"
PROXY_CONFIG_KEY = "fhevm_executor.address"
HELM_BIN = os.environ.get("HELM_BIN", "helm")


def render_listener(values_path: Path, release: str) -> list[dict]:
    result = subprocess.run(
        [
            HELM_BIN,
            "template",
            release,
            str(CHART),
            "--namespace",
            "fhevm-e2e",
            "--values",
            str(values_path),
            "--set-string",
            "env[0].value=postgresql://preview.invalid/listener",
            "--set-string",
            "env[1].value=redis://preview.invalid:6379",
        ],
        check=True,
        capture_output=True,
        text=True,
        timeout=60,
    )
    return [
        document
        for document in yaml.safe_load_all(result.stdout)
        if isinstance(document, dict) and document.get("kind") == "Deployment"
    ]


def assert_proxy_env(deployments: list[dict], expected_config_map: str) -> None:
    containers = [
        container
        for deployment in deployments
        for container in deployment["spec"]["template"]["spec"]["containers"]
        if container.get("name") == "listener"
    ]
    if not containers:
        raise AssertionError("Helm render produced no listener container")

    for container in containers:
        matches = [
            entry
            for entry in container.get("env", [])
            if entry.get("name") == PROXY_ENV_NAME
        ]
        if len(matches) != 1:
            raise AssertionError(
                f"expected one {PROXY_ENV_NAME} entry, got {len(matches)}"
            )
        reference = matches[0].get("valueFrom", {}).get("configMapKeyRef", {})
        if reference != {"name": expected_config_map, "key": PROXY_CONFIG_KEY}:
            raise AssertionError(
                f"{PROXY_ENV_NAME} must reference "
                f"{expected_config_map}/{PROXY_CONFIG_KEY}, got {reference}"
            )


def main() -> None:
    cases = (
        ("values-listener-e2e.yaml", "listener-host", "host-sc-addresses"),
        (
            "values-listener-polygon-e2e.yaml",
            "listener-polygon",
            "polygon-sc-addresses",
        ),
    )
    for values_name, release, expected_config_map in cases:
        deployments = render_listener(PREVIEW_LISTENERS / values_name, release)
        assert_proxy_env(deployments, expected_config_map)
        print(f"verified {values_name}: {expected_config_map}/{PROXY_CONFIG_KEY}")


if __name__ == "__main__":
    main()
