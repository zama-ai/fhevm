const RESET = "\u001b[0m";
const GREEN = "\u001b[32m";
const RED = "\u001b[31m";
const YELLOW = "\u001b[33m";
const BOLD = "\u001b[1m";
const DIM = "\u001b[2m";

export function isTTY(): boolean {
  return process.stdout.isTTY === true;
}

export function isCI(): boolean {
  return process.env.CI === "true" || process.env.GITHUB_ACTIONS === "true";
}

export function green(text: string): string {
  return colorize(text, GREEN);
}

export function red(text: string): string {
  return colorize(text, RED);
}

export function yellow(text: string): string {
  return colorize(text, YELLOW);
}

export function bold(text: string): string {
  return colorize(text, BOLD);
}

export function dim(text: string): string {
  return colorize(text, DIM);
}

export function pass(label: string, detail?: string): string {
  return formatStatus("PASS", "✓", green, label, detail);
}

export function fail(label: string, detail?: string): string {
  return formatStatus("FAIL", "✗", red, label, detail);
}

export function warn(label: string, detail?: string): string {
  return formatStatus("WARN", "⚠", yellow, label, detail);
}

function canColor(): boolean {
  return isTTY() && !isCI();
}

function colorize(text: string, code: string): string {
  return canColor() ? `${code}${text}${RESET}` : text;
}

function formatStatus(
  plain: string,
  symbol: string,
  color: (text: string) => string,
  label: string,
  detail?: string,
): string {
  const prefix = canColor() ? color(symbol) : `[${plain}]`;
  return detail ? `${prefix} ${label}: ${detail}` : `${prefix} ${label}`;
}
