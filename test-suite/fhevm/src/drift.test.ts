import { describe, expect, test } from "bun:test";

import { DRIFT_CLEANUP_SQL, DRIFT_INSTALL_SQL } from "./drift";

describe("ciphertext drift injector SQL", () => {
  test("covers legacy and branch-scoped publication tables", () => {
    expect(DRIFT_INSTALL_SQL).toContain("BEFORE UPDATE ON ciphertext_digest");
    expect(DRIFT_INSTALL_SQL).toContain(
      "BEFORE UPDATE ON ciphertext_digest_branch",
    );
    expect(DRIFT_INSTALL_SQL).toContain(
      "SELECT 1 FROM computations_branch WHERE output_handle = $1",
    );
  });

  test("removes both publication triggers", () => {
    expect(DRIFT_CLEANUP_SQL).toContain("ciphertext_drift_injector_branch");
    expect(DRIFT_CLEANUP_SQL).toContain(
      "ciphertext_drift_injector ON ciphertext_digest",
    );
  });
});
