"""Render the production charts and verify the Solana deployment boundaries."""
import pathlib
import json
import os
import tempfile
import subprocess
import unittest

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[3]
VALUES = ROOT / "ci/preview-env/solana-host"


def render(release, chart, values, *options):
    command = ["helm", "template", release, str(ROOT / "charts" / chart)]
    for value in values:
        command += ["-f", str(value)]
    return list(yaml.safe_load_all(subprocess.check_output(command + list(options), text=True)))


class SolanaCharts(unittest.TestCase):
    def test_host_and_demos_are_separate_jobs(self):
        for release, filename, operation in [
            ("solana-host", "values-solana-programs-e2e.yaml", "host deploy"),
            ("solana-demos", "values-solana-demos-e2e.yaml", "demos deploy"),
        ]:
            documents = render(release, "contracts", [VALUES / filename])
            job = next(d for d in documents if d and d["kind"] == "Job")
            self.assertEqual(job["metadata"]["name"], release + "-deploy")
            container = job["spec"]["template"]["spec"]["containers"][0]
            self.assertEqual(container["command"], ["/app/deploy-contracts.sh"])
            config = next(d for d in documents if d and d["kind"] == "ConfigMap")
            self.assertIn("node /app/cli.mjs " + operation, config["data"]["deploy-contracts.sh"])
            if release == "solana-host":
                self.assertFalse(any("TOKEN_KEYPAIR" in e["name"] for e in container["env"]))

    def test_listener_shares_database_and_keeps_proofs_private(self):
        documents = render("coprocessor-1", "coprocessor", [VALUES / "values-solana-coprocessor-e2e.yaml"],
                           "--set-string", "commonConfig.databaseUrl=postgresql://db/coprocessor")
        name = "coprocessor-1-solana-host-listener"
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and d["metadata"]["name"] == name)
        self.assertEqual(deployment["spec"]["replicas"], 1)
        self.assertEqual(deployment["spec"]["strategy"]["type"], "Recreate")
        container = deployment["spec"]["template"]["spec"]["containers"][0]
        env = {e["name"]: e for e in container["env"]}
        self.assertEqual(env["DATABASE_URL"]["value"], "postgresql://db/coprocessor")
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
        # Same composition as deploy-preview.sh: the party's values plus the appended Solana entry.
        base = yaml.safe_load((ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml").read_text())
        chain = yaml.safe_load((VALUES / "connector-host-chain.yaml").read_text())
        chain[0]["solanaProofEndpoints"] = ["http://coprocessor-1-solana-host-listener:8080"]
        base["kmsConnectorKmsWorker"]["config"]["hostChains"] += chain
        with tempfile.NamedTemporaryFile("w", suffix=".yaml") as merged:
            yaml.safe_dump(base, merged)
            merged.flush()
            documents = render("kms-connector-1", "kms-connector",
                               [pathlib.Path(merged.name), VALUES / "values-solana-connector-e2e.yaml"])
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and "kms-worker" in d["metadata"]["name"])
        env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
        names = [e["name"] for e in env]
        self.assertLess(names.index("SOLANA_PROOF_API_KEY"), names.index("KMS_CONNECTOR_HOST_CHAINS"))
        value = next(e["value"] for e in env if e["name"] == "KMS_CONNECTOR_HOST_CHAINS")
        self.assertIn('"chainId":9223372036854788153', value)
        self.assertIn('"aclAddress"', value)
        self.assertIn('http://coprocessor-1-solana-host-listener:8080', value)
        self.assertIn('"solanaProofApiKey":"$(SOLANA_PROOF_API_KEY)"', value)

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

    def test_dispatch_overrides_reject_unknown_keys_and_multiline_values(self):
        script = ROOT / "ci/preview-env/scripts/parse-overrides.cjs"
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
