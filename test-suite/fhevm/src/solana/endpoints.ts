// The local Solana stack's endpoints, derived once from the port definitions in `layout.ts`. Every
// consumer (harness env defaults, lifecycle health checks, operator, scripts) reads them from
// here; a remote stack (preview namespace) overrides each through the environment instead of
// relying on any port coincidence.
import {
  DEFAULT_GATEWAY_RPC_PORT,
  DEFAULT_HOST_RPC_PORT,
  DEMO_DAPP_PORT,
  DEMO_OPERATOR_PORT,
  RELAYER_PORT,
  SOLANA_LEAF_PROOF_PORT,
  SOLANA_LISTENER_GRPC_PORT,
  SOLANA_LISTENER_HEALTH_PORT,
  SOLANA_VALIDATOR_RPC_PORT,
  SOLANA_VALIDATOR_WS_PORT,
} from "../layout";

const loopback = (port: number, scheme: "http" | "ws" = "http"): string => `${scheme}://127.0.0.1:${port}`;

export const LOCAL_SOLANA_ENDPOINTS = {
  validatorRpc: loopback(SOLANA_VALIDATOR_RPC_PORT),
  validatorWs: loopback(SOLANA_VALIDATOR_WS_PORT, "ws"),
  relayer: loopback(RELAYER_PORT),
  hostRpc: loopback(DEFAULT_HOST_RPC_PORT),
  gatewayRpc: loopback(DEFAULT_GATEWAY_RPC_PORT),
  /** The first coprocessor's leaf-proof server. */
  leafProof: loopback(SOLANA_LEAF_PROOF_PORT),
  /** The first coprocessor's host listener: its health routes, and the Yellowstone gRPC it reads. */
  listenerHealth: loopback(SOLANA_LISTENER_HEALTH_PORT),
  listenerGrpc: loopback(SOLANA_LISTENER_GRPC_PORT),
  demoOperator: loopback(DEMO_OPERATOR_PORT),
  demoDapp: loopback(DEMO_DAPP_PORT),
} as const;

