// solidity-coverage disables the optimizer by default during instrumented compilation, which
// trips stack-too-deep on Decryption.sol's unified `userDecryptionRequest` path. `irMinimum`
// keeps the optimizer enabled and uses a minimal Yul pipeline (stackAllocation only), enough
// for coverage to compile without turning `viaIR` on project-wide.
module.exports = {
  irMinimum: true,
};
