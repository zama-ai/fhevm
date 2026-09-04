////////////////////////////////////////////////////////////////////////////////
// WasmScope
////////////////////////////////////////////////////////////////////////////////

export interface Freeable {
  free(): void;
}

/**
 * Collects wasm-bindgen objects created within a scope and frees them in
 * reverse (LIFO) order. WASM memory is NOT reclaimed by the JS garbage
 * collector, so every object obtained from a wasm module must be freed
 * explicitly.
 *
 * Only `track()` objects this scope OWNS. Never track an object that is:
 * - moved (consumed) into another wasm call — e.g. values passed by-value to a
 *   wasm function that takes ownership, or
 * - a borrowed native handle of a caller-owned object.
 *
 * ```ts
 * const scope = new WasmScope();
 * try {
 *   const pk = scope.track(kmsLib.ml_kem_pke_get_pk(sk));
 *   // ...
 * } finally {
 *   scope.free();
 * }
 * ```
 */
export class WasmScope {
  readonly #items: Freeable[] = [];

  public track<T extends Freeable>(item: T): T {
    this.#items.push(item);
    return item;
  }

  public free(): void {
    // LIFO, best-effort: a failed free() must not strand the remaining objects.
    while (this.#items.length > 0) {
      const item = this.#items.pop();
      if (item !== undefined) {
        try {
          item.free();
        } catch {
          // best-effort cleanup
        }
      }
    }
  }
}
