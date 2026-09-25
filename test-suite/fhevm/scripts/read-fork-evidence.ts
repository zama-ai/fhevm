import { forkEvidence } from "../src/consensus/fork-evidence";
const [runId, caseId] = process.argv.slice(2);
if (!runId || !caseId) throw new Error("run and case required");
console.log(forkEvidence(await Bun.stdin.text(), runId, caseId).join("\n"));
