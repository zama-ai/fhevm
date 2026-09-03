/** Seconds since the Unix epoch, the unit decryption permits are dated in. */
export function timestampNow(): number {
  return Math.floor(Date.now() / 1000);
}
