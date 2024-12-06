import { ZodBranded, ZodTypeAny } from "zod";

import { typeUtil, util } from "./util";
import { ValueObject as ValueObjectCtor } from "./value-object.constructor";

/**
 * This factory is used to type our instance correctly
 * and set static properties.
 */
const createBaseClass = <
  Type extends string,
  Schema extends ZodBranded<any, any>
>(
  brandedSchema: Schema,
  type: Type
) => {
  class ValueObjectType extends ValueObjectCtor<Type, typeof brandedSchema> {
    static schema: typeof brandedSchema;
    static type: typeof type;

    constructor(value: typeUtil.Input<typeof brandedSchema>) {
      super(value);
    }

    static async createAsync(value: typeUtil.Input<typeof brandedSchema>) {
      return super.createAsync(value);
    }
  }
  /* To secure immutability */
  util.defineImmutable(ValueObjectType, "schema", brandedSchema, true);
  util.defineImmutable(ValueObjectType, "type", type, true);
  util.defineImmutable(ValueObjectType, "name", type);
  return ValueObjectType;
};

export function ValueObject<Type extends string, TSchema extends ZodTypeAny>(
  type: typeUtil.Literal<Type>,
  schema: TSchema
) {
  const brandedSchema = util.brandSchema(schema, type);
  const BaseClass = createBaseClass(brandedSchema, type);
  type T = typeof BaseClass & typeof brandedSchema;

  /* Return a Proxy to allow using the ValueObject as Zod-Type */
  return new Proxy<T>(BaseClass as T, {
    get: (target, prop) => {
      if (!(prop in target) && prop in brandedSchema) {
        // @ts-expect-error ts(2339)
        return brandedSchema[prop];
      }
      // @ts-expect-error ts(2339)
      return target[prop];
    },
  });
}
