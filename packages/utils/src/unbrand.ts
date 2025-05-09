import { BRAND } from 'zod'

type UnionToIntersection<Union> =
  // `extends unknown` is always going to be the case and is used to convert the
  // `Union` into a [distributive conditional
  // type](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-2-8.html#distributive-conditional-types).
  (
    Union extends unknown
      ? // The union type is used as the only argument to a function since the union
        // of function arguments is an intersection.
        (distributedUnion: Union) => void
      : // This won't happen.
        never
  ) extends // Infer the `Intersection` type since TypeScript represents the positional
  // arguments of unions of functions as an intersection of the union.
  (mergedIntersection: infer Intersection) => void
    ? // The `& Union` is to allow indexing by the resulting type
      Intersection & Union
    : never

type Primitive = null | undefined | string | number | boolean | symbol | bigint

type IterateOnTuple<T extends [...unknown[]]> = T extends [
  infer Head,
  ...infer Tail,
]
  ? [Unbrand<Head>, ...IterateOnTuple<Tail>]
  : []

type RemoveBrand<T> =
  T extends BRAND<infer Brand>
    ? T extends (
        | BRAND<Brand>
        | UnionToIntersection<{ [K in Brand]: BRAND<K> }[Brand]>
      ) &
        infer X
      ? RemoveBrand<X>
      : never
    : T

/**
 * Recursively removes the brand from T.
 *
 * @see https://github.com/colinhacks/zod/discussions/1994#discussioncomment-6068940
 */
export type Unbrand<T> = T extends Primitive
  ? RemoveBrand<T>
  : T extends Promise<infer E>
    ? Promise<Unbrand<E>>
    : T extends [unknown, ...unknown[]]
      ? IterateOnTuple<RemoveBrand<T>>
      : T extends Date
        ? RemoveBrand<T>
        : T extends (infer E)[]
          ? Unbrand<E>[]
          : T extends Set<infer E>
            ? Set<Unbrand<E>>
            : T extends Map<infer E, infer F>
              ? Map<Unbrand<E>, Unbrand<F>>
              : { [K in keyof T]: Unbrand<T[K]> }
