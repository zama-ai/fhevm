import { open, readFile, type FileHandle } from "node:fs/promises";
import { dirname } from "node:path";
import { mkdir } from "node:fs/promises";

/** Awaited, serialized JSONL writer with durable orderly close. */
export class JsonlWriter<T> {
  private tail = Promise.resolve();
  private failure: unknown;
  private closed = false;

  private constructor(
    private readonly path: string,
    private readonly handle: FileHandle,
  ) {}

  static async open<T>(path: string): Promise<JsonlWriter<T>> {
    await mkdir(dirname(path), { recursive: true });
    return new JsonlWriter<T>(path, await open(path, "a", 0o600));
  }

  write(record: T): Promise<void> {
    if (this.closed) return Promise.reject(new Error(`JSONL writer for ${this.path} is closed.`));
    const operation = this.tail.then(async () => {
      if (this.failure) throw this.failure;
      await this.handle.writeFile(`${JSON.stringify(record)}\n`, "utf8");
    });
    this.tail = operation.catch((error: unknown) => {
      this.failure ??= error;
    });
    return operation;
  }

  async close(): Promise<void> {
    if (this.closed) return;
    this.closed = true;
    await this.tail;
    let primary = this.failure;
    if (!primary) {
      try {
        await this.handle.sync();
      } catch (error) {
        primary = error;
      }
    }
    try {
      await this.handle.close();
    } catch (error) {
      if (primary) throw new AggregateError([primary, error], `Failed to close ${this.path}`);
      throw error;
    }
    if (primary) throw primary;
  }
}

/** Reads a whole JSONL file into memory. Missing files yield an empty list. */
export const readJsonl = async <T>(path: string): Promise<T[]> => {
  let text: string;
  try {
    text = await readFile(path, "utf8");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return [];
    throw error;
  }
  return text
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .map((line) => JSON.parse(line) as T);
};
