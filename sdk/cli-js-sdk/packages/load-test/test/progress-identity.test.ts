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

  it("rejects a public-decrypt terminal request or job identity mismatch", () => {
    const initial = { requestId: "request-a", jobId: "job-a" };
    expect(sdkTerminalIdentityError(initial, {
      type: "succeeded", method: "GET", requestId: "request-b", jobId: "job-a",
    })).toMatch(/did not match/);
    expect(sdkTerminalIdentityError(initial, {
      type: "succeeded", method: "GET", requestId: "request-a", jobId: "job-b",
    })).toMatch(/did not match/);
  });

  it("rejects missing POST acceptance or terminal success identity", () => {
    expect(sdkTerminalIdentityError(undefined, {
      type: "succeeded", method: "GET", requestId: "request-a", jobId: "job-a",
    })).toMatch(/initial POST acceptance/);
    expect(sdkTerminalIdentityError(
      { requestId: "request-a", jobId: "job-a" },
      { type: "succeeded", method: "GET", jobId: "job-a" },
    )).toMatch(/did not match/);
  });
});
