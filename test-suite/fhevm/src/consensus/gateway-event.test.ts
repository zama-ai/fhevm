import {expect,test} from 'bun:test';
import {gatewayEventWarnings} from './gateway-event';
const event={handle:`0x${'11'.repeat(32)}`,eventBlock:123,eventBlockHash:`0x${'22'.repeat(32)}`,eventTxHash:`0x${'33'.repeat(32)}`,eventLogIndex:0};
const warning=(override:Record<string,unknown>={})=>JSON.stringify({level:'WARN',fields:{message:'Drift detected: local digest does not match consensus',source:'consensus',handle:event.handle,block_number:event.eventBlock,block_hash:`Some(${event.eventBlockHash})`,tx_hash:`Some(${event.eventTxHash})`,log_index:'Some(0)',...override}});
test('gateway recovery requires the exact event receipt and log identity, not watermark movement or unrelated drift',()=>{
 expect(gatewayEventWarnings('watermark advanced to 124',event)).toHaveLength(0);
 expect(gatewayEventWarnings(warning(),event)).toHaveLength(1);
 for(const override of [{handle:`0x${'44'.repeat(32)}`},{block_number:124},{block_hash:'Some(0x00)'},{tx_hash:'Some(0x00)'},{log_index:'Some(1)'},{log_index:undefined},{source:'peer_submission'}]) {
  expect(gatewayEventWarnings(warning(override),event)).toHaveLength(0);
 }
});
test('repeated handling remains visible to the once-only warning assertion',()=>{
 expect(gatewayEventWarnings(`${warning()}\n${warning()}`,event)).toHaveLength(2);
});
