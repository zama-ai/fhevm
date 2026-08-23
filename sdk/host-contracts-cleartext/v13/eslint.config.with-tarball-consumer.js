// Config for `lint:tarball-consumer`: the base rules with NO ignores, so the `test/ts` files that
// import the package by its published name are linted too. Only valid once
// `npm run prepare:tarball-consumer` has installed the tarball fixture — without it the type-aware
// project service cannot resolve those imports and reports ~186 spurious errors.
//
// Three configs, one job each:
//   eslint.config.base.js                  the shared rule set (never run directly)
//   eslint.config.js                       base + ignores — the DEFAULT, safe on a bare checkout
//   eslint.config.with-tarball-consumer.js base as-is — the stricter gate, needs the fixture
import baseConfig from './eslint.config.base.js';

export default [...baseConfig];
