import { readFileSync } from 'fs'
import { parse } from 'yaml'

function deepMerge(dst: Record<string, any>, src: Record<string, any>) {
  for (let key in src) {
    if (!src.hasOwnProperty(key)) continue
    if (key === '__proto__' || key === 'constructor') continue
    if (src[key] instanceof Object && dst[key] instanceof Object) {
      dst[key] = deepMerge(dst[key], src[key])
    } else {
      dst[key] = src[key]
    }
  }

  return dst
}

export default () => {
  const runMode = process.env.RUN_MODE || 'local'
  return deepMerge(
    parse(readFileSync(`./config/${runMode}.yaml`, 'utf8')),
    processEnvVariables(),
  )
}

function toCamelCase(str: string, separator: string = '_'): string {
  return str
    .toLowerCase()
    .split(separator)
    .reduce((acc, t) => acc + t[0].toUpperCase() + t.slice(1))
}

function nestingVar(
  keys: string[],
  value: any,
  separator: string = '_',
): Record<string, any> {
  const current = toCamelCase(keys[0], separator)
  return keys.length === 1
    ? { [current]: value }
    : { [current]: nestingVar(keys.slice(1), value) }
}

function processEnvVariables(config?: {
  prefix?: string
  keySeparator?: string
  wordSeparator?: string
}): Record<string, any> {
  const prefix = config?.prefix || 'APP_'
  const keySeparator = config?.keySeparator || '__'
  const wordSeparator = config?.wordSeparator || '_'

  let r: Record<string, any> = {}
  for (const key in process.env) {
    if (key.startsWith(prefix)) {
      const keys = key.split(keySeparator).slice(1)
      r = deepMerge(r, nestingVar(keys, process.env[key], wordSeparator))
    }
  }

  return r
}
