// leg — runs one chain's leg of a parity case over every value set, in this process alone, and
// writes the reads to the result file the parent named. Usage: bun leg.ts <case file> <chain> <result file>

import { writeFile } from "node:fs/promises";

import { CHAINS, runLeg, type Chain, type LegRun, type ParityCase } from "./parity";

const [caseFile, chain, resultFile] = process.argv.slice(2);
if (caseFile === undefined || resultFile === undefined || !CHAINS.includes(chain as Chain)) {
  throw new Error("usage: bun leg.ts <case file> <evm|solana> <result file>");
}
const spec = (await import(caseFile)).parityCase as ParityCase<Record<string, bigint>>;
const runs: LegRun[] = [];
for (const values of spec.cases) runs.push(await runLeg(spec[chain as Chain], values));
// Written once, after every value set ran: the parent treats a missing file as a failed leg.
await writeFile(resultFile, JSON.stringify(runs));
// Exit now: an SDK worker pool or RPC subscription left open must not hold the child past its result.
process.exit(0);
