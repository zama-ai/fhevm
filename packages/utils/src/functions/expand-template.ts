const regex = /(?:%{{( *\w+ *)}})/gm

/**
 * It replaces placeholders with env variables.
 *
 * @param template Template to exampe with env variables
 * @returns Expanded template
 */
export function expandTemplate(template: string | undefined | null): string {
  if (!template) return ''
  return template.replace(regex, (_match, key) => {
    key = key.trim()
    if (!(key in process.env)) {
      throw new Error(`No env variable found for ${key}`)
    }
    return process.env[key] || ''
  })
}
