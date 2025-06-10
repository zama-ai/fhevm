export function toCamelCase(
  str: string | undefined,
  separator: string = '_',
): string | undefined {
  return str
    ?.toLowerCase()
    .split(separator)
    .reduce((acc, t) => acc + t[0].toUpperCase() + t.slice(1))
}
