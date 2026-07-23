import { describe, expect, it } from "vitest";

import {
  captureInitialPostIdentity,
  sdkTerminalIdentityError,
} from "../src/flows/progress-identity";

describe("SDK progress provenance", () => {
  it("retains the initial public-decrypt POST identity across queued GETs", () => {
    const post = { type: "queued", method: "POST", requestId: "request-a", jobId: "job-a" };
    const initial = captureInitialPostIdentity(undefined, post);
    expect(captureInitialPostIdentity(initial, {
      type: "queued", method: "GET", requestId: "request-b", jobId: "job-b",
    })).toEqual({ requestId: "request-a", jobId: "job-a" });
  });

  it("allows per-response request identities while requiring a stable job identity", () => {
    const initial = { requestId: "request-a", jobId: "job-a" };
    expect(sdkTerminalIdentityError(initial, {
      type: "succeeded", method: "GET", requestId: "request-b", jobId: "job-a",
    })).toBeUndefined();
    expect(sdkTerminalIdentityError(initial, {
      type: "succeeded", method: "GET", requestId: "request-a", jobId: "job-b",
    })).toMatch(/job identity did not match/);
  });

  it("rejects missing POST acceptance or terminal success identity", () => {
    expect(sdkTerminalIdentityError(undefined, {
      type: "succeeded", method: "GET", requestId: "request-a", jobId: "job-a",
    })).toMatch(/initial POST acceptance/);
    expect(sdkTerminalIdentityError(
      { requestId: "request-a", jobId: "job-a" },
      { type: "succeeded", method: "GET", jobId: "job-a" },
    )).toMatch(/HTTP request identity/);
  });
});
