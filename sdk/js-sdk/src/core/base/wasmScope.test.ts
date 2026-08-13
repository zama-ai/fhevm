import { describe, expect, it } from 'vitest';
import { WasmScope, type Freeable } from './wasmScope.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/wasmScope.test.ts
////////////////////////////////////////////////////////////////////////////////

/**
 * A fake wasm-bindgen-like object: records the order in which it is freed and
 * how many times, and optionally throws from `free()` to exercise the
 * best-effort cleanup path.
 */
function freeable(
  id: string,
  order: string[],
  opts?: { readonly throws?: boolean },
): Freeable & { readonly calls: () => number } {
  let count = 0;
  return {
    free() {
      count++;
      order.push(id);
      if (opts?.throws) {
        throw new Error(`free failed: ${id}`);
      }
    },
    calls: () => count,
  };
}

describe('WasmScope', () => {
  it('track() returns the same reference it was given', () => {
    const scope = new WasmScope();
    const a = freeable('a', []);
    expect(scope.track(a)).toBe(a);
  });

  it('free() frees every tracked item exactly once, in reverse (LIFO) order', () => {
    const scope = new WasmScope();
    const order: string[] = [];
    const a = freeable('a', order);
    const b = freeable('b', order);
    const c = freeable('c', order);
    scope.track(a);
    scope.track(b);
    scope.track(c);

    scope.free();

    expect(order).toEqual(['c', 'b', 'a']);
    expect([a.calls(), b.calls(), c.calls()]).toEqual([1, 1, 1]);
  });

  it('free() on an empty scope is a no-op', () => {
    const scope = new WasmScope();
    expect(() => scope.free()).not.toThrow();
  });

  it('free() is best-effort: a throwing free() does not strand the other items', () => {
    const scope = new WasmScope();
    const order: string[] = [];
    const a = freeable('a', order);
    const b = freeable('b', order, { throws: true });
    const c = freeable('c', order);
    scope.track(a);
    scope.track(b);
    scope.track(c);

    expect(() => scope.free()).not.toThrow();

    // All three were attempted in LIFO order, and `a` (below the thrower) was still freed.
    expect(order).toEqual(['c', 'b', 'a']);
    expect(a.calls()).toBe(1);
  });

  it('free() drains the scope: a second free() does not re-free items', () => {
    const scope = new WasmScope();
    const order: string[] = [];
    const a = freeable('a', order);
    scope.track(a);

    scope.free();
    scope.free();

    expect(a.calls()).toBe(1);
    expect(order).toEqual(['a']);
  });

  it('is reusable: items tracked after a free() are freed on the next free()', () => {
    const scope = new WasmScope();
    const order: string[] = [];
    const a = freeable('a', order);
    scope.track(a);
    scope.free();

    const b = freeable('b', order);
    scope.track(b);
    scope.free();

    expect(order).toEqual(['a', 'b']);
    expect(a.calls()).toBe(1); // not re-freed
    expect(b.calls()).toBe(1);
  });
});
