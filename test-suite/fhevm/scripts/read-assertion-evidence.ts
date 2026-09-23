import { existsSync, readFileSync } from "node:fs";

const [file, runId, caseId] = process.argv.slice(2);
if (!file || !runId || !caseId) throw new Error("missing assertion evidence identity");
if (existsSync(file)) {
  for (const line of readFileSync(file, "utf8").split("\n").filter(Boolean)) {
    const receipt = JSON.parse(line);
    if (!receipt || typeof receipt !== "object" || typeof receipt.runId !== "string" || typeof receipt.caseId !== "string") {
      throw new Error("malformed assertion evidence identity");
    }
    if (receipt.runId !== runId || receipt.caseId !== caseId) continue;
    if (typeof receipt.name !== "string" || !/^[a-z][a-z0-9-]*$/.test(receipt.name) || receipt.outcome !== "pass" ||
        typeof receipt.detail !== "string" || !receipt.detail.trim() || /[\r\n]/.test(receipt.detail)) {
      throw new Error("invalid or failed assertion evidence");
    }
    console.log(`assert=${receipt.name}=pass:${receipt.detail}`);
  }
}
