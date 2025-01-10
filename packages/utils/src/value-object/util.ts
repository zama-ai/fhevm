import {
  z,
  ZodArray,
  ZodBranded,
  ZodFirstPartySchemaTypes,
  ZodFirstPartyTypeKind,
  ZodObject,
  ZodRecord,
  ZodTuple,
  ZodType,
  ZodTypeAny,
} from 'zod'

// eslint-disable-next-line  @typescript-eslint/no-namespace
export namespace util {
  export const sortObj = <T>(obj: T): T =>
    obj === null || typeof obj !== 'object'
      ? obj
      : Array.isArray(obj)
        ? obj.map(sortObj)
        : obj instanceof Date
          ? obj
          : Object.assign(
              {},
              ...Object.entries(obj)
                .sort(([keyA], [keyB]) => keyA.localeCompare(keyB))
                .map(([k, v]) => ({ [k]: sortObj(v) })),
            )

  export const objectKeys: ObjectConstructor['keys'] =
    typeof Object.keys === 'function'
      ? (obj: any) => Object.keys(obj)
      : (object: any) => {
          const keys = []
          for (const key in object) {
            if (Object.prototype.hasOwnProperty.call(object, key)) {
              keys.push(key)
            }
          }
          return keys
        }

  export const defineImmutable = <
    TObject,
    TValue,
    TKey extends string | symbol,
  >(
    object: TObject,
    property: TKey,
    value: TValue,
    enumerable = false,
  ): TObject & { [key in TKey]: TValue } =>
    Object.defineProperty(object, property, {
      value: value,
      writable: false,
      enumerable,
    }) as TObject & { [key in TKey]: TValue }

  export const isPrimitive = (value: unknown): value is typeUtil.Primitive => {
    const type = typeof value
    return (
      value === null ||
      ['string', 'number', 'boolean', 'bigint', 'symbol', 'undefined'].includes(
        type,
      )
    )
  }

  export const extendClass = <
    Ctor extends typeUtil.SuperConstructor,
    ExtendCtor extends typeUtil.SuperConstructor,
  >(
    Ctor: Ctor,
    extend?: ExtendCtor,
  ): Ctor & ExtendCtor => {
    if (!extend) return Ctor as Ctor & ExtendCtor
    const names = Object.getOwnPropertyNames(extend.prototype)
    for (const prop of names) {
      if (prop in Ctor.prototype) continue
      const desc = Object.getOwnPropertyDescriptor(extend.prototype, prop)
      if (desc) {
        Object.defineProperty(Ctor.prototype, prop, desc)
      }
    }
    return Ctor as Ctor & ExtendCtor
  }

  const db = Object.create(null)
  export const flyweightEnabled = <T extends typeUtil.Ctor, TValue>(
    Ctor: T,
    type: typeUtil.Instance<T>['type'],
    value: TValue,
  ): typeUtil.Instance<T> => {
    const table = (db[type] = db[type] || Object.create(null))

    const key = JSON.stringify(util.sortObj(value))
    if (typeof table[key] !== 'undefined') return table[key]

    table[key] = null
    table[key] = new Ctor(value)

    return table[key]
  }
  export const flyweightDisabled = <T, TValue>(
    Ctor: typeUtil.Ctor,
    value: TValue,
  ): T => {
    return new Ctor(value, true)
  }

  export const flyweight = {
    run: flyweightEnabled,
  }

  export const brandSchema = <
    Schema extends ZodFirstPartySchemaTypes,
    Type extends string,
  >(
    schema: Schema,
    type: Type,
  ): typeUtil.Branded<Schema, Type> => {
    if (schema._def.typeName === ZodFirstPartyTypeKind.ZodBranded) {
      return schema as unknown as typeUtil.Branded<Schema, Type>
    } else {
      return schema.brand(type) as typeUtil.Branded<Schema, Type>
    }
  }

  export const copyPrototypeTo = (prototype1: object, prototype2: object) => {
    const names = Object.getOwnPropertyNames(prototype1)
    for (const prop of names) {
      if (prop in prototype2) continue
      const desc = Object.getOwnPropertyDescriptor(prototype1, prop)
      desc && Object.defineProperty(prototype2, prop, desc) // eslint-disable-line @typescript-eslint/no-unused-expressions
    }
  }

  export const IS_VALUE_OBJECT = Symbol('IS_VALUE_OBJECT')
  export const isValueObject = (
    value: unknown,
  ): value is typeUtil.ValueObjectCtor<string> => {
    return (
      value !== null &&
      typeof value === 'object' &&
      IS_VALUE_OBJECT in value &&
      value[IS_VALUE_OBJECT] === true
    )
  }

