import { toCamelCase } from './to-camel-case.js'

export function nestingVar(
  keys: string[],
  value: any,
  separator: string = '_',
): Record<string, any> {
  const current = toCamelCase(keys[0], separator)
  if (!current) return {}
  return keys.length === 1
    ? { [current]: value }
    : { [current]: nestingVar(keys.slice(1), value) }
}
