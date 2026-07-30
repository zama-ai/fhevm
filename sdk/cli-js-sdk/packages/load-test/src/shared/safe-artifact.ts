const REDACTED = "[REDACTED]";
const MAX_ERROR_TEXT = 500;

const sensitiveKey = (key: string): boolean =>
  /(?:^|[_-])(?:api[_-]?key|authorization|credential|mnemonic|pass(?:word|phrase)?|private[_-]?key|secret|signature|token|transport[_-]?key)(?:$|[_-])/i
    .test(key.replace(/([a-z])([A-Z])/g, "$1_$2"));

/** Recursively removes secret-bearing values while retaining useful structure. */
export const redactSensitiveData = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(redactSensitiveData);
  if (typeof value === "string") return redactTextPatterns(value);
  if (typeof value !== "object" || value === null) return value;
  const redacted: Record<string, unknown> = {};
  for (const [key, child] of Object.entries(value)) {
    redacted[key] = sensitiveKey(key) ? REDACTED : redactSensitiveData(child);
  }
  return redacted;
};

const redactTextPatterns = (text: string): string =>
  text
    .replace(
      /([a-z][a-z0-9+.-]*:\/\/[^\s:/]+:)[^@\s/]+@/gi,
      `$1${REDACTED}@`,
    )
    .replace(/\bBearer\s+[^\s,;]+/gi, `Bearer ${REDACTED}`)
    .replace(
      /((?:api[_-]?key|authorization|credential|mnemonic|pass(?:word|phrase)?|private[_-]?key|secret|signature|token|transport[_-]?key)\s*[=:]\s*)(["']?)[^\s,}\]\r\n]+\2/gi,
      `$1${REDACTED}`,
    )
    .replace(/\b(?:0x)?[0-9a-f]{64,}\b/gi, REDACTED);

/** Redacts JSON structurally, with a conservative text fallback for TOML/YAML. */
export const redactConfigText = (text: string): string => {
  try {
    return `${JSON.stringify(redactSensitiveData(JSON.parse(text)), null, 2)}\n`;
  } catch {
    return text
      .split(/(?<=\n)/)
      .map((line) => {
        const assignment = /^(\s*)([A-Za-z0-9_.-]+)(\s*[:=]\s*)(.*?)(\r?\n)?$/.exec(line);
        if (!assignment || !sensitiveKey(assignment[2] ?? "")) {
          return redactTextPatterns(line);
        }
        return `${assignment[1] ?? ""}${assignment[2] ?? ""}${assignment[3] ?? ""}${REDACTED}${assignment[5] ?? ""}`;
      })
      .join("");
  }
};

/** Safe, bounded text for reports and JSONL request records. */
export const safeArtifactText = (
  value: unknown,
  maxLength = MAX_ERROR_TEXT,
): string | undefined => {
  if (value === undefined || value === null) return undefined;
  const source = typeof value === "string" ? value : String(value);
  let safe: string;
  try {
    safe = JSON.stringify(redactSensitiveData(JSON.parse(source)));
  } catch {
    safe = redactTextPatterns(source);
  }
  if (safe.length <= maxLength) return safe;
  return `${safe.slice(0, Math.max(0, maxLength - 1))}…`;
};
