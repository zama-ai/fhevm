// The smallest hardhat project that carries the plugin: what `hardhat node` is started with in the
// child-process test. Loaded with `--config`, so the project root is the plugin owner directory.

import { defineConfig } from 'hardhat/config';

import plugin from '#esm/index.js';

export default defineConfig({ plugins: [plugin] });
