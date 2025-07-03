export function shortString(
  str: string | undefined,
  length: number = 20,
): string | undefined {
  if (!str) return
  return str.length > length
    ? `${str.slice(0, length - 6)}...${str.slice(-3)}`
    : str
}
