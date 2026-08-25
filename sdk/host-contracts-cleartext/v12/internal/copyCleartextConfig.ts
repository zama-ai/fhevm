// Copies internal/cleartext-config.ts into the payload, and verifies the copy.
//
// The two files must be byte-for-byte identical; see the header of cleartext-config.ts for why the shared
// config is duplicated rather than imported. This module owns both directions:
//
//   copyCleartextConfig()   writes the payload copy   — `npm run generate:cleartext-config`
//   checkCleartextConfig()  compares the two          — `npm run check:cleartext-config`
//
// Byte comparison, not a parse: the point is that the payload compiles the same text the harness does, so
// anything that could rewrite one side and not the other — prettier, an editor, a hand-edit of the copy —
// has to show up as a failure. Reading both as Buffers keeps line endings and trailing whitespace in
// scope, which a string comparison after any normalization would quietly forgive.

import { copyFileSync, existsSync, readFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { PACKAGE_ROOT_ABS_PATH, PKG_DIR_ABS_PATH } from './constants.ts';

////////////////////////////////////////////////////////////////////////////////

/** The source of truth. Edited by hand; never generated. */
export const CLEARTEXT_CONFIG_SOURCE_PATH = join(PACKAGE_ROOT_ABS_PATH, 'internal', 'cleartext-config.ts');

/** The generated payload copy. Committed, like pkg/ts/artifacts and pkg/ts/signers. */
export const CLEARTEXT_CONFIG_PAYLOAD_PATH = join(PKG_DIR_ABS_PATH, 'ts', 'cleartext-config.ts');

export type CleartextConfigCopyStatus =
  { readonly status: 'identical' } | { readonly status: 'missing' } | { readonly status: 'different' };

////////////////////////////////////////////////////////////////////////////////

/** Path of `absolutePath` relative to the package root, for messages. */
export function packageRelative(absolutePath: string): string {
  return relative(PACKAGE_ROOT_ABS_PATH, absolutePath);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Writes the payload copy, overwriting whatever is there.
 *
 * `copyFileSync` rather than read-then-write: it cannot introduce an encoding or newline difference, which
 * is the one thing this function must not do.
 */
export function copyCleartextConfig(): void {
  if (!existsSync(CLEARTEXT_CONFIG_SOURCE_PATH)) {
    throw new Error(
      `${packageRelative(CLEARTEXT_CONFIG_SOURCE_PATH)} not found — it is the source of truth for the ` +
        `cleartext config and is committed, not generated.`,
    );
  }

  copyFileSync(CLEARTEXT_CONFIG_SOURCE_PATH, CLEARTEXT_CONFIG_PAYLOAD_PATH);
}

////////////////////////////////////////////////////////////////////////////////

/** Compares the payload copy against the source of truth, byte for byte. */
export function checkCleartextConfig(): CleartextConfigCopyStatus {
  if (!existsSync(CLEARTEXT_CONFIG_PAYLOAD_PATH)) {
    return { status: 'missing' };
  }

  const source = readFileSync(CLEARTEXT_CONFIG_SOURCE_PATH);
  const payload = readFileSync(CLEARTEXT_CONFIG_PAYLOAD_PATH);

  return { status: source.equals(payload) ? 'identical' : 'different' };
}
