export const istanbulReporter = ["html", "lcov"];
export const providerOptions = {
  mnemonic: process.env.MNEMONIC,
};
export const skipFiles = ["test"];
export const mocha = {
  fgrep: "[skip-on-coverage]",
  invert: true,
};
