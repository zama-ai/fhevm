import { z, ZodType, type ZodBranded, type ZodTypeAny } from 'zod'
import { fromZodError } from './app-error.js'

type TPlain<TSchemaBranded> =
  TSchemaBranded extends ZodBranded<infer TSchema, infer _TBrand> // eslint-disable-line @typescript-eslint/no-unused-vars
    ? z.infer<TSchema>
    : TSchemaBranded extends ZodType
      ? z.infer<TSchemaBranded>
      : never
type ZodUnbranded<T extends ZodTypeAny = ZodTypeAny> =
  T extends ZodBranded<any, any> ? never : T

export function ValueObject<TType extends string, TSchema extends ZodUnbranded>(
  type: TType,
  unbranded: TSchema,
) {
  const schema = unbranded.brand(type)
  type UnbrandedValue = z.infer<TSchema>
  type Value = z.infer<typeof schema>

  return class ValueObject {
    static readonly schema = schema
    static readonly type = type
    #value: Value

    protected constructor(value: Value) {
      this.#value = value
    }

    static from(value: UnbrandedValue) {
      const parsed = schema.safeParse(value)
      if (!parsed.success) {
        throw fromZodError(parsed.error)
      }
      return new this(parsed.data)
    }

    toPlainValue(): TPlain<TSchema> {
      return this.#value
    }

    toString() {
      return this.toPlainValue().toString()
    }

    valueOf() {
      return this.toPlainValue().valueOf()
    }

    get value(): Value {
      return this.#value
    }

    // Custom equality check
    equals(other: ValueObject): boolean {
      return this.value === other.value
    }
  }
}
