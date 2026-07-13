import { readFile } from "node:fs/promises";
import { describe, expect, it } from "vitest";

describe("README capability contract", () => {
  it("documents redaction and does not advertise a PostgreSQL collector", async () => {
    const readme = await readFile(new URL("../README.md", import.meta.url), "utf8");
    expect(readme).toContain("recursively redacting secret-bearing fields");
    expect(readme).toContain("intentionally does not connect to PostgreSQL");
    expect(readme).toContain("future adapter boundaries");
    expect(readme).not.toContain("when the DB collector is on");
  });
});
