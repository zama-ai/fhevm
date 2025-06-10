import { deepMerge } from './deep-merge.js'
import { nestingVar } from './nesting-var.js'
import { hasAllIntegerKeys } from './object-to-array.js'

export function processEnvVariables(config?: {
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

  return mergeArrayNode(r)
}

function mergeArrayNode(obj: object): object {
  if (typeof obj === 'object' && obj !== null) {
    return Object.keys(obj).length && hasAllIntegerKeys(obj)
      ? Object.values(obj).map(mergeArrayNode)
      : Object.fromEntries(
          Object.entries(obj).map(([key, value]) => [
            key,
            mergeArrayNode(value),
          ]),
        )
  } else {
    return obj
  }
}
