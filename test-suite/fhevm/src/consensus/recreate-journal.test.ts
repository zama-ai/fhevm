import {expect,test} from 'bun:test';
import {mkdtempSync,rmSync} from 'node:fs';
import {randomUUID} from 'node:crypto';
import {tmpdir} from 'node:os';
import path from 'node:path';
const scripts=path.resolve(import.meta.dir,'../../scripts');
const enabled=process.env.RUN_DOCKER_RECOVERY_TESTS==='1';
test.skipIf(!enabled)('recreated non-root harness can read its original private journal and handshake ownership',()=>{
 const directory=mkdtempSync(path.join(tmpdir(),'recreate-nonroot-journal-'));
 const owner=randomUUID();const name=`review-r4-journal-${owner}`;
 try{
 const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${directory}'; FHEVM_STATE_DIR='${directory}'
source "$SCRIPT_DIR/lib/suite-process.sh"
sp_init
cleanup(){ local status=$?; trap - EXIT; if [[ "$(command docker inspect -f '{{index .Config.Labels "reviewround4"}}' '${name}' 2>/dev/null)" == '${owner}' ]]; then command docker rm -f '${name}' >/dev/null 2>&1; fi; exit "$status"; }
trap cleanup EXIT
command docker run -d --network none --user 10000:10000 --name '${name}' --label reviewround4='${owner}' debian:12-slim sleep 300 >/dev/null || exit 2
docker exec '${name}' sh -c 'umask 077; mkdir -p /tmp/handshake /tmp/fhevm-consensus-abort-recovery; echo handshake > /tmp/handshake/work.json; echo original-private-recovery > /tmp/fhevm-consensus-abort-recovery/recovery.json' || exit 3
sp_snapshot_recreate '${name}' /tmp/handshake || exit 4
[[ "$(stat -c %a "$SP_RUNTIME_DIR/recreate-${name}/journal.tar")" == 600 ]] || exit 5
docker cp '${name}:/tmp/fhevm-consensus-abort-recovery/.' '${directory}/plain-backup' || exit 10
docker rm -f '${name}' >/dev/null || exit 6
docker run -d --network none --user 10000:10000 --name '${name}' --label reviewround4='${owner}' debian:12-slim sleep 300 >/dev/null || exit 7
# Negative control: ordinary Docker copy turns the private file into root:root.
docker exec '${name}' mkdir -p /tmp/fhevm-consensus-abort-recovery || exit 10
docker cp '${directory}/plain-backup/.' '${name}:/tmp/fhevm-consensus-abort-recovery' || exit 11
if docker exec '${name}' cat /tmp/fhevm-consensus-abort-recovery/recovery.json >/dev/null 2>&1; then exit 12; fi
sp_restore_recreated '${name}' || exit 8
# These checks execute as Config.User=10000, not root through a helper override.
docker exec '${name}' sh -c '
 test "$(id -u)" = 10000 &&
 test "$(cat /tmp/fhevm-consensus-abort-recovery/recovery.json)" = original-private-recovery &&
 test "$(cat /tmp/handshake/work.json)" = handshake &&
 test "$(stat -c %u:%g:%a /tmp/fhevm-consensus-abort-recovery/recovery.json)" = 10000:10000:600 &&
 test "$(stat -c %u:%g:%a /tmp/fhevm-consensus-abort-recovery)" = 10000:10000:700
' || exit 9
`],{timeout:40000,env:{...process.env,DOCKER_CONTEXT:'',DOCKER_HOST:process.env.DOCKER_HOST||'unix:///var/run/docker.sock'}});
 expect(r.exitCode,r.stderr.toString()).toBe(0);
 }finally{
   // Only remove the uniquely labeled resource this test owns, even after a timeout.
   const inspected=Bun.spawnSync(['docker','inspect','-f','{{index .Config.Labels "reviewround4"}}',name],{timeout:5000});
   if(inspected.exitCode===0&&inspected.stdout.toString().trim()===owner) Bun.spawnSync(['docker','rm','-f',name],{timeout:10000});
   rmSync(directory,{recursive:true,force:true});
 }
},50000);
