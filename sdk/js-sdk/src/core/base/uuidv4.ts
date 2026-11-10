/**
 * Created once: RFC 9562 UUID shape with the version nibble fixed to `4` (UUIDv4)
 * and the variant nibble restricted to `8`/`9`/`a`/`b`.
 */
const UUID_V4_RE = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

/**
 * Returns whether `value` is a UUIDv4 string (RFC 9562): the version nibble is
 * `4` and the variant nibble is `8`, `9`, `a`, or `b`.
 */
export function isUuidV4(value: unknown): value is string {
  return typeof value === 'string' && UUID_V4_RE.test(value);
}
