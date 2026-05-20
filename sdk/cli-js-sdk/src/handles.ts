import type { Hex } from "viem";

import type { DecryptType } from "./types";

type PublicDecryptHandleSet = Readonly<Partial<Record<DecryptType, readonly Hex[]>>>;

export const testnetPublicHandles: PublicDecryptHandleSet = {
  bool: ["0xf6751d547a5c06123575aad93f22f76b7d841c4cacff0000000000aa36a70000"],
  uint8: ["0x6f17228bda73a5e57b94511c5bab2665e6a2870399ff0000000000aa36a70200"],
};

export const resolveCachedHandles = (decryptType: DecryptType, handles?: readonly Hex[]): readonly Hex[] => {
  if (handles?.length) return handles;
  const defaultHandles = testnetPublicHandles[decryptType];
  if (!defaultHandles) {
    throw new Error(
      `No built-in ${decryptType} cached testnet handle is available for @fhevm/sdk alpha handles. Pass one or more --handle values.`,
    );
  }
  return defaultHandles;
};
