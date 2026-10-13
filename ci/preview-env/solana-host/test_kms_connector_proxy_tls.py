"""Check the proxy TLS material deploy-kms-connector.sh makes, and its refusal of a Secret that predates the CA."""
import base64
import json
import os
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[3]
SCRIPT = ROOT / "ci/preview-env/scripts/deploy/deploy-kms-connector.sh"
MOCK = '''#!/usr/bin/env python3
import base64,fcntl,json,os,pathlib,shutil,sys
a=sys.argv[1:]
# `kubectl create ... | kubectl apply -f -` runs two mocks at once: the lock, held until exit, makes
# their read-modify-writes take turns. apply drains stdin first, as create needs the lock to write it.
# Writes stay in place; a rename would leave a waiting mock locked on the old file.
if a[:1]==['apply']: sys.stdin.read()
p=pathlib.Path(os.environ['TLS_TEST_STATE']); f=p.open('r+'); fcntl.flock(f,fcntl.LOCK_EX); s=json.load(f)
tool=pathlib.Path(sys.argv[0]).name
s['calls'].append([tool]+a)
if tool=='kubectl':
 if a[:2]==['get','configmap']: print('0x'+'11'*20)
 elif a[:3]==['get','secret','kms-connector-proxy-tls'] and '-o' not in a:
  if not s['secret']: p.write_text(json.dumps(s)); sys.exit(1)
 elif a[:3]==['get','secret','kms-connector-proxy-tls']:
  print(s['secret'].get('ca.crt',''))
 elif a[:3]==['create','secret','generic'] and a[3]=='kms-connector-proxy-tls':
  for arg in a:
   if arg.startswith('--from-file='):
    key,src=arg[len('--from-file='):].split('=',1)
    shutil.copy(src,p.parent/key)
    s['secret'][key]=base64.b64encode(pathlib.Path(src).read_bytes()).decode()
 elif a[0]=='create': print('{}')
 elif a[0]=='apply': pass
 else: raise Exception(a)
p.write_text(json.dumps(s))
'''


class ProxyTls(unittest.TestCase):
    def deploy(self, secret):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        work = pathlib.Path(directory.name)
        for name in ("kubectl", "helm", "sleep"):
            command = work / name
            command.write_text(MOCK)
            command.chmod(0o700)
        state = work / "state.json"
        state.write_text(json.dumps({"calls": [], "secret": secret}))
        result = subprocess.run(["/bin/bash", str(SCRIPT)], cwd=ROOT, capture_output=True, text=True, env={
            **os.environ, "PATH": f"{work}:{os.environ['PATH']}", "TLS_TEST_STATE": str(state), "NAMESPACE": "ns",
            "NB_KMS_CORE": "4", "DEPLOY_POLYGON": "false", "OBSERVABILITY": "false", "TAGS_JSON": "{}",
            "KMS_CONNECTOR_CHART": "chart"})
        return result, json.loads(state.read_text())["calls"], work

    def test_a_tls_secret_without_the_ca_is_refused_before_any_release(self):
        result, calls, _ = self.deploy({"tls.crt": "old"})
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("kms-connector-proxy-tls has no ca.crt", result.stderr)
        self.assertFalse([c for c in calls if c[0] == "helm"])

    def test_a_new_namespace_gets_a_ca_and_a_server_certificate_it_issued(self):
        result, calls, work = self.deploy({})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len([c for c in calls if c[:2] == ["helm", "upgrade"]]), 4)
        openssl = lambda *args: subprocess.check_output(["openssl", *args], text=True, stderr=subprocess.STDOUT)
        self.assertIn("CA:TRUE", openssl("x509", "-in", str(work / "ca.crt"), "-noout", "-text"))
        server = openssl("x509", "-in", str(work / "tls.crt"), "-noout", "-text")
        # rustls refuses a CA certificate served as a server certificate (CaUsedAsEndEntity).
        self.assertIn("CA:FALSE", server)
        self.assertIn("TLS Web Server Authentication", server)
        for party in range(1, 5):
            self.assertIn(f"DNS:kms-connector-{party}-kms-connector-proxy", server)
        self.assertIn("OK", openssl("verify", "-CAfile", str(work / "ca.crt"), str(work / "tls.crt")))


if __name__ == "__main__":
    unittest.main()
