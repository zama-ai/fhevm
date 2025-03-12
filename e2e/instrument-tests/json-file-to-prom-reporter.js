// Custom Mocha reporter to create

// ############# IMPORTS ###########################
// Mocha
const Mocha = require("mocha");
const fs = require("node:fs");
const path = require("path");
const Base = require("mocha").reporters.Base;
const uuid = require("uuid");
const pushMetrics = require("prometheus-remote-write").pushMetrics;

//
// All available hooks from Mocha
// readonly EVENT_HOOK_BEGIN: "hook";
// readonly EVENT_HOOK_END: "hook end";
// readonly EVENT_RUN_BEGIN: "start";
// readonly EVENT_DELAY_BEGIN: "waiting";
// readonly EVENT_DELAY_END: "ready";
// readonly EVENT_RUN_END: "end";
// readonly EVENT_SUITE_BEGIN: "suite";
// readonly EVENT_SUITE_END: "suite end";
// readonly EVENT_TEST_BEGIN: "test";
// readonly EVENT_TEST_END: "test end";
// readonly EVENT_TEST_FAIL: "fail";
// readonly EVENT_TEST_PASS: "pass";
// readonly EVENT_TEST_PENDING: "pending";
// readonly EVENT_TEST_RETRY: "retry";
// readonly STATE_IDLE: "idle";
// readonly STATE_RUNNING: "running";
// readonly STATE_STOPPED: "stopped";

const {
  EVENT_SUITE_BEGIN,
  EVENT_RUN_BEGIN,
  EVENT_RUN_END,
  EVENT_TEST_PENDING,
  EVENT_TEST_END,
  EVENT_TEST_PASS,
  EVENT_TEST_FAIL,
} = Mocha.Runner.constants;

class PrometheusReporter extends Base {
  constructor(runner, options) {
    super(runner);

    this.results = {
      stats: {
        suites: 0,
        tests: 0,
        passes: 0,
        pending: 0,
        failures: 0,
        duration: 0,
        start: null,
        end: null,
      },
      tests: [],
      failures: [],
      passes: [],
      pending: [],
      run_id: null,
    };

    // Get output file path from reporter options or use default
    const reporterOptions = options.reporterOptions || {};
    this.outputFile = reporterOptions.output || "test-results.json";

    runner.on(EVENT_RUN_BEGIN, () => {
      this.results.stats.start = new Date();
      this.run_id = uuid.v4();
      this.results.run_id = this.run_id;
    });

    runner.on(EVENT_SUITE_BEGIN, (suite) => {
      if (suite.root) return;
      this.results.stats.suites++;
    });

    runner.on(EVENT_TEST_END, (test) => {
      this.results.stats.tests++;
    });

    runner.on(EVENT_TEST_PASS, (test) => {
      this.results.stats.passes++;

      const testInfo = {
        title: test.title,
        fullTitle: test.fullTitle(),
        duration: test.duration,
        currentRetry: test.currentRetry(),
        suite: test.parent.title,
        timestamp: new Date().toISOString(),
      };

      this.results.passes.push(testInfo);
      this.results.tests.push({
        ...testInfo,
        status: "passed",
      });
    });

    runner.on(EVENT_TEST_FAIL, (test, err) => {
      this.results.stats.failures++;

      const testInfo = {
        title: test.title,
        fullTitle: test.fullTitle(),
        duration: test.duration,
        currentRetry: test.currentRetry(),
        suite: test.parent.title,
        timestamp: new Date().toISOString(),
        error: {
          message: err.message,
          stack: err.stack,
          type: err.type || err.name,
        },
      };

      this.results.failures.push(testInfo);
      this.results.tests.push({
        ...testInfo,
        status: "failed",
      });
    });

    runner.on(EVENT_TEST_PENDING, (test) => {
      this.results.stats.pending++;

      const testInfo = {
        title: test.title,
        fullTitle: test.fullTitle(),
        currentRetry: test.currentRetry(),
        suite: test.parent.title,
        timestamp: new Date().toISOString(),
      };

      this.results.pending.push(testInfo);
      this.results.tests.push({
        ...testInfo,
        status: "pending",
      });
    });

    runner.on(EVENT_RUN_END, () => {
      this.results.stats.end = new Date();
      this.results.stats.duration = this.results.stats.end - this.results.stats.start;

      // Generate Prometheus-friendly metrics
      const prometheusMetrics = {
        run_id: this.run_id,
        test_suites_total: this.results.stats.suites,
        tests_total: this.results.stats.tests,
        tests_passed_total: this.results.stats.passes,
        tests_failed_total: this.results.stats.failures,
        tests_pending_total: this.results.stats.pending,
        test_duration_seconds: this.results.stats.duration / 1000,
        test_execution_timestamp: Math.floor(this.results.stats.start.getTime() / 1000),
        tests: this.results.tests.map((test) => ({
          name: test.title,
          suite: test.suite,
          status: test.status,
          duration: test.duration ? test.duration / 1000 : 0,
          timestamp: new Date(test.timestamp).getTime() / 1000,
        })),
      };

      // Combine test results and Prometheus metrics
      const output = {
        testResults: this.results,
        prometheusMetrics,
      };

      // Ensure output directory exists
      const outputDir = path.dirname(this.outputFile);
      if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
      }

      // Write results to file
      fs.writeFileSync(this.outputFile, JSON.stringify(output, null, 2), "utf-8");
    });
  }
}

module.exports = PrometheusReporter;
