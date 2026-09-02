// Paths the harness resolves from its own location, so no script depends on the caller's CWD.

import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

/** The harness root — the directory holding this package's package.json. */
export const PACKAGE_ROOT_ABS_PATH = join(dirname(fileURLToPath(import.meta.url)), '..');

/** The published payload's root. Everything under it ships; everything beside it does not. */
export const PKG_DIR_ABS_PATH = join(PACKAGE_ROOT_ABS_PATH, 'pkg');

/**
 * The two entries `pkg/package.json` advertises, as absolute paths.
 *
 * Kept here rather than read out of the manifest at each use: they are what the build must produce and
 * what a node10 consumer resolves, so a test can assert them without reimplementing manifest parsing.
 */
export const CJS_ENTRY_ABS_PATH = join(PKG_DIR_ABS_PATH, '_cjs', 'index.js');
export const TYPES_ENTRY_ABS_PATH = join(PKG_DIR_ABS_PATH, '_types', 'index.d.ts');
