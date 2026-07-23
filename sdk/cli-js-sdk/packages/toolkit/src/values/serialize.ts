import type { EncryptValue } from "../types";

export const serializeValue = (
  value: EncryptValue,
): Readonly<{ type: string; value: string }> => ({
  type: value.type,
  value:
    typeof value.value === "bigint"
      ? value.value.toString()
      : String(value.value),
});
