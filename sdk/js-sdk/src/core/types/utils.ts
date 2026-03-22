/**
 * Utility type that flattens intersection types for better IDE display.
 */
export type Prettify<T> = {
  [K in keyof T]: T[K];
} & {};

export type ValidationResult<T, E extends Error = Error> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly error: E };

export type PartialWithUndefined<T> = {
  [P in keyof T]?: T[P] | undefined;
};

declare const symbol: unique symbol;
export type Branded<T, U> = T & { [symbol]: U };

export interface Branding<BrandT> {
  __brand: BrandT;
}

export interface Flavoring<FlavorT> {
  __flavor?: FlavorT;
}

export type Flavor<T, FlavorT> = T & Flavoring<FlavorT>;

export type NonEmptyExtract<T, U> =
  Extract<T, U> extends never
    ? { error: "Extract produced never - no matching types found" }
    : Extract<T, U>;

export type StrictEquals<A, B> = [A] extends [B]
  ? [B] extends [A]
    ? true
    : false
  : false;

export type NonEmptyReadonlyArray<T> = readonly [T, ...T[]];
