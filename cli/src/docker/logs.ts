const DEFAULT_LOG_LINES = 20;

export function toLogLines(raw: string, maxLines = DEFAULT_LOG_LINES): string[] {
  return raw
    .split("\n")
    .map((line) => line.trimEnd())
    .filter(Boolean)
    .slice(-maxLines);
}
