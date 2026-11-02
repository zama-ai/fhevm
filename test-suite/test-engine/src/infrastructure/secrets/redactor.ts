/**
 * Replaces known secret values with `[REDACTED:<key>]` in text and JSON-like data.
 *
 * Every output that can leave the engine (console, Cucumber artifacts, reports, evidence) goes
 * through a redactor built from the resolved profile, so that nothing unredacted reaches the disk.
 */

export interface SecretValue {
  /** Setting key, used in the placeholder so readers know which secret was removed. */
  key: string;
  value: string;
}

export interface Redactor {
  /** True when there is at least one secret to redact. */
  readonly active: boolean;
  redact(text: string): string;
  /** Deep copy of `value` with every string redacted (object keys included). */
  redactDeep<T>(value: T): T;
}

/** Shortest secret the engine accepts; shorter values would make substring redaction mangle output. */
export const MIN_SECRET_LENGTH = 8;

const escapeRegExp = (text: string) => text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

/**
 * Builds a redactor matching each secret in its raw, JSON-escaped and URL-encoded forms, which
 * is how secrets usually show up inside error messages and serialised messages.
 */
export function createRedactor(secrets: SecretValue[]): Redactor {
  const placeholders = new Map<string, string>();
  for (const { key, value } of secrets) {
    if (value.length < MIN_SECRET_LENGTH) {
      throw new Error(`secret '${key}' is shorter than ${MIN_SECRET_LENGTH} characters`);
    }
    const placeholder = `[REDACTED:${key}]`;
    for (const form of [value, JSON.stringify(value).slice(1, -1), encodeURIComponent(value)]) {
      if (!placeholders.has(form)) placeholders.set(form, placeholder);
    }
  }
  // Longest forms first, so that a secret containing another secret is replaced as a whole.
  const forms = [...placeholders.keys()].sort((a, b) => b.length - a.length);
  const pattern = forms.length > 0 ? new RegExp(forms.map(escapeRegExp).join('|'), 'g') : undefined;

  const redact = (text: string): string =>
    pattern ? text.replace(pattern, (match) => placeholders.get(match) ?? match) : text;

  const redactDeep = <T>(value: T): T => {
    if (!pattern) return value;
    if (typeof value === 'string') return redact(value) as T;
    if (Array.isArray(value)) return value.map(redactDeep) as T;
    if (value !== null && typeof value === 'object') {
      return Object.fromEntries(
        Object.entries(value as Record<string, unknown>).map(([key, entry]) => [redact(key), redactDeep(entry)]),
      ) as T;
    }
    return value;
  };

  return { active: pattern !== undefined, redact, redactDeep };
}

/** Line-buffered redaction for streams: a secret split across two chunks is still redacted. */
export class RedactingLineBuffer {
  private pending = '';

  constructor(
    private readonly redactor: Redactor,
    private readonly write: (text: string) => void,
    /** Above this size a line without newline is flushed anyway (secrets spanning it are not caught). */
    private readonly maxPending = 1024 * 1024,
  ) {}

  push(chunk: string): void {
    this.pending += chunk;
    const lastNewline = this.pending.lastIndexOf('\n');
    if (lastNewline >= 0) {
      this.write(this.redactor.redact(this.pending.slice(0, lastNewline + 1)));
      this.pending = this.pending.slice(lastNewline + 1);
    }
    if (this.pending.length > this.maxPending) this.flush();
  }

  flush(): void {
    if (this.pending !== '') this.write(this.redactor.redact(this.pending));
    this.pending = '';
  }
}
