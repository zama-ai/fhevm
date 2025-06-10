export function hasAllIntegerKeys(obj: object): boolean {
  return Object.keys(obj).every(key => {
    return Number.isInteger(parseInt(key))
  })
}

export function objectToArray(obj: object): any[] {
  return Object.values(obj)
}
