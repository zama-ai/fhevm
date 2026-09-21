"""Recovery follows the latest deployed tooling, unless explicitly overridden."""
import json
import os
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parent
MOCK = '''#!/usr/bin/env python3
import json,os,pathlib,sys
a=sys.argv[1:]
if a[:2]==['get','jobs']: print(os.environ['IMAGE_TEST_JOBS'])
elif a[0]=='get': print('exists')
elif a[0]=='apply':
 data=sys.stdin.read()
 if data.startswith('{'):
  pathlib.Path(os.environ['IMAGE_TEST_OUTPUT']).write_text(data)
  sys.exit(1) # Stop after inspecting the proposed Job; no live cluster operations.
else: raise Exception(a)
'''

class RecoveryImage(unittest.TestCase):
    def check_image(self, jobs, expected, override=''):
        with tempfile.TemporaryDirectory() as directory:
            work = pathlib.Path(directory)
            command = work / 'kubectl'
            command.write_text(MOCK)
            command.chmod(0o700)
            output = work / 'job.json'
            items = [{'metadata': {'name': name, 'creationTimestamp': stamp},
                      'spec': {'template': {'spec': {'containers': [{'image': image}]}}}}
                     for name, stamp, image in jobs]
            result = subprocess.run(['/bin/bash', str(ROOT / 'recover.sh')],
                env={**os.environ, 'PATH': f'{work}:{os.environ["PATH"]}',
                     'NAMESPACE': 'fhevm-ci-image-test', 'SOLANA_OPERATION_HELD': '1',
                     'SOLANA_RECOVERY_IMAGE': override, 'IMAGE_TEST_JOBS': json.dumps({'items': items}),
                     'IMAGE_TEST_OUTPUT': str(output)}, capture_output=True, text=True)
            self.assertEqual(result.returncode, 1, result.stderr)
            job = json.loads(output.read_text())
            self.assertEqual(job['spec']['template']['spec']['containers'][0]['image'], expected)

    def test_new_deployment_supersedes_old_recovery(self):
        self.check_image([('solana-recovery-old', '2026-09-20', 'repo/solana-programs:old'),
                          ('solana-host-new', '2026-09-21', 'repo/solana-programs:new')], 'repo/solana-programs:new')

    def test_new_recovery_fix_supersedes_deployment(self):
        self.check_image([('solana-host-old', '2026-09-20', 'repo/solana-programs:old'),
                          ('solana-recovery-new', '2026-09-21', 'repo/solana-programs:new')], 'repo/solana-programs:new')

    def test_explicit_override_wins(self):
        self.check_image([], 'repo/solana-programs:override', 'repo/solana-programs:override')
