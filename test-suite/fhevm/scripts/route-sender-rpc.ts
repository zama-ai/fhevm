#!/usr/bin/env bun
import { senderRoute } from '../src/consensus/sender-rpc-route';

const [snapshot, url] = process.argv.slice(2);
const row = (await Bun.file(snapshot!).json())[0];
const route = senderRoute(row.Config.Cmd, url);
if (!url) console.log(route.original);
else {
  const service = row.Config.Labels['com.docker.compose.service'];
  if (service !== 'coprocessor1-transaction-sender' || !/^sha256:[a-f0-9]{64}$/.test(row.Image))
    throw new Error('unowned sender route');
  console.log(JSON.stringify({ services: { [service]: { command: route.command, image: row.Image } } }));
}
