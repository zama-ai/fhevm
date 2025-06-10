import { readFileSync } from 'fs'
import { deepMerge, expandTemplate, processEnvVariables } from 'utils'
import { parse } from 'yaml'

export default () => {
  const runMode = process.env.RUN_MODE || 'local'
  const yaml = expandTemplate(readFileSync(`./config/${runMode}.yaml`, 'utf8'))
  return deepMerge(parse(yaml), processEnvVariables())
}
