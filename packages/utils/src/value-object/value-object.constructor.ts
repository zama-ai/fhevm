import { ZodBranded, ZodFirstPartySchemaTypes, ZodTypeAny } from 'zod'

import { typeUtil, util } from './util'

export class ValueObject<
  Type extends string,
  TSchema extends ZodBranded<ZodTypeAny, string> = ZodBranded<
    ZodTypeAny,
    string
  >,
  T extends typeUtil.TPlain<TSchema> = typeUtil.TPlain<TSchema>,
  TInput extends typeUtil.Input<TSchema> = typeUtil.Input<TSchema>,
> implements typeUtil.ValueObjectCtor<Type, TSchema, T, TInput>
{
  static schema: InstanceType<typeof ZodBranded>
  static type: string

  public readonly type: Type
  public readonly schema: TSchema
  public readonly value: T

  toString() {
    return this.toPlainValue().toString()
  }

  valueOf() {
    return this.toPlainValue().valueOf()
  }

  toJSON() {
    const plain = this.toPlainValue()
    if (util.isPrimitive(plain)) return plain
    return JSON.stringify(plain)
  }

  validate(value: T): T {
    // TODO: handle exception with either monad
    return this.schema.parse(value)
  }

  toPlainValue(value: TInput = this.value): T {
    return ValueObject.toPlainValue<TSchema>(value)
  }

  equals(other: TInput): boolean {
    const Ctor = this.constructor as typeof ValueObject
    return (
      util.flyweight.run(Ctor, this.type, Ctor.toPlainValue(other)) === this
    )
  }

  with(value: typeUtil.PartialValue<T>): this {
    const Ctor = this.constructor as typeUtil.Ctor<this>
    if (util.isPrimitive(value))
      return util.flyweight.run(Ctor, this.type, value)

    if (typeof value === 'object' && value !== null) {
      const newValue = Object.assign({}, this.value, value)
      return util.flyweight.run(Ctor, this.type, newValue)
    }

    return util.flyweight.run(Ctor, this.type, value)
  }

  // Note: I should use a protected constructor, but that will break the next line
  // TODO: Look for a solution without the current implemention
  public constructor(value: TInput) {
    const Ctor = this.constructor as typeof ValueObject<
      Type,
      TSchema,
      T,
      TInput
    >

    util.defineImmutable(this, 'schema', Ctor.schema)
    util.defineImmutable(this, 'type', Ctor.type)
    util.defineImmutable(this, 'value', Object.freeze(value), true)
    util.defineImmutable(this, util.IS_VALUE_OBJECT, true)

    const unwrapped = Ctor.toPlainValue(value)
    const instance = util.flyweight.run(Ctor, Ctor.type as Type, unwrapped)
    /* Instance will be null once, if the flyweight cache is empty.
     * In this case we don't return the flyweight instance, but the
     * newly (implicitly) constructed instance */
    if (instance) {
      util.validateSync(instance, unwrapped, this.constructor.name)
      util.copyPrototypeTo(
        this.constructor.prototype,
        instance.constructor.prototype,
      )

      return instance
    }
  }

  static async createAsync(
    value: typeUtil.Input<InstanceType<typeof ZodBranded>>,
  ) {
    const unwrapped = this.toPlainValue(value)
    await this.schema.parseAsync(unwrapped)
    return util.flyweight.run(this, this.type, unwrapped)
  }

  static toPlainValue<
    Schema extends ZodFirstPartySchemaTypes,
    T extends typeUtil.Input<Schema> = typeUtil.Input<Schema>,
  >(value: T): typeUtil.TPlain<Schema> {
    if (util.isValueObject(value)) return value.toPlainValue()

    if (Array.isArray(value)) {
      return value.map((item: typeUtil.Input<any>) =>
        ValueObject.toPlainValue(item),
      ) as typeUtil.TPlain<Schema>
    }
    if (typeof value === 'object' && value !== null) {
      return Object.entries(value).reduce(
        (acc, [key, value]) => ({
          ...acc,
          [key]: ValueObject.toPlainValue(value as typeUtil.Input<any>),
        }),
        {} as typeUtil.TPlain<Schema>,
      )
    }
    return value as typeUtil.TPlain<Schema>
  }

  static _disableFlyweight() {
    util.flyweight.run = util.flyweightDisabled
  }
  static _enableFlyweight() {
    util.flyweight.run = util.flyweightEnabled
  }
}
