module.exports = {
  istanbulReporter: ["html", "lcov"],
  providerOptions: {
    mnemonic: process.env.MNEMONIC,
  },
  skipFiles: ["test", "fhevmTemp"],
  mocha: {
    fgrep: "[skip-on-coverage]",
    invert: true,
  },
};
