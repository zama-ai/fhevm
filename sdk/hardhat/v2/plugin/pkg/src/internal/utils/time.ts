/** Current POSIX time in seconds. Ported from `@fhevm/mock-utils`' `utils.timestampNow`. */
export function timestampNow(): number {
  return Math.floor(Date.now() / 1000);
}
