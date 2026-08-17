import { describe, expect, it } from "vitest";

import {
  discoverPrometheusCapabilities,
  mappedMetricFamily,
} from "../src/collectors/prom-capabilities";
import { parsePrometheusText } from "../src/collectors/prom-parse";

describe("Prometheus capability discovery", () => {
  it("discovers and normalizes the legacy status families", () => {
    const families = parsePrometheusText(`
# TYPE relayer_request_count gauge
relayer_request_count{req_type="user_decrypt",status="processing"} 3
# TYPE relayer_request_status_duration_seconds histogram
relayer_request_status_duration_seconds_bucket{req_type="user_decrypt",previous_status="queued",le="1"} 2
relayer_request_status_duration_seconds_bucket{req_type="user_decrypt",previous_status="queued",le="+Inf"} 2
`);
    const capabilities = discoverPrometheusCapabilities(families);
    expect(capabilities.profile).toBe("legacy");
    expect(capabilities.signals.queueDepth.available).toBe(true);
    expect(capabilities.signals.e2eDuration.available).toBe(false);
    expect(mappedMetricFamily(families, capabilities, "queueDepth")?.metrics[0]?.labels)
      .toEqual({ flow: "user_decrypt", status: "processing" });
    expect(mappedMetricFamily(families, capabilities, "stageDuration")?.metrics[0]?.labels)
      .toEqual({ flow: "user_decrypt", stage: "queued" });
  });

  it("discovers v2 families without claiming legacy queue signals", () => {
    const families = parsePrometheusText(`
# TYPE input_proof_requests_inserted_total counter
input_proof_requests_inserted_total 1
# TYPE relayer_http_responses_total counter
relayer_http_responses_total{endpoint="/input-proof",method="POST",status="202"} 1
`);
    const capabilities = discoverPrometheusCapabilities(families);
    expect(capabilities.profile).toBe("v2");
    expect(capabilities.signals.queueDepth.available).toBe(false);
    expect(capabilities.signals.stageDuration.available).toBe(false);
    expect(capabilities.signals.httpRequests.available).toBe(true);
    expect(capabilities.signals.v2InputProofInserted.available).toBe(true);
    expect(mappedMetricFamily(families, capabilities, "v2InputProofInserted")?.name)
      .toBe("input_proof_requests_inserted_total");
  });

  it("recognizes the actual v2 wallet-lease transition family", () => {
    const capabilities = discoverPrometheusCapabilities(parsePrometheusText(`
# TYPE relayer_wallet_lease_transitions_total counter
relayer_wallet_lease_transitions_total{result="acquired"} 1
`));
    expect(capabilities.profile).toBe("v2");
    expect(capabilities.signals.v2WalletLeaseTransitions.available).toBe(true);
  });

  it("recognizes shared HTTP metrics without guessing a profile", () => {
    const families = parsePrometheusText(`
# TYPE relayer_http_responses_total counter
relayer_http_responses_total{endpoint="/input-proof",status="202"} 1
`);
    const capabilities = discoverPrometheusCapabilities(families);
    expect(capabilities.profile).toBe("unknown");
    expect(capabilities.signals.httpRequests.available).toBe(true);
    expect(mappedMetricFamily(families, capabilities, "httpRequests")?.name)
      .toBe("relayer_http_responses_total");
  });

  it("leaves every unknown signal explicitly unavailable", () => {
    const capabilities = discoverPrometheusCapabilities(parsePrometheusText("custom_metric 1"));
    expect(capabilities.profile).toBe("unknown");
    expect(Object.values(capabilities.signals).every((signal) => !signal.available)).toBe(true);
    expect(capabilities.signals.queueDepth.reason).toMatch(/No recognized/);
  });
});
