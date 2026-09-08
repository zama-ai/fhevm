"""Render the production charts and verify the Solana deployment boundaries."""
import pathlib
import json
import os
import tempfile
import subprocess
import textwrap
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

    def test_persistent_anvil_and_connector_migrations_are_opt_in(self):
        for enabled in [False, True]:
            docs = render("anvil", "anvil-node", [], "--set", f"persistState={str(enabled).lower()}")
            node = next(d for d in docs if d and d["kind"] == "StatefulSet")
            args = node["spec"]["template"]["spec"]["containers"][0]["args"]
            self.assertEqual("--state" in args, enabled)
            if enabled:
                self.assertIn("--state-interval", args)
                self.assertIn("--preserve-historical-states", args)
            docs = render("connector", "kms-connector", [], "--set", f"kmsConnectorDbMigration.runOnUpgrade={str(enabled).lower()}")
            job = next(d for d in docs if d and d["kind"] == "Job")
            hooks = job["metadata"].get("annotations", {}).get("helm.sh/hook")
            self.assertEqual(hooks, "pre-install,pre-upgrade" if enabled else None)

    def test_program_keys_are_optional_but_deployer_and_lock_are_required(self):
        for filename in ["values-solana-programs-e2e.yaml", "values-solana-demos-e2e.yaml"]:
            docs = render("solana", "contracts", [VALUES / filename])
            job = next(d for d in docs if d and d["kind"] == "Job")
            env = job["spec"]["template"]["spec"]["containers"][0]["env"]
            for entry in env:
                ref = entry.get("valueFrom", {}).get("secretKeyRef", {})
                if ref.get("name") == "solana-deployer":
                    self.assertEqual(ref.get("optional", False), ref["key"] != "deployer.json")
                if entry["name"] == "SOLANA_DEPLOY_DATABASE_URL":
                    self.assertEqual(ref["name"], "solana-deployment-lock")

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

    def test_persistent_preview_preserves_namespace_and_rejects_chain_changes(self):
        with tempfile.TemporaryDirectory() as directory:
            fixture = pathlib.Path(directory)
            kubectl = fixture / "kubectl"
            kubectl.write_text(textwrap.dedent('''\
                #!/usr/bin/env python3
                import base64, json, os, pathlib, sys
                root = pathlib.Path(os.environ["PREVIEW_TEST_DIR"])
                args = sys.argv[1:]
                with (root / "calls").open("a") as log:
                    log.write(" ".join(args) + "\\n")
                if args[:2] == ["get", "namespace"]:
                    sys.exit(0 if (root / "namespace").exists() else 1)
                if args[:2] == ["create", "namespace"]:
                    (root / "namespace").touch()
                elif args[:2] == ["get", "configmap"]:
                    marker = root / "fingerprint"
                    if marker.exists(): print(marker.read_text(), end="")
                    else: sys.exit(0 if "--ignore-not-found" in args else 1)
                elif args[:2] == ["get", "secret"] and "json" in args:
                    names = args[2:args.index("-n")]
                    records = [{"apiVersion":"v1", "kind":"Secret", "type":"Opaque",
                        "metadata":{"name":name}, "data":{
                            "rpc-url":base64.b64encode(b"https://secret.invalid/rpc").decode(),
                            "deployer.json":"WzFd"}} for name in names]
                    print(json.dumps(records[0] if len(records) == 1 else {"items":records}))
                elif args[:1] == ["apply"]:
                    sys.stdin.read()
                elif args[:1] == ["delete"]:
                    sys.exit("unexpected deletion")
            '''))
            kubectl.chmod(0o755)
            rpc = fixture / "rpc.cjs"
            rpc.write_text('global.fetch = async () => ({ok: true, json: async () => ({result: process.env.TEST_GENESIS})});')
            env = {**os.environ, "PATH":f"{fixture}:{os.environ['PATH']}",
                   "NODE_OPTIONS":f"--require={rpc}", "PREVIEW_TEST_DIR":directory,
                   "GITHUB_ENV":str(fixture / "env"), "SOLANA_ACTION":"deploy",
                   "NAMESPACE":"fhevm-ci-test", "SOLANA_SECRETS_NAMESPACE":"vault",
                   "NB_COPROCESSOR":"1", "NB_KMS_CORE":"1", "KMS_REPO_REF":"main",
                   "KMS_CORE_TAG":"pinned", "TEST_GENESIS":"1" * 32}

            def prepare(**changes):
                (fixture / "env").write_text("")
                return subprocess.run(["bash", str(VALUES / "prepare-preview.sh")],
                                      env={**env, **changes}, capture_output=True, text=True)

            initial = prepare()
            self.assertEqual(initial.returncode, 0, initial.stderr)
            output = (fixture / "env").read_text()
            self.assertIn("PREVIEW_BOOTSTRAP=true", output)
            fingerprint = next(line.split("=", 1)[1] for line in output.splitlines()
                               if line.startswith("SOLANA_PREVIEW_FINGERPRINT="))
            (fixture / "fingerprint").write_text(fingerprint)
            resumed = prepare(SOLANA_ACTION="upgrade")
            self.assertEqual(resumed.returncode, 0, resumed.stderr)
            self.assertIn("PREVIEW_BOOTSTRAP=false", (fixture / "env").read_text())
            self.assertNotEqual(prepare(TEST_GENESIS="2" * 32).returncode, 0)
            self.assertNotEqual(prepare(SOLANA_ACTION="off").returncode, 0)
            self.assertNotIn("delete", (fixture / "calls").read_text())


if __name__ == "__main__":
    unittest.main()
