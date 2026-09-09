import type { FheTestEthersConfig } from '../../fheTest/setup-ethers.js';
import type { WasmMemoryInfo } from '../support/memorySampler.js';

// ---------------------------------------------------------------------------
// Shared scenario contract
// ---------------------------------------------------------------------------
// Every scenario is a tight loop that performs one unit of work per iteration.
// `run.mjs` owns the sampler tick cadence, the iteration/duration bound, and
// error handling — a scenario only needs to describe one iteration and (if
// relevant) how to take a live reading of the WASM memory it touches.

export type ScenarioOptions = {
  readonly config: FheTestEthersConfig;
};

export type ScenarioIterationFn = () => Promise<void>;

export type ScenarioSetupResult = {
  /** Performs exactly one unit of work (e.g. one encrypt+decrypt cycle). */
  readonly iterate: ScenarioIterationFn;
  /** Live tfhe WASM memory reader, if this scenario's iterate() touches the encrypt module. */
  readonly readTfheMemory?: () => WasmMemoryInfo | undefined;
  /** Live tkms WASM memory reader, if this scenario's iterate() touches the decrypt module. */
  readonly readTkmsMemory?: () => WasmMemoryInfo | undefined;
  /** Optional cleanup called once after the loop ends (or on error). */
  readonly teardown?: () => Promise<void>;
};

export type Scenario = {
  readonly name: string;
  readonly description: string;
  /** One-time setup (e.g. create a shared client, fetch a stable on-chain handle) before the loop starts. */
  readonly setup: (options: ScenarioOptions) => Promise<ScenarioSetupResult>;
  /** Default iteration count used when the CLI is not given an explicit bound. */
  readonly defaultIterations: number;
  /**
   * Rough order-of-magnitude estimate of how long `defaultIterations` takes
   * to run, shown alongside it in `--help`. Not a measured benchmark — just
   * enough to tell a fast pure-local loop apart from a slow relayer-bound one.
   */
  readonly defaultIterationsDuration: string;
};
