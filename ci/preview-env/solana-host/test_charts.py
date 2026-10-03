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
LISTENER = "coprocessor-1-solana-host-listener"
INDEXER = "coprocessor-1-solana-merkle-indexer"
PROOF_SERVER = "coprocessor-1-solana-merkle-proof-server"
# deploy-preview.sh sets it on every rollout.
START_SLOT = ["--set-string", "solanaHostListener.merkleIndexer.startSlot=4242"]
# deploy-preview.sh merges the Solana values into the party's coprocessor release.
COPROCESSOR = [ROOT / "ci/preview-env/coprocessor/values-coprocessor-e2e.yaml",
               VALUES / "values-solana-coprocessor-e2e.yaml"]
PROOF_URLS = [f"http://{PROOF_SERVER}:8080"]


def render(release, chart, values, *options):
    command = ["helm", "template", release, str(ROOT / "charts" / chart)]
    for value in values:
        command += ["-f", str(value)]
    return list(yaml.safe_load_all(subprocess.check_output(command + list(options), text=True)))


def render_error(*options):
    """Renders the Solana coprocessor values with `options` and returns why Helm refused."""
    result = subprocess.run(["helm", "template", "coprocessor-1", str(ROOT / "charts/coprocessor"),
                             *[option for values in COPROCESSOR for option in ["-f", str(values)]], *options],
                            capture_output=True, text=True)
    assert result.returncode != 0, "the chart rendered"
    return result.stderr


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

    def test_the_merkle_service_has_its_own_database_and_keeps_proofs_private(self):
        documents = render("coprocessor-1", "coprocessor", COPROCESSOR, *START_SLOT)
        deployments = {}
        for name, database in [(LISTENER, "fhevm_e2e"), (INDEXER, "solana_merkle"), (PROOF_SERVER, "solana_merkle")]:
            deployment = next(d for d in documents if d and d["kind"] == "Deployment" and d["metadata"]["name"] == name)
            container = deployment["spec"]["template"]["spec"]["containers"][0]
            env = {e["name"]: e for e in container["env"]}
            self.assertEqual(env["DATABASE_URL"]["value"],
                             f"postgresql://$(DATABASE_USER):$(DATABASE_PASSWORD)@$(DATABASE_ENDPOINT)/{database}")
            names = [e["name"] for e in container["env"]]
            for variable in ["DATABASE_USER", "DATABASE_PASSWORD", "DATABASE_ENDPOINT"]:
                self.assertLess(names.index(variable), names.index("DATABASE_URL"))
            deployments[name] = (deployment, env)
        for name in [LISTENER, INDEXER]:
            writer, writer_env = deployments[name]
            self.assertEqual(writer["spec"]["replicas"], 1)
            self.assertEqual(writer["spec"]["strategy"]["type"], "Recreate")
        indexer, indexer_env = deployments[INDEXER]
        indexer_container = indexer["spec"]["template"]["spec"]["containers"][0]
        self.assertEqual(indexer_container["command"], ["solana_merkle_indexer"])
        self.assertEqual(indexer_env["SOLANA_MERKLE_START_SLOT"]["value"], "4242")
        self.assertEqual(indexer_env["SOLANA_GRPC_URL"]["valueFrom"]["secretKeyRef"]["name"], "solana-rpc")
        server, server_env = deployments[PROOF_SERVER]
        self.assertEqual(server["spec"]["template"]["spec"]["containers"][0]["command"], ["solana_merkle_proof_server"])
        self.assertEqual(server_env["ETHEREUM_RPC_URL"]["value"], "http://anvil-host-anvil-node:8545")
        self.assertEqual(server_env["PROTOCOL_CONFIG_ADDRESS"]["valueFrom"]["configMapKeyRef"],
                         {"name": "host-sc-addresses", "key": "protocol_config.address"})
        service = next(d for d in documents if d and d["kind"] == "Service" and d["metadata"]["name"] == PROOF_SERVER)
        self.assertEqual(service["spec"]["type"], "ClusterIP")
        self.assertEqual(service["spec"]["selector"], server["spec"]["selector"]["matchLabels"])
        for name in [LISTENER, INDEXER]:
            writer_service = next(d for d in documents if d and d["kind"] == "Service" and d["metadata"]["name"] == name)
            self.assertEqual([p["name"] for p in writer_service["spec"]["ports"]], ["metrics"])

    def test_the_indexer_requires_a_start_slot(self):
        self.assertIn("solanaHostListener.merkleIndexer.startSlot is required", render_error())

    def test_a_numeric_start_slot_renders_as_an_integer(self):
        documents = render("coprocessor-1", "coprocessor", COPROCESSOR,
                           "--set-json", "solanaHostListener.merkleIndexer.startSlot=312000000")
        indexer = next(d for d in documents if d and d["kind"] == "Deployment" and d["metadata"]["name"] == INDEXER)
        env = {e["name"]: e for e in indexer["spec"]["template"]["spec"]["containers"][0]["env"]}
        self.assertEqual(env["SOLANA_MERKLE_START_SLOT"]["value"], "312000000")

    def test_a_start_slot_that_is_not_a_number_is_refused(self):
        self.assertIn("startSlot must be a slot number",
                      render_error("--set-string", "solanaHostListener.merkleIndexer.startSlot=$(START_SLOT)"))

    def test_the_merkle_service_requires_its_database(self):
        self.assertIn("solanaHostListener.merkleDatabaseUrl is required",
                      render_error(*START_SLOT, "--set", "solanaHostListener.merkleDatabaseUrl="))

    def test_the_proof_server_requires_the_canonical_chain(self):
        self.assertIn("commonConfig.canonicalProtocolConfigChainId is required",
                      render_error(*START_SLOT, "--set", "commonConfig.canonicalProtocolConfigChainId="))
        self.assertIn("needs a chains entry with chainId 1, the canonicalProtocolConfigChainId",
                      render_error(*START_SLOT, "--set-string", "commonConfig.canonicalProtocolConfigChainId=1"))
        self.assertIn("chains entry host needs httpUrl or httpUrlValueFrom",
                      render_error(*START_SLOT, "--set", "chains[0].httpUrl="))
        self.assertIn("chains entry host needs protocolConfigContractAddress",
                      render_error(*START_SLOT, "--set", "chains[0].protocolConfigContractAddressValueFrom=null"))

    def test_iam_certificate_is_a_volume_list(self):
        documents = render("coprocessor-1", "coprocessor", COPROCESSOR,
                           "--set", "commonConfig.databaseAuthMode=iam",
                           "--set", "commonConfig.databaseSslRootCert.enabled=true", *START_SLOT)
        for name in [LISTENER, INDEXER, PROOF_SERVER]:
            deployment = next(d for d in documents if d and d["kind"] == "Deployment" and d["metadata"]["name"] == name)
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
        # proof URLs the script sets. aclAddress is filled per party by deploy-kms-connector.sh.
        documents = render("kms-connector-1", "kms-connector",
                           [ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml",
                            VALUES / "values-solana-connector-e2e.yaml"],
                           "--set-string", "commonConfig.hostChains.ethereum.aclAddress=0x" + "11" * 20,
                           "--set-json",
                           'commonConfig.hostChains.solana.solanaProofUrls=' + json.dumps(PROOF_URLS))
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and "kms-worker" in d["metadata"]["name"])
        env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
        tx_sender = next(d for d in documents if d and d["kind"] == "Deployment" and "tx-sender" in d["metadata"]["name"])
        tx_sender_env = {e["name"]: e for e in tx_sender["spec"]["template"]["spec"]["containers"][0]["env"]}
        # kms-worker signs its proof requests with the tx-sender's key.
        worker_env = {e["name"]: e for e in env}
        self.assertEqual(worker_env["KMS_CONNECTOR_PRIVATE_KEY"], tx_sender_env["KMS_CONNECTOR_PRIVATE_KEY"])
        chains = {c["chainId"]: c for c in json.loads(next(e["value"] for e in env if e["name"] == "KMS_CONNECTOR_HOST_CHAINS"))}
        self.assertEqual(chains[12345]["aclAddress"], "0x" + "11" * 20)
        solana = chains[130140237723663404]
        self.assertNotIn("aclAddress", solana)
        self.assertEqual(solana["solanaProofUrls"], PROOF_URLS)
        endpoint = next(d for d in documents if d and d["kind"] == "Deployment" and "endpoint" in d["metadata"]["name"])
        ids = next(e["value"] for e in endpoint["spec"]["template"]["spec"]["containers"][0]["env"]
                   if e["name"] == "KMS_CONNECTOR_SUPPORTED_CHAIN_IDS")
        self.assertIn("130140237723663404", ids.split(","))

    def test_kms_worker_gets_the_tx_sender_wallet_only_with_a_solana_chain(self):
        def wallet_env(values, *options):
            documents = render("kms-connector-1", "kms-connector", values,
                               "--set-string", "commonConfig.hostChains.ethereum.aclAddress=0x" + "11" * 20,
                               *options)
            envs = {}
            for component in ["kms-worker", "tx-sender"]:
                deployment = next(d for d in documents if d and d["kind"] == "Deployment"
                                  and component in d["metadata"]["name"])
                env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
                envs[component] = {e["name"]: e for e in env if e["name"] in
                                   ["KMS_CONNECTOR_PRIVATE_KEY", "KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID"]}
            return envs
        evm = [ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml"]
        self.assertEqual(wallet_env(evm)["kms-worker"], {})
        envs = wallet_env(evm + [VALUES / "values-solana-connector-e2e.yaml"],
                          "--set-json", "commonConfig.hostChains.solana.solanaProofUrls=" + json.dumps(PROOF_URLS),
                          "--set", "kmsConnectorTxSender.wallet.awsKms.enabled=true")
        self.assertEqual(list(envs["kms-worker"]), ["KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID"])
        self.assertEqual(envs["kms-worker"], envs["tx-sender"])

    def test_connector_reads_the_kind_from_the_type_byte_at_any_cluster_tag(self):
        # The PoC chain id has cluster tag 12345: a wrong shift would read it as EVM.
        documents = render("kms-connector-1", "kms-connector",
                           [ROOT / "ci/preview-env/kms-connector/values-kms-connector-e2e.yaml",
                            VALUES / "values-solana-connector-e2e.yaml"],
                           "--set-string", "commonConfig.hostChains.ethereum.aclAddress=0x" + "11" * 20,
                           "--set-string", "commonConfig.hostChains.solana.chainId=72057594037940281",
                           "--set-json",
                           'commonConfig.hostChains.solana.solanaProofUrls=' + json.dumps(PROOF_URLS))
        deployment = next(d for d in documents if d and d["kind"] == "Deployment" and "kms-worker" in d["metadata"]["name"])
        env = deployment["spec"]["template"]["spec"]["containers"][0]["env"]
        chains = {c["chainId"]: c for c in json.loads(next(e["value"] for e in env if e["name"] == "KMS_CONNECTOR_HOST_CHAINS"))}
        self.assertEqual(chains[72057594037940281]["solanaProofUrls"], PROOF_URLS)

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
                           "--set-json", "commonConfig.hostChains.solana.solanaProofUrls=" + json.dumps(PROOF_URLS),
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

    def test_preview_proof_urls_name_the_merkle_proof_servers(self):
        # The connector tests pass PROOF_URLS in themselves: this pins it to what deploy-preview.sh sets.
        script = (VALUES / "deploy-preview.sh").read_text()
        program = re.search(r"^proof_urls=\$\(seq 1 \"\$NB_COPROCESSOR\" \| jq -Rsc '(.*)'\)$", script, re.M).group(1)
        proof_urls = subprocess.check_output(["jq", "-Rsc", program], input="1\n", text=True)
        self.assertEqual(json.loads(proof_urls), PROOF_URLS)

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
