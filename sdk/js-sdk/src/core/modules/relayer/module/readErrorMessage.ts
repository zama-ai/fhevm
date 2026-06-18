import { isRecordStringProperty } from '../../../base/string.js';

////////////////////////////////////////////////////////////////////////////////
// readErrorMessage
////////////////////////////////////////////////////////////////////////////////

/**
 * Best-effort reader that surfaces the message a relayer or an intermediary
 * edge proxy (Cloudflare/Kong) actually sent in a non-2xx JSON error body.
 *
 * Two body shapes are recognized:
 * - relayer:           `{ "error": { "message": "...", "label": "..." } }`
 * - Cloudflare / Kong: `{ "message": "...", "label": "..." }` (flat)
 *
 * The body is read at most once and only surfaced when it parses as JSON. A
 * non-JSON body (a bare `"Forbidden"`/`"Rate Limit"` string, an HTML error
 * page, ...) intentionally surfaces nothing, so the previous behavior for those
 * responses is preserved and no noise leaks into error messages.
 *
 * This function never throws.
 *
 * @param response - The non-2xx fetch {@link Response} to inspect.
 * @returns The surfaced `message` and/or `label`, or an empty object when the
 *          body is missing, unreadable, not JSON, or carries neither field.
 */
export async function readErrorMessage(
  response: Response,
): Promise<{ message?: string | undefined; label?: string | undefined }> {
  let text: string;
  try {
    text = await response.text();
  } catch {
    return {};
  }

  if (text.length === 0) {
    return {};
  }

  try {
    const json: unknown = JSON.parse(text);
    const err: unknown = typeof json === 'object' && json !== null && 'error' in json ? json.error : json;
    const message = isRecordStringProperty(err, 'message') ? err.message : undefined;
    const label = isRecordStringProperty(err, 'label') ? err.label : undefined;
    if (message !== undefined || label !== undefined) {
      return { message, label };
    }
  } catch {
    // Body wasn't JSON — surface nothing and leave the non-JSON behavior unchanged.
  }

  return {};
}
