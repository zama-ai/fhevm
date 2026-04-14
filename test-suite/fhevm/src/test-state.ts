import path from "node:path";
import { tmpdir } from "node:os";
import { mkdtemp, rm } from "node:fs/promises";

import { DEFAULT_STATE_DIR, setStateDir } from "./layout";

let stateDirLock = Promise.resolve();

/** Runs a test body against an isolated temporary fhevm state root. */
export const withTempStateDir = async <T>(run: (stateDir: string) => Promise<T>) => {
  const previous = process.env.FHEVM_STATE_DIR;
  let release = () => {};
  const previousLock = stateDirLock;
  stateDirLock = new Promise<void>((resolve) => {
    release = resolve;
  });
  await previousLock;
  const stateDir = await mkdtemp(path.join(tmpdir(), "fhevm-test-"));
  process.env.FHEVM_STATE_DIR = stateDir;
  setStateDir(stateDir);
  try {
    return await run(stateDir);
  } finally {
    if (previous === undefined) {
      delete process.env.FHEVM_STATE_DIR;
      setStateDir(DEFAULT_STATE_DIR);
    } else {
      process.env.FHEVM_STATE_DIR = previous;
      setStateDir(previous);
    }
    await rm(stateDir, { recursive: true, force: true });
    release();
  }
};
