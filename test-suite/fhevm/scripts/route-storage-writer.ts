#!/usr/bin/env bun
import { storageRoute } from '../src/consensus/storage-write-route';
const [snapshot, url] = process.argv.slice(2);
const route = storageRoute(await Bun.file(snapshot!).json(), url);
console.log(url ? JSON.stringify(route.override) : route.upstream);