  export const validateSync = (
    instance: typeUtil.ValueObjectCtor<any>,
    value: any,
    name: string,
  ) => {
    // TODO: replace it with an either monad
    try {
      instance['validate'](value)
    } catch (error) {
      if (
        error instanceof Error &&
        (error.message.includes('Async refinement encountered') ||
          error.message.includes('Asynchronous transform encountered'))
      ) {
        error.message = error.message.replace(
          'Use .parseAsync instead.',
          `Use ${name}.createAsync() instead of new ${name}()`,
        )
      }
      throw error
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace typeUtil {
  export type Primitive =
    | string
    | number
    | boolean
    | bigint
    | symbol
    | undefined
    | null

  export type Ctor<T = any, A extends any[] = any[]> = new (...args: A) => T
  export type SuperConstructor = abstract new (...args: any) => object
  export type Instance<T extends Ctor> = T extends Ctor<infer U> ? U : never

  export type Literal<S extends string> = S extends string
    ? string extends S
      ? never
      : S
    : never

  export type Branded<TSchema extends ZodTypeAny, Type extends string> =
    TSchema extends ZodBranded<infer S, typeUtil.Literal<Type>>
      ? ZodBranded<S, typeUtil.Literal<Type>>
      : ZodBranded<TSchema, typeUtil.Literal<Type>>

  export interface ValueObjectCtor<
    Type extends KeyType,
    TSchema extends ZodBranded<ZodTypeAny, string> = ZodBranded<
      ZodTypeAny,
      string
    >,
    T extends TPlain<TSchema> = TPlain<TSchema>,
    TInput extends Input<TSchema> = Input<TSchema>,
  > {
    type: Type
    schema: TSchema
    readonly value: T
    toPlainValue(value: TInput): T
    validate(value: T): T
    equals(other: TInput): boolean
    with(value: PartialValue<T>): ValueObjectCtor<Type, TSchema, T, TInput>
  }

  export type TPlain<TSchemaBranded> =
    TSchemaBranded extends ZodBranded<infer TSchema, infer _TBrand> // eslint-disable-line @typescript-eslint/no-unused-vars
      ? z.infer<TSchema>
      : TSchemaBranded extends ZodType
        ? z.infer<TSchemaBranded>
        : never

  export type PartialValue<T> = T extends typeUtil.Primitive
    ? T
    : T extends Array<infer U>
      ? {
          [key: number]:
            | U
            | ValueObjectCtor<string, ZodBranded<ZodType<U>, string>>
        }
      : T extends Record<infer key, infer U>
        ? Partial<
            Record<
              key,
              U | ValueObjectCtor<string, ZodBranded<ZodType<U>, string>>
            >
          >
        : never

  export type KeyType = string | number | symbol

  type NativeTypes<TSchema extends ZodType> =
    TSchema extends ZodBranded<infer TSchema, infer _> // eslint-disable-line @typescript-eslint/no-unused-vars
      ? z.infer<TSchema>
      : TSchema extends ZodType<any>
        ? z.infer<TSchema>
        : never

  type PureValueObjectType<TSchema extends ZodType> =
    TSchema extends ZodBranded<infer TSchema, infer TType>
      ? ValueObjectCtor<TType, ZodBranded<TSchema, TType>>
      : TSchema extends ZodType<any>
        ? ValueObjectCtor<string, ZodBranded<TSchema, any>>
        : never

  type MixedTypes<TSchema extends ZodType> =
    TSchema extends ZodBranded<infer S, any>
      ? S extends ZodType<infer O>
        ? /* Array or Tuple */
          O extends Array<infer I>
          ? S extends ZodArray<infer s>
            ? Array<Input<s> | I>
            : S extends ZodTuple<infer s>
              ? { [key in keyof s]: Input<s[key]> | z.infer<s[key]> }
              : never
          : /* Record or Object */
            O extends Record<any, any>
            ? S extends ZodRecord<infer key, infer value>
              ? { [k in keyof key]: Input<value> | z.infer<value> }
              : S extends ZodObject<infer s>
                ? { [key in keyof s]: Input<s[key]> | z.infer<s[key]> }
                : never
            : never
        : never
      : TSchema extends ZodType<infer O>
        ? /* Array or Tuple */
          O extends Array<infer I>
          ? TSchema extends ZodArray<infer s>
            ? Array<Input<s> | I>
            : TSchema extends ZodTuple<infer s>
              ? { [key in keyof s]: Input<s[key]> | z.infer<s[key]> }
              : never
          : /* Record or Object */
            O extends Record<any, any>
            ? TSchema extends ZodRecord<infer key, infer value>
              ? { [k in keyof key]: Input<value> | z.infer<value> }
              : TSchema extends ZodObject<infer s>
                ? {
                    [key in keyof s]: Input<s[key]> | z.infer<s[key]>
                  }
                : never
            : never
        : never

  export type Input<TSchema extends ZodType> =
    | NativeTypes<TSchema>
    | PureValueObjectType<TSchema>
    | MixedTypes<TSchema>
}
