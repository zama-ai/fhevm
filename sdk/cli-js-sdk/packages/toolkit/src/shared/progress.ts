import type { DecryptedValue, EncryptValue, FheTestHandle } from "../types";
import { serializeValue } from "../values";

export type ProgressReporter = (message: string) => void;

export const describeValue = (value: EncryptValue): string => {
  const serialized = serializeValue(value);
  return `${serialized.type}=${serialized.value}`;
};

export const describeValues = (values: readonly EncryptValue[]): string =>
  values.map(describeValue).join(", ");

export const describeDecryptedValues = (
  values: readonly DecryptedValue[],
): string => values.map((value) => `${value.type}=${value.value}`).join(", ");

export const describeHandle = (handle: FheTestHandle): string =>
  `${handle.handle}${handle.clearText ? ` (clearText=${handle.clearText})` : ""}`;
