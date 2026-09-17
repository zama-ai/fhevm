import { expect, test } from 'bun:test';
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, existsSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
const scripts=path.resolve(import.meta.dir,'../../scripts');
for(const variant of ['complete','missing','wrong-run','wrong-case','failed','malformed','bare']) {
  test(`executed assertion receipt is required by the actual record CLI: ${variant}`,()=>{
    const dir=mkdtempSync(path.join(tmpdir(),'assertion-recording-'));
    try {
      mkdirSync(path.join(dir,'bin'));
      writeFileSync(path.join(dir,'bin/docker'),`#!/bin/bash
[[ "$1" == exec ]] || exit 90
shift
while [[ "$1" == -e ]]; do export "$2"; shift 2; done
shift
exec "$@"
`,{mode:0o755});
      const receipts=['bytes','digest','provenance','liveness','quorum'].filter(name=>variant!=='missing'||name!=='provenance').map(name=>({runId:variant==='wrong-run'?'stale':'receipt-test',caseId:variant==='wrong-case'?'MAT-02-ALIAS-SOURCING':'MAT-01-BOUNDARY-FANOUT',name,outcome:variant==='failed'?'fail':'pass',detail:'executed fixture check'}));
      writeFileSync(path.join(dir,'emit.cjs'),variant==='bare'?'':`for(const receipt of ${JSON.stringify(receipts)}) console.log('[consensus-assertion] '+${variant==='malformed'?"'{not-json}'":'JSON.stringify(receipt)'});`);
      const result=Bun.spawnSync(['bash','-c',`set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${dir}'
source "$SCRIPT_DIR/lib/suite-process.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/runner-assertions.sh"
sp_init
CR_RUN_ID=receipt-test; CR_REVISION=fixture; CR_SCENARIO=three-of-three
CR_OPERATORS=3; CR_THRESHOLD=3; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=fixture
sp_exec target node '${dir}/emit.cjs' || exit 91
cr_record_checked_pass MAT-01-BOUNDARY-FANOUT cleanup=ok; status=$?
echo record_status=$status counter=\${CR_RECORD_FAILURES:-0}
sp_cancel_all; sp_dispose
exit "$status"
`],{env:{...process.env,PATH:`${dir}/bin:${process.env.PATH}`,FHEVM_STATE_DIR:dir,CONSENSUS_RESULTS_DIR:path.join(dir,'results')},timeout:10000});
      expect(result.exitCode===0,result.stderr.toString()).toBe(variant==='complete');
      expect(result.stdout.toString()).toContain(`counter=${variant==='complete'?0:1}`);
      const resultFile=path.join(dir,'results/receipt-test.jsonl');
      expect(existsSync(resultFile)).toBe(variant==='complete');
      if(variant==='complete') expect(JSON.parse(readFileSync(resultFile,'utf8')).assertions).toHaveLength(5);
    } finally {rmSync(dir,{recursive:true,force:true});}
  },15000);
}
