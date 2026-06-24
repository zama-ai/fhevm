// EXEMPLAR — engine factory. Wires the CLI/runbooks to the concrete Stack implementation.
//
// `createStack()` returns a configured KubectlStack (kind + Helm + kubectl), reading
// deployment specifics from the environment with the proven v0.13 defaults. The CLI
// (stack/cli/main.ts) and runbooks depend ONLY on this factory + the Stack interface, so
// the engine can be swapped without touching the front-ends.

import { KubectlStack, type KubectlStackConfig } from "./kubectl-stack";
import type { Stack } from "./stack";

const TARGET = "v0.13.0-6";

/** Build the engine config from env (proven defaults). */
export const engineConfig = (overrides: Partial<KubectlStackConfig> = {}): KubectlStackConfig => ({
  namespace: process.env.FHEVM_NAMESPACE ?? "fhevm",
  chartsDir: process.env.FHEVM_CHARTS_DIR ?? "charts",
  valuesDir: process.env.FHEVM_VALUES_DIR ?? "stack/values",
  // Host-chain JSON-RPC for chain(): in-cluster svc by default; a port-forward for host runs.
  rpcUrl: process.env.FHEVM_HOST_RPC ?? "http://host-node:8545",
  // chain() execs `cast rpc` in this pod (host-side CLI can't reach the in-cluster RPC).
  anvilPod: process.env.FHEVM_ANVIL_POD ?? "host-anvil-node-0",
  contractsImage: {
    host: process.env.FHEVM_HOST_CONTRACTS_IMAGE ?? `ghcr.io/zama-ai/fhevm/host-contracts:${TARGET}`,
    gateway:
      process.env.FHEVM_GATEWAY_CONTRACTS_IMAGE ?? `ghcr.io/zama-ai/fhevm/gateway-contracts:${TARGET}`,
  },
  testImage: process.env.FHEVM_TEST_IMAGE,
  numCoprocessors: Number(process.env.FHEVM_NUM_COPROCESSORS) || 1,
  ...overrides,
});

/** The concrete engine the CLI + runbooks run against. */
export const createStack = (overrides: Partial<KubectlStackConfig> = {}): Stack =>
  new KubectlStack(engineConfig(overrides));
