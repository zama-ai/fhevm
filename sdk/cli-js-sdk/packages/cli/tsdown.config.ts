import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["index.ts"],
  format: "esm",
  platform: "node",
  target: "node22",
  outDir: "dist",
  clean: true,
  dts: false,
  // The toolkit ships TypeScript source (exports point at .ts files), so it
  // must be bundled into the compiled CLI instead of resolved at runtime.
  // Its npm dependencies stay external and therefore must be declared as
  // dependencies of this package.
  deps: {
    alwaysBundle: [/^@cli-fhevm-sdk\/toolkit(\/|$)/],
  },
});
