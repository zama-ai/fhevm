#!/usr/bin/env bun
import {gatewayEventWarnings} from '../src/consensus/gateway-event';
const [recordFile, logsFile] = process.argv.slice(2);
try {
  const event = (await Bun.file(recordFile).json()).payload;
  if (!/^0x[a-f0-9]{64}$/i.test(event.handle) || !/^0x[a-f0-9]{64}$/i.test(event.eventTxHash) || !/^0x[a-f0-9]{64}$/i.test(event.eventBlockHash) ||
    !Number.isSafeInteger(event.eventBlock) || !Number.isSafeInteger(event.eventLogIndex)) throw new Error('invalid gateway event identity');
  const matches = gatewayEventWarnings(await Bun.file(logsFile).text(), event);
  if (matches.length > 1) throw new Error(`identified gateway event was applied ${matches.length} times`);
  if (!matches.length) process.exitCode = 2; // still pending; the caller has a deadline
  else console.log(matches[0]);
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
}
