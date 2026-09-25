"""Render the production charts and verify the Solana deployment boundaries."""
import pathlib
import json
import re
import os
import tempfile
import subprocess
import unittest

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[3]
VALUES = ROOT / "ci/preview-env/solana-host"
ROUTES = [{"url": "http://coprocessor-1-solana-host-listener:8080", "apiKey": "$(SOLANA_PROOF_API_KEY)"}]


def render(release, chart, values, *options):
    command = ["helm", "template", release, str(ROOT / "charts" / chart)]
    for value in values:
        command += ["-f", str(value)]
    return list(yaml.safe_load_all(subprocess.check_output(command + list(options), text=True)))


class SolanaCharts(unittest.TestCase):
    def test_host_and_demos_are_separate_jobs(self):
        for release, filename, operations in [
            ("solana-host", "values-solana-programs-e2e.yaml", ["host deploy --allow-upgrade"]),
            ("solana-demos", "values-solana-demos-e2e.yaml", ["demos deploy --allow-upgrade"]),
        ]:
            documents = render(release, "contracts", [VALUES / filename])
            job = next(d for d in documents if d and d["kind"] == "Job")
            self.assertEqual(job["metadata"]["name"], release + "-deploy")
            container = job["spec"]["template"]["spec"]["containers"][0]
            self.assertEqual(container["command"], ["/app/deploy-contracts.sh"])
            config = next(d for d in documents if d and d["kind"] == "ConfigMap")
            script = config["data"]["deploy-contracts.sh"]
            positions = [script.index("node /app/cli.mjs " + operation) for operation in operations]
            self.assertEqual(positions, sorted(positions))  # deployment commands keep their declared order
            if release == "solana-host":
                self.assertFalse(any("TOKEN_KEYPAIR" in e["name"] for e in container["env"]))

    def test_listener_shares_database_and_keeps_proofs_private(self):
        documents = render("coprocessor-1", "coprocessor", [ROOT / "ci/preview-env/coprocessor/values-coprocessor-e2e.yaml",
                            VALUES / "values-solana-coprocessor-e2e.yaml"])
        name = "coprocessor-1-solana-host-listener"
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and d["metadata"]["name"] == name)
        self.assertEqual(deployment["spec"]["replicas"], 1)
        self.assertEqual(deployment["spec"]["strategy"]["type"], "Recreate")
        container = deployment["spec"]["template"]["spec"]["containers"][0]
        env = {e["name"]: e for e in container["env"]}
        self.assertEqual(env["DATABASE_URL"]["value"],
                         "postgresql://$(DATABASE_USER):$(DATABASE_PASSWORD)@$(DATABASE_ENDPOINT)/fhevm_e2e")
        names = [e["name"] for e in container["env"]]
        for variable in ["DATABASE_USER", "DATABASE_PASSWORD", "DATABASE_ENDPOINT"]:
            self.assertLess(names.index(variable), names.index("DATABASE_URL"))
        self.assertEqual(env["SOLANA_PROOF_API_KEY"]["valueFrom"]["secretKeyRef"]["name"], "solana-proof-api")
        service = next(d for d in documents if d and d["kind"] == "Service" and d["metadata"]["name"] == name)
        self.assertEqual(service["spec"]["type"], "ClusterIP")
        self.assertEqual(service["spec"]["selector"], deployment["spec"]["selector"]["matchLabels"])

    def test_listener_iam_certificate_is_a_volume_list(self):
        documents = render("coprocessor-1", "coprocessor", [VALUES / "values-solana-coprocessor-e2e.yaml"],
                           "--set", "commonConfig.databaseAuthMode=iam",
                           "--set", "commonConfig.databaseSslRootCert.enabled=true")
        deployment = next(d for d in documents if d and d["kind"] == "Deployment"
                          and d["metadata"]["name"] == "coprocessor-1-solana-host-listener")
        pod = deployment["spec"]["template"]["spec"]
        container = pod["containers"][0]
        self.assertIsInstance(pod["volumes"], list)
        self.assertIsInstance(container["volumeMounts"], list)
        self.assertGreater(container["securityContext"]["runAsUser"], 0)
        self.assertTrue(container["securityContext"]["runAsNonRoot"])
        self.assertEqual(container["volumeMounts"][0]["name"], pod["volumes"][0]["name"])
        env = {e["name"]: e for e in container["env"]}
        self.assertEqual(env["DATABASE_IAM_AUTH_ENABLED"]["value"], "true")
        self.assertIn("DATABASE_SSL_ROOT_CERT_PATH", env)

    def test_connector_preserves_evm_and_exact_solana_chain_id(self):
        # Same composition as deploy-preview.sh: the party's values, the Solana overlay, and the
        # proof routes the script sets. aclAddress is filled per party by deploy-kms-connector.sh.
        documents = render("kms-connector-1", "kms-connector",
                           [ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml",
                            VALUES / "values-solana-connector-e2e.yaml"],
                           "--set-string", "commonConfig.hostChains.ethereum.aclAddress=0x" + "11" * 20,
                           "--set-json",
                           'commonConfig.hostChains.solana.solanaProofRoutes=' + json.dumps(ROUTES))
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and "kms-worker" in d["metadata"]["name"])
        env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
        names = [e["name"] for e in env]
        self.assertLess(names.index("SOLANA_PROOF_API_KEY"), names.index("KMS_CONNECTOR_HOST_CHAINS"))
        chains = {c["chainId"]: c for c in json.loads(next(e["value"] for e in env if e["name"] == "KMS_CONNECTOR_HOST_CHAINS"))}
        self.assertEqual(chains[12345]["aclAddress"], "0x" + "11" * 20)
        solana = chains[130140237723663404]
        self.assertNotIn("aclAddress", solana)
        self.assertEqual(solana["solanaProofRoutes"], ROUTES)
        endpoint = next(d for d in documents if d and d["kind"] == "Deployment" and "endpoint" in d["metadata"]["name"])
        ids = next(e["value"] for e in endpoint["spec"]["template"]["spec"]["containers"][0]["env"]
                   if e["name"] == "KMS_CONNECTOR_SUPPORTED_CHAIN_IDS")
        self.assertIn("130140237723663404", ids.split(","))

    def test_connector_reads_the_kind_from_the_type_byte_at_any_cluster_tag(self):
        # The PoC chain id has cluster tag 12345: a wrong shift would read it as EVM.
        documents = render("kms-connector-1", "kms-connector",
                           [ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml",
                            VALUES / "values-solana-connector-e2e.yaml"],
                           "--set-string", "commonConfig.hostChains.ethereum.aclAddress=0x" + "11" * 20,
                           "--set-string", "commonConfig.hostChains.solana.chainId=72057594037940281",
                           "--set-json",
                           'commonConfig.hostChains.solana.solanaProofRoutes=' + json.dumps(ROUTES))
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and "kms-worker" in d["metadata"]["name"])
        env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
        chains = {c["chainId"]: c for c in json.loads(next(e["value"] for e in env if e["name"] == "KMS_CONNECTOR_HOST_CHAINS"))}
        self.assertEqual(chains[72057594037940281]["solanaProofRoutes"], ROUTES)

    def test_connector_refuses_an_entry_whose_settings_are_not_its_kind(self):
        # YAML reads a large unquoted number as a float, which the chart must not round.
        with tempfile.NamedTemporaryFile("w", suffix=".yaml") as unquoted:
            unquoted.write("commonConfig:\n  hostChains:\n    solana:\n      chainId: 72057594037940281\n")
            unquoted.flush()
            cases = [
                (["--set-string", "commonConfig.hostChains.solana.aclAddress=0x" + "11" * 20],
                 "solana.aclAddress does not apply to a Solana chain"),
                (["--set-string", "commonConfig.hostChains.solana.chainId=31888",
                  "--set-string", "commonConfig.hostChains.solana.aclAddress=0x" + "11" * 20],
                 "solana.solanaHostProgramId does not apply to an EVM chain"),
                (["--set-string", "commonConfig.hostChains.solana.chainId=144115188075855881"],
                 "solana.chainId has type byte 0x02, which names no host kind"),
                (["--set-string", "commonConfig.hostChains.solana.chainId=abc"],
                 'solana.chainId "abc" is not a decimal integer below 2^63'),
                (["--set-string", "commonConfig.hostChains.solana.chainId=9295429630892703744"],
                 'solana.chainId "9295429630892703744" is not a decimal integer below 2^63'),
                (["--set-string", "commonConfig.hostChains.solana.chainId=0x0100000000003039"],
                 'solana.chainId "0x0100000000003039" is not a decimal integer below 2^63'),
                (["-f", unquoted.name],
                 "quote it, as YAML reads a large unquoted number as a float"),
            ]
            for options, expected in cases:
                command = ["helm", "template", "kms-connector-1", str(ROOT / "charts/kms-connector"),
                           "-f", str(ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml"),
                           "-f", str(VALUES / "values-solana-connector-e2e.yaml"),
                           "--set-string", "commonConfig.hostChains.ethereum.aclAddress=0x" + "11" * 20,
                           "--set-json", "commonConfig.hostChains.solana.solanaProofRoutes=" + json.dumps(ROUTES),
                           *options]
                result = subprocess.run(command, capture_output=True, text=True)
                self.assertNotEqual(result.returncode, 0, expected)
                self.assertIn(expected, result.stderr)

    def test_connector_refuses_solana_settings_on_a_preset_chain(self):
        command = ["helm", "template", "kms-connector-1", str(ROOT / "charts/kms-connector"),
                   "--set", "commonConfig.network=devnet",
                   *[option for name in ["ethereum", "polygon", "bnb"] for option in
                     ["--set", f"commonConfig.hostChains.{name}.url=http://{name}:8545"]],
                   "--set", "commonConfig.hostChains.ethereum.solanaHostProgramId=" + "1" * 32]
        result = subprocess.run(command, capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ethereum.solanaHostProgramId does not apply to a preset chain", result.stderr)

    def test_program_keys_are_optional_but_deployer_is_required(self):
        for filename in ["values-solana-programs-e2e.yaml", "values-solana-demos-e2e.yaml"]:
            docs = render("solana", "contracts", [VALUES / filename])
            job = next(d for d in docs if d and d["kind"] == "Job")
            env = job["spec"]["template"]["spec"]["containers"][0]["env"]
            for entry in env:
                ref = entry.get("valueFrom", {}).get("secretKeyRef", {})
                if ref.get("name") == "solana-deployer":
                    self.assertEqual(ref.get("optional", False), ref["key"] != "deployer.json")

    def test_script_appended_env_reaches_each_job(self):
        # deploy-preview.sh appends these entries; a renamed key would otherwise be a silent no-op.
        for filename, release, appended in [
            ("values-solana-programs-e2e.yaml", "solana-host",
             {"KMS_THRESHOLD": "1", "COPROCESSOR_THRESHOLD": "1"}),
            ("values-solana-register-coprocessor-e2e.yaml", "solana-register-coprocessor-1",
             {"DATABASE_URL": "postgresql://zama:zama@postgres-coprocessor-1:5432/fhevm_e2e"}),
        ]:
            values = yaml.safe_load((VALUES / filename).read_text())
            names = [e["name"] for e in values["scDeploy"]["env"]]
            self.assertFalse(set(appended) & set(names), "overlay must not predefine appended env")
            values["scDeploy"]["env"] += [{"name": k, "value": v} for k, v in appended.items()]
            with tempfile.NamedTemporaryFile("w", suffix=".yaml") as merged:
                yaml.safe_dump(values, merged)
                merged.flush()
                documents = render(release, "contracts", [pathlib.Path(merged.name)])
            job = next(d for d in documents if d and d["kind"] == "Job")
            env = {e["name"]: e.get("value") for e in job["spec"]["template"]["spec"]["containers"][0]["env"]}
            for name, value in appended.items():
                self.assertEqual(env[name], value)
            if "register" in filename:
                self.assertEqual(env["SOLANA_KEY_SOURCE_CHAIN_ID"], "12345")

    def test_gateway_registration_overlay_renders(self):
        documents = render("gateway-add-host-chains-solana", "contracts",
                           [VALUES / "values-gateway-add-host-chains-solana-e2e.yaml"])
        job = next(d for d in documents if d and d["kind"] == "Job")
        self.assertTrue(job["spec"]["template"]["spec"]["containers"])

    def test_synced_secret_keys_match_what_the_overlays_read(self):
        # sync-secrets' template names the Secret keys. Every required secretKeyRef in the
        # overlays and in deploy-preview.sh must be one of them, or a pod starts without its value.
        synced = {}
        for filename in ["values-solana-rpc.yaml", "values-solana-deployer.yaml"]:
            spec = yaml.safe_load((VALUES / filename).read_text())["externalSecret"]
            rendered = "".join(spec["template"]["data"].values())
            for entry in spec["data"]:
                self.assertIn("{{ ." + entry["secretKeyName"] + " }}", rendered, filename)
            synced[spec["targetSecretName"]] = set(spec["template"]["data"])

        def refs(node):
            if isinstance(node, dict):
                ref = node.get("secretKeyRef")
                if isinstance(ref, dict) and ref.get("name") in synced and not ref.get("optional"):
                    yield ref["name"], ref["key"]
                for value in node.values():
                    yield from refs(value)
            elif isinstance(node, list):
                for value in node:
                    yield from refs(value)

        consumed = {name: set() for name in synced}
        for path in VALUES.glob("values-solana-*-e2e.yaml"):
            for name, key in refs(yaml.safe_load(path.read_text())):
                consumed[name].add(key)
        script = (VALUES / "deploy-preview.sh").read_text()
        for name, key in re.findall(r'"secretKeyRef":\{"name":"(solana-[a-z]+)","key":"([\w.-]+)"\}', script):
            consumed[name].add(key)
        for name, keys in consumed.items():
            self.assertTrue(keys, name)
            self.assertLessEqual(keys, synced[name], name)

    def test_dispatch_overrides_reject_unknown_keys_and_multiline_values(self):
        script = ROOT / "ci/preview-env/scripts/resolve/parse-overrides.cjs"
        for value, valid in [({}, True), ({"solana_programs_version": "abcdef0"}, True),
                             ({"unknown": "x"}, False), ({"kms_repo_ref": "a\nb"}, False),
                             ({"coprocessor_version": 1}, False), ([], False)]:
            with tempfile.NamedTemporaryFile() as output:
                result = subprocess.run(["node", str(script)], env={**os.environ,
                    "OVERRIDES_RAW": json.dumps(value), "EVENT_NAME": "workflow_dispatch",
                    "GITHUB_OUTPUT": output.name}, capture_output=True, text=True)
                self.assertEqual(result.returncode == 0, valid, result.stderr)
                if value == {}:
                    self.assertIn("relayer_sdk_version=0.4.4", pathlib.Path(output.name).read_text())



if __name__ == "__main__":
    unittest.main()
