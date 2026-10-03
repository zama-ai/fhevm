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
elif tool=='curl':
 print(json.dumps({'result':5000}))
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
 elif a[:2]==['get','deployment/coprocessor-1-solana-merkle-indexer']: print(a[1])
 elif a[:2]==['get','pods']: pass
 elif a[:2]==['get','secret']: print('aHR0cDovL3JwYw==')
 elif a[0]=='scale': s.setdefault('scaled',[]).append(a[-1])
 elif a[:2]==['set','env']:
  assert s.get('recovered'); s['start_slot']=a[-1]
 elif a[:2]==['rollout','status']: pass
 elif a[0]=='exec':
  sql=a[-1]
  if sql.startswith('DROP') and s.get('drop_fails'): sys.exit(1)
  if sql.startswith(('DROP','CREATE')):
   assert s.get('recovered') and s['scaled']==['--replicas=0']; s.setdefault('sql',[]).append(sql)
  elif 'FROM checkpoint' in sql: print(5001)
  else: raise Exception(a)
 else: raise Exception(a)
p.write_text(json.dumps(s))
'''


class ResetPreview(unittest.TestCase):
    def run_reset(self, active=False, missing=False, drop_fails=False):
        with tempfile.TemporaryDirectory() as directory:
            work = pathlib.Path(directory)
            for name in ('helm', 'kubectl', 'bash', 'curl'):
                command = work / name
                command.write_text(MOCK)
                command.chmod(0o700)
            state = work / 'state.json'
            state.write_text(json.dumps({'calls': [], 'active': active, 'missing': missing, 'drop_fails': drop_fails,
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
        self.assertEqual(state['sql'], ['DROP DATABASE IF EXISTS solana_merkle WITH (FORCE)',
                                        'CREATE DATABASE solana_merkle'])
        self.assertEqual(state['start_slot'], 'SOLANA_MERKLE_START_SLOT=5000')
        self.assertEqual(state['scaled'], ['--replicas=0', '--replicas=1'])

    def test_a_merkle_record_that_cannot_be_dropped_stops_the_reset(self):
        result, state = self.run_reset(drop_fails=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('start_slot', state)
        self.assertFalse(any(c[:2] == ['helm', 'upgrade'] for c in state['calls']))

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
