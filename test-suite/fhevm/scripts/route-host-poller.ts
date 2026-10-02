#!/usr/bin/env bun
import { pollerRoute } from "../src/consensus/host-rpc-route";
const [snapshot, url] = process.argv.slice(2);
const row = (await Bun.file(snapshot!).json())[0];
const route = pollerRoute(row.Config.Cmd, url);
if (!url) console.log(route.original);
else {
  const service = row.Config.Labels["com.docker.compose.service"];
  if (!/^coprocessor1-host-listener-poller$/.test(service) || !/^sha256:[a-f0-9]{64}$/.test(row.Image)) throw new Error("unowned poller route");
  console.log(JSON.stringify({ services: { [service]: { command: route.command, image: row.Image } } }));
}
