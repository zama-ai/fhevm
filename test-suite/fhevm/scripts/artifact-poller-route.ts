#!/usr/bin/env bun
import { artifactReplayRoute } from '../src/consensus/artifact-recovery';

const file = process.argv[2];
if (!file) throw new Error('route snapshot required');
const route = await Bun.file(file).json();
const poller = route.services?.['coprocessor1-host-listener-poller'];
if (!poller) throw new Error('isolated victim poller required');
poller.command = artifactReplayRoute(poller.command, process.argv[3] ?? '');
console.log(JSON.stringify(route));
