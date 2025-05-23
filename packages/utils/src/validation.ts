export const uuidRegex =
  /^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$/i

export function validateNanoId(length: number = 21, prefix: string = '') {
  const re = nanoIdRegex(length, prefix)
  return function (value: string) {
    return re.test(value)
  }
}

export function nanoIdRegex(length: number = 21, prefix: string = '') {
  return new RegExp(`^${prefix}[a-z0-9_-]{${length}}$`, 'i')
}
