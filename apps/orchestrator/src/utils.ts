export type ExhaustiveTuple<
  TUnion,
  TTuple extends readonly any[],
  TOriginal extends readonly any[] = TTuple,
> = TTuple extends readonly [infer F, ...infer R]
  ? Exclude<TUnion, F> extends never
    ? TOriginal
    : ExhaustiveTuple<Exclude<TUnion, F>, R, TOriginal>
  : never;
