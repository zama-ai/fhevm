import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

export const printJson = (value: unknown): void => {
  process.stdout.write(
    JSON.stringify(
      value,
      (_key, item) => (typeof item === "bigint" ? item.toString() : item),
      2,
    ) + "\n",
  );
};

export const writeJsonFile = async (
  path: string,
  value: unknown,
): Promise<void> => {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(
    path,
    JSON.stringify(
      value,
      (_key, item) => (typeof item === "bigint" ? item.toString() : item),
      2,
    ) + "\n",
    { mode: 0o600 },
  );
};

export const readJsonFile = async (path: string): Promise<unknown> =>
  JSON.parse(await readFile(path, "utf8")) as unknown;
