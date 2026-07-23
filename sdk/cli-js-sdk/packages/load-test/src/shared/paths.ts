import { lstat } from "node:fs/promises";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { z } from "zod";

/** Stable file-key syntax used for scenario names, suite names, and labels. */
export const artifactSlugSchema = z
  .string()
  .min(1)
  .max(128)
  .regex(/^[A-Za-z0-9][A-Za-z0-9._-]*$/, {
    message: "must be a safe slug containing only letters, digits, '.', '_', and '-'",
  })
  .refine((value) => value !== "." && value !== "..", {
    message: "must not be '.' or '..'",
  });

export const safeJoin = (root: string, ...components: readonly string[]): string => {
  const absoluteRoot = resolve(root);
  const candidate = resolve(absoluteRoot, ...components);
  const displacement = relative(absoluteRoot, candidate);
  if (
    displacement === ".." ||
    displacement.startsWith(`..${process.platform === "win32" ? "\\" : "/"}`) ||
    isAbsolute(displacement)
  ) {
    throw new Error(`Resolved path escapes its configured root: ${candidate}`);
  }
  return candidate;
};

/**
 * Lexically contains a path and rejects every existing symlink component
 * below the configured root. Suitable for artifact reads and publications
 * where following a pre-planted link could escape the intended tree.
 */
export const safeJoinNoSymlinks = async (
  root: string,
  ...components: readonly string[]
): Promise<string> => {
  const absoluteRoot = resolve(root);
  const candidate = safeJoin(absoluteRoot, ...components);
  const displacement = relative(absoluteRoot, candidate);
  if (displacement === "") return candidate;

  let current = absoluteRoot;
  for (const component of displacement.split(sep)) {
    current = join(current, component);
    try {
      const entry = await lstat(current);
      if (entry.isSymbolicLink()) {
        throw new Error(`Refusing symlinked artifact path component: ${current}`);
      }
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") break;
      throw error;
    }
  }
  return candidate;
};
