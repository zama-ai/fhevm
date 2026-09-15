import {expect} from 'chai';
import {spawnSync} from 'node:child_process';
import {mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';

describe('gateway digest recovery record',()=>{
  for (const failOperator of [-1,1]) {
    it(failOperator < 0 ? 'records all originals before any write and restores every operator' : 'continues other restorations after a failure and retains the original recovery record',()=>{
      const dir=mkdtempSync(path.join(tmpdir(),'gateway-digest-recovery-'));
      try {
        const result=spawnSync(process.execPath,['-r',require.resolve('ts-node/register/transpile-only'),'-e',`
const assert=require('node:assert/strict');
const fs=require('node:fs');
const {Pool}=require('pg');
const writes=[];
Pool.prototype.query=async function(sql) {
  const index=this.options.connectionString.endsWith('coprocessor')?0:Number(this.options.connectionString.split('_').at(-1));
  if (sql.startsWith('SELECT')) return {rowCount:1,rows:[{digest:String(index+1).repeat(64),ciphertext:Buffer.from(String(index+1).repeat(64),'hex')}]};
  if (sql.startsWith('UPDATE')) {
    assert.equal(fs.existsSync(process.env.CONSENSUS_HANDSHAKE_DIR+'/degraded-gw-event.json'),true);
    writes.push(index);
    if(index===${failOperator}) throw new Error('injected restore failure');
    return {rowCount:1};
  }
  throw new Error('unexpected query');
};
Pool.prototype.connect=async function(){return {query:Pool.prototype.query.bind(this),release(){}};};
const {getCoprocessorDbUrls}=require('./test/consensus/helpers');
const {recordPendingGatewayEvent,restorePendingGatewayDigests}=require('./test/consensus/gatewayRecovery');
(async()=>{
 await recordPendingGatewayEvent(getCoprocessorDbUrls(3),{handle:'0x'+'aa'.repeat(32),eventBlock:1,eventBlockHash:'0x'+'bb'.repeat(32),eventTxHash:'0x'+'cc'.repeat(32),eventLogIndex:0});
 assert.deepEqual(writes,[]);
 let error;
 try {await restorePendingGatewayDigests();} catch(e) {error=e;}
 assert.deepEqual(writes.sort(),[0,1,2]);
 const record=JSON.parse(fs.readFileSync(process.env.CONSENSUS_HANDSHAKE_DIR+'/degraded-gw-event.json','utf8')).payload;
 assert.equal(record.originals.length,3);
 if(${failOperator}<0) {assert.equal(error,undefined);assert.equal(record.restored,true);}
 else {assert.match(error.message,/cleanup failed for operator\\(s\\) 1/);assert.notEqual(record.restored,true);}
})().catch(e=>{console.error(e);process.exitCode=1;});
`],{cwd:path.join(__dirname,'../..'),env:{...process.env,COPROCESSOR_COUNT:'3',CONSENSUS_HANDSHAKE_DIR:dir},encoding:'utf8'});
        expect(result.status,result.stderr).to.eq(0);
      } finally {rmSync(dir,{recursive:true,force:true});}
    });
  }
});
