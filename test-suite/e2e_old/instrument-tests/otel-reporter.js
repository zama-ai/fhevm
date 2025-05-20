// Custom Mocha reporter to create

// ############# IMPORTS ###########################
const opentelemetry = require("@opentelemetry/api");
const { ATTR_SERVICE_NAME } = require("@opentelemetry/semantic-conventions");
const { diag, DiagConsoleLogger, DiagLogLevel } = require("@opentelemetry/api");
const { Resource } = require("@opentelemetry/resources");

// Metrics
const { OTLPMetricExporter } = require("@opentelemetry/exporter-metrics-otlp-http");
const { PeriodicExportingMetricReader } = require("@opentelemetry/sdk-metrics");
const { MeterProvider } = require("@opentelemetry/sdk-metrics");

// Mocha
const Mocha = require("mocha");

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
  EVENT_SUITE_END,
  EVENT_RUN_BEGIN,
  EVENT_RUN_END,
  EVENT_TEST_BEGIN,
  EVENT_TEST_PASS,
  EVENT_TEST_FAIL,
} = Mocha.Runner.constants;

const version = "0.1.0";
const name = "mocha-reporter";

// https://mochajs.org/#run-cycle-overview
// RUN -> SUITE -> TEST
// Create custom Mocha reporter that add open-telemetry metrics

class OTelReporter {
  constructor(runner) {
    // Mocha runner statistics
    const stats = runner.stats;

    // Resource
    const resource = Resource.default().merge(
      new Resource({
        [ATTR_SERVICE_NAME]: "e2e-tests",
      }),
    );

    // Define exporter
    const metric_exporter = new OTLPMetricExporter();
    // const metric_exporter = new ConsoleMetricExporter();

    // Define reader and provider
    const metric_reader = new PeriodicExportingMetricReader({
      exporter: metric_exporter,
      // Default is 60000ms (60 seconds). Set to 10ms since we don't want to hold the process for ever
      exportIntervalMillis: 10,
    });

    const meter_provider = new MeterProvider({
      resource: resource,
      readers: [metric_reader],
    });

    opentelemetry.metrics.setGlobalMeterProvider(meter_provider);

    // Init meter and tracer
    const meter = opentelemetry.metrics.getMeter(name, version);

    // Create counters and histograms
    const testCounter = meter.createCounter("test.total", {
      description: "Total number of tests run",
    });

    const testDurationHistogram = meter.createHistogram("test.duration", {
      description: "Test execution duration",
      unit: "ms",
    });
    const testResultCounter = meter.createCounter("test.results", {
      description: "Test results by status",
    });

    // Run
    runner.on(EVENT_RUN_BEGIN, () => {});

    runner.on(EVENT_RUN_END, () => {
      // Setup summary Gauge
      meter
        .createObservableGauge("test.summary", {
          description: "Test suite summary metrics",
        })
        .addCallback((result) => {
          result.observe(stats.passes, { metric: "passes" });
          result.observe(stats.failures, { metric: "failures" });
          result.observe(stats.pending, { metric: "pending" });
          result.observe(stats.duration, { metric: "duration_ms" });
        });
      function sleep(ms) {
        return new Promise((resolve) => {
          setTimeout(resolve, ms);
        });
      }
      sleep(100);

      // NOTE: https://github.com/open-telemetry/opentelemetry-specification/issues/2983
      metric_reader.collect();
      metric_reader.forceFlush();
      metric_exporter.forceFlush().then(
        () => {
          console.log("Metric exporter force flush success");
        },
        (reason) => {
          console.log("Metric exporter force flush failed.");
          console.error(reason);
        },
      );
      sleep(100);
    });
    // Suite
    runner.on(EVENT_SUITE_BEGIN, (suite) => {});

    runner.on(EVENT_SUITE_END, (suite) => {});

    // Test
    runner.on(EVENT_TEST_BEGIN, (test) => {});

    runner.on(EVENT_TEST_PASS, (test) => {
      testResultCounter.add(1, { result: "pass", name: test.title, suite: test.parent.title });
      testDurationHistogram.record(test.duration, {
        suite: test.parent.title,
        test: test.title,
      });
      testCounter.add(1);
    });

    runner.on(EVENT_TEST_FAIL, (test) => {
      testResultCounter.add(1, { result: "fail", name: test.title, suite: test.parent.title });
      testCounter.add(1);
    });
  }
}

module.exports = OTelReporter;
