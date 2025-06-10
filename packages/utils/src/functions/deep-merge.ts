export function deepMerge(dst: Record<string, any>, src: Record<string, any>) {
  for (const key in src) {
    // eslint-disable-next-line no-prototype-builtins
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
