export const uuidRegex =
  /^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$/i

export function validateNanoId(length: number = 21, prefix: string = '') {
  const re = new RegExp(`^${prefix}[a-zA-Z0-9_-]{${length}}$`, 'i')
  return function (value: string) {
    return re.test(value)
  }
}
