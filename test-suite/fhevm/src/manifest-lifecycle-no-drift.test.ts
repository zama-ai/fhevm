import { describe, expect, test } from "bun:test";
import { assertHealthyDescriptor } from "./commands/manifest-lifecycle-no-drift";

describe("no-drift manifest evidence", () => {
  const handle = "0x1234";
  const digest = "0xabcd";
  test("requires a computed fixture descriptor with the uploaded digest", () => {
    expect(() => assertHealthyDescriptor([{ handle, status: "computed", ct64_digest: digest }], handle, digest)).not.toThrow();
    for (const rows of [[], [{ handle, status: "uncomputed" }], [{ handle, status: "error" }],
      [{ handle, status: "computed", ct64_digest: "0xffff" }],
      [{ handle, status: "computed", ct64_digest: digest }, { handle, status: "computed", ct64_digest: digest }]]) {
      expect(() => assertHealthyDescriptor(rows, handle, digest)).toThrow();
    }
  });
});
