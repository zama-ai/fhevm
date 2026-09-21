"""Exercise exclusivity against a small Kubernetes API command double."""
import json
import os
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parent
MOCK = '''#!/usr/bin/env python3
import json,os,sys
p=os.environ['CLUSTER_STATE']; s=json.load(open(p)); a=sys.argv[1:]; changed=False
if a[:2]==['get','namespace']:
 n=a[2]
 if n=='fhevm-ci-solana-owner':
  if 'owner' not in s: sys.exit(1)
  print(s['owner'])
 else: print(s['uid'])
elif a[:2]==['get','secrets']: print(json.dumps({'items':[]}))
elif a[:2]==['get','jobs']:
 print(json.dumps({'items':[{'spec':{'template':{'spec':{'containers':[{'image':'repo/solana-programs:test'}]}}},'status':{}}] if s.get('running') else []}))
elif a[:2]==['create','namespace']: print(json.dumps({'metadata':{}}))
elif a[:2]==['create','-f']:
 if 'owner' in s: sys.exit(1)
 s['owner']=json.load(sys.stdin)['metadata']['annotations']['solana-preview-owner-uid']; changed=True
elif a[:2]==['create','configmap']:
 if s.get('locked'): sys.exit(1)
 s['locked']=True; changed=True
elif a[:2]==['delete','configmap']: s['locked']=False; changed=True
else: raise Exception(a)
if changed: json.dump(s,open(p,'w'))
'''


class Ownership(unittest.TestCase):
    def test_exclusive_owner_and_interrupted_job_lock(self):
        with tempfile.TemporaryDirectory() as work:
            work = pathlib.Path(work)
            mock = work / 'kubectl'
            mock.write_text(MOCK)
            mock.chmod(0o700)
            state = work / 'state.json'
            state.write_text(json.dumps({'uid': 'first'}))
            env = {**os.environ, 'PATH': f'{work}:{os.environ["PATH"]}',
                   'CLUSTER_STATE': str(state), 'NAMESPACE': 'fhevm-ci-eikix-test'}
            def run(command):
                return subprocess.run(['bash', '-euo', 'pipefail', '-c',
                    f'source "{ROOT}/ownership.sh"; {command}'], env=env,
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE).returncode
            self.assertEqual(run('solana_acquire'), 0)
            self.assertNotEqual(run('solana_acquire'), 0)
            cluster = json.loads(state.read_text())
            cluster['running'] = True
            state.write_text(json.dumps(cluster))
            self.assertEqual(run('solana_release_operation'), 0)
            self.assertTrue(json.loads(state.read_text())['locked'])
            cluster['running'] = False
            state.write_text(json.dumps(cluster))
            self.assertEqual(run('solana_release_operation'), 0)
            cluster = json.loads(state.read_text())
            self.assertFalse(cluster['locked'])
            cluster['uid'] = 'replacement-namespace'
            state.write_text(json.dumps(cluster))
            self.assertNotEqual(run('solana_acquire'), 0)


if __name__ == '__main__':
    unittest.main()
