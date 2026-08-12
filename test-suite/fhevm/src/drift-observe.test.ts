import { expect, test } from "bun:test";

import { findDriftObservations, formatDriftScan, gwListenerContainerNames } from "./drift";

// A real peer-divergence line, as gw-listener emits it.
const PEER_LINE =
  '{"timestamp":"2026-08-10T09:44:01Z","level":"WARN","message":"Drift detected: observed multiple digest variants for handle","handle":"0xabc123"}';
const CONSENSUS_LINE =
  '{"timestamp":"2026-08-10T09:44:02Z","level":"WARN","message":"Drift detected: local digest does not match consensus","handle":"0xDEF456"}';

test("finds nothing in a log with no drift warnings", () => {
  const output = ['{"level":"INFO","message":"submitted ciphertext material"}', ""].join("\n");
  expect(findDriftObservations("coprocessor1-gw-listener", output)).toEqual([]);
});

test("distinguishes peer divergence from a consensus mismatch", () => {
  const observations = findDriftObservations("coprocessor2-gw-listener", [PEER_LINE, CONSENSUS_LINE].join("\n"));
  expect(observations.map((entry) => entry.kind)).toEqual(["peer-divergence", "consensus-mismatch"]);
  expect(observations.map((entry) => entry.container)).toEqual([
    "coprocessor2-gw-listener",
    "coprocessor2-gw-listener",
  ]);
});

// The handle is what makes a warning actionable — it is the thing an operator greps for next.
test("extracts the handle regardless of hex case", () => {
  expect(findDriftObservations("coprocessor1-gw-listener", PEER_LINE)[0]?.handle).toBe("abc123");
  expect(findDriftObservations("coprocessor1-gw-listener", CONSENSUS_LINE)[0]?.handle).toBe("DEF456");
});

// A warning without a handle field is still a warning; dropping it would hide real drift.
test("keeps a warning that carries no handle", () => {
  const line = '{"message":"Drift detected: local digest does not match consensus"}';
  const observations = findDriftObservations("coprocessor1-gw-listener", line);
  expect(observations).toHaveLength(1);
  expect(observations[0]?.handle).toBeUndefined();
});

test("selects only gateway listeners from docker ps output", () => {
  const output = [
    "coprocessor1-gw-listener",
    "coprocessor2-gw-listener",
    "coprocessor-gw-listener",
    "coprocessor1-tfhe-worker",
    "coprocessor1-host-listener",
    "kms-node-1-core",
    "  coprocessor3-gw-listener  ",
    "",
  ].join("\n");
  expect(gwListenerContainerNames(output)).toEqual([
    "coprocessor1-gw-listener",
    "coprocessor2-gw-listener",
    "coprocessor-gw-listener",
    "coprocessor3-gw-listener",
  ]);
});

test("reports a clean scan with the number of listeners that were actually read", () => {
  const summary = formatDriftScan("coprocessor 2/5 on 0.14", {
    containers: ["coprocessor1-gw-listener", "coprocessor2-gw-listener"],
    observations: [],
  });
  expect(summary).toBe("drift scan coprocessor 2/5 on 0.14: clean across 2 listeners");
});

test("names every distinct handle when a scan is dirty", () => {
  const summary = formatDriftScan("coprocessor 1/5 on 0.14", {
    containers: ["coprocessor1-gw-listener"],
    observations: [
      { container: "coprocessor1-gw-listener", kind: "peer-divergence", handle: "abc123", line: PEER_LINE },
      { container: "coprocessor1-gw-listener", kind: "consensus-mismatch", handle: "abc123", line: CONSENSUS_LINE },
    ],
  });
  expect(summary).toBe("drift scan coprocessor 1/5 on 0.14: 2 warning(s) across 1 listener, handles abc123");
});
