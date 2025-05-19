const readFileSync = require("node:fs").readFileSync;
const pushMetrics = require("prometheus-remote-write").pushMetrics;

function loadMetrics(file_path) {
  try {
    const data = JSON.parse(readFileSync(file_path, "utf-8"));
    return data;
  } catch (error) {
    throw Error(`Failed to load metrics from JSON: ${error.message}`);
  }
}

let metrics = loadMetrics(process.env.PATH_TO_JSON || "test-results.json");

console.log(JSON.stringify(metrics));

function addPrefix(obj, prefix) {
  return Object.fromEntries(Object.entries(obj).map(([key, value]) => [`${prefix}_${key}`, value]));
}

// Just push some metrics metrics
for (const test of metrics.prometheusMetrics.tests) {
  console.log(test);
  let prom_metrics = {
    duration: test.duration,
    status: test.status == "passed" ? 1 : 0,
  };
  prom_metrics = addPrefix(prom_metrics, test.name.replaceAll(" ", "_"));
  prom_metrics = addPrefix(prom_metrics, test.suite.toLowerCase().replaceAll(" ", "_"));

  let labels = {
    status: test.status,
    namespace: "hackathon-net-sepolia-e2e-tests",
    // NOTE: bad for cardinality
    // service: process.env.SERVICE_NAME || "hackathon-net-sepolia-e2e-tests",
    // run: metrics.prometheusMetrics.run_id,
    // timestamp: String(test.timestamp),
    // test: test.name,
    // suite: test.suite,
  };

  let metrics_result = pushMetrics(prom_metrics, {
    url: process.env.PROMETHEUS_REMOTE_WRITE_ENDPOINT || "http://localhost:9090/api/v1/write",
    labels: labels,
  }).then(()=>{
  console.log("metric pushed");
  })

  console.log(metrics_result);
  ;
}
