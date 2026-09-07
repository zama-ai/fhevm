"""Render the production charts and verify the Solana deployment boundaries."""
import pathlib
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
            self.assertEqual(job["metadata"]["name"], release + "-deploy-1")
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
        documents = render("kms-connector-1", "kms-connector", [VALUES / "values-solana-connector-e2e.yaml"])
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and "kms-worker" in d["metadata"]["name"])
        env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
        names = [e["name"] for e in env]
        self.assertLess(names.index("SOLANA_PROOF_API_KEY"), names.index("KMS_CONNECTOR_HOST_CHAINS"))
        value = next(e["value"] for e in env if e["name"] == "KMS_CONNECTOR_HOST_CHAINS")
        self.assertIn('"chainId":9223372036854788153', value)
        self.assertIn('"aclAddress"', value)
        self.assertIn('http://coprocessor-1-solana-host-listener:8080', value)
        self.assertIn('"solanaProofApiKey":"$(SOLANA_PROOF_API_KEY)"', value)


if __name__ == "__main__":
    unittest.main()
