module.exports = {
  istanbulReporter: ["html", "lcov"],
  providerOptions: {
    mnemonic: process.env.MNEMONIC,
  },
  skipFiles: ["test"],
  mocha: {
    fgrep: "[skip-on-coverage]",
    invert: true,
  },
};
