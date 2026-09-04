// Package root entry (`@fhevm/sdk`) — mapped by the `.` export / `main` / `module`
// / `types` in package.json. Intentionally empty: the SDK exposes no root API.
// Import from the environment- and capability-specific entry points instead:
//
//   @fhevm/sdk/ethers   — create clients in an ethers.js environment
//   @fhevm/sdk/viem     — create clients in a viem environment
//   @fhevm/sdk/chains   — chain definitions
//   @fhevm/sdk/actions/{host,base,encrypt,decrypt,chain} — capability-scoped actions
//
// This file must still exist so the build emits _esm/index.js, _cjs/index.js and
// _types/index.d.ts that the package.json `.` export points to. The `export {}`
// keeps it a valid (empty) ES module under isolatedModules.
export {};
