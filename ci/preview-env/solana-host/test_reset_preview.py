"""Check reset orchestration preserves services and fails before recovery on active jobs."""
import json
import os
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parent
MOCK = '''#!/usr/bin/env python3
import json,os,pathlib,sys
p=pathlib.Path(os.environ['RESET_TEST_STATE']); s=json.loads(p.read_text()); a=sys.argv[1:]
tool=pathlib.Path(sys.argv[0]).name
s['calls'].append([tool]+a)
if tool=='helm':
 assert s.get('locked')
 if a[0]=='list': print(json.dumps([{'name':n} for n in s['releases']]))
 elif a[:2]==['get','values']: print('scDeploy: {image: {tag: pinned}}')
 elif a[0]=='upgrade':
  assert s.get('locked')
  assert 'pinned' in pathlib.Path(a[a.index('-f')+1]).read_text()
 else: raise Exception(a)
elif tool=='bash':
 assert a[0].endswith('/recover.sh') and a[1]=='reset' and s.get('locked')
 s['recovered']=True
elif tool=='kubectl':
 if a[:2]==['get','namespace']: print('preview-uid')
 elif a[:2]==['get','job']:
  if not s.get('missing'): print(json.dumps({'status':{'conditions':[] if s.get('active') else [{'type':'Complete','status':'True'}]}}))
 elif a[:2]==['get','jobs']:
  print(json.dumps({'items':[{'spec':{'template':{'spec':{'containers':[{'image':'repo/solana-programs:pinned'}]}}},'status':{}}] if s.get('active') else []}))
 elif a[:2]==['create','configmap']:
  assert not s.get('locked'); s['locked']=True
 elif a[:2]==['delete','configmap']: s['locked']=False
 elif a[:2]==['delete','job']: assert s.get('recovered') and s.get('locked')
 else: raise Exception(a)
p.write_text(json.dumps(s))
'''


class ResetPreview(unittest.TestCase):
    def run_reset(self, active=False, missing=False):
        with tempfile.TemporaryDirectory() as directory:
            work = pathlib.Path(directory)
            for name in ('helm', 'kubectl', 'bash'):
                command = work / name
                command.write_text(MOCK)
                command.chmod(0o700)
            state = work / 'state.json'
            state.write_text(json.dumps({'calls': [], 'active': active, 'missing': missing,
                'releases': ['solana-demos', 'relayer', 'solana-register-coprocessor-1', 'solana-host']}))
            result = subprocess.run(['/bin/bash', str(ROOT / 'reset-preview.sh')],
                env={**os.environ, 'PATH': f'{work}:{os.environ["PATH"]}',
                    'RESET_TEST_STATE': str(state), 'NAMESPACE': 'fhevm-ci-eikix-test'},
                capture_output=True, text=True)
            return result, json.loads(state.read_text())

    def test_reset_preserves_services_and_pins(self):
        result, state = self.run_reset()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(state['recovered'])
        self.assertFalse(state['locked'])
        upgrades = [c[2] for c in state['calls'] if c[:2] == ['helm', 'upgrade']]
        self.assertEqual(upgrades, ['solana-host', 'solana-register-coprocessor-1', 'solana-demos'])
        deletions = [c for c in state['calls'] if c[:2] == ['kubectl', 'delete']]
        self.assertTrue(all(c[2] in ('job', 'configmap') for c in deletions))

    def test_active_bootstrap_stops_before_recovery(self):
        result, state = self.run_reset(active=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(state.get('recovered'))
        self.assertTrue(state.get('locked'))

    def test_retry_accepts_missing_bootstrap_job(self):
        result, state = self.run_reset(missing=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(state['recovered'])


if __name__ == '__main__':
    unittest.main()
