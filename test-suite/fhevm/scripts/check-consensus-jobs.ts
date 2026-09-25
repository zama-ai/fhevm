import { failedSelectedJobs } from "../src/consensus/ci";

const legs = (process.env.CONSENSUS_SELECTED_LEGS ?? "").split(",").filter(Boolean);
if (!legs.length) throw new Error("The consensus plan selected no jobs");
const problems = failedSelectedJobs(legs, JSON.parse(process.env.CONSENSUS_JOB_RESULTS ?? "{}"));
if (problems.length) {
  console.error(`Selected consensus jobs did not succeed: ${problems.join(", ")}`);
  process.exit(1);
}
console.log("Every selected consensus job, including its final validity gates, succeeded");
