import type { Hex } from "viem";

const randomBytes = (length: number): Uint8Array =>
  crypto.getRandomValues(new Uint8Array(length));

export const randomAddress = (): Hex =>
  `0x${Array.from(randomBytes(20), (byte) => byte.toString(16).padStart(2, "0")).join("")}`;

export const randomUint8 = (): number => randomBytes(1)[0] ?? 0;

export const randomUint16 = (): number => {
  const bytes = randomBytes(2);
  return ((bytes[0] ?? 0) << 8) + (bytes[1] ?? 0);
};

export const randomUint32 = (): number => {
  const bytes = randomBytes(4);
  return (
    (bytes[0] ?? 0) * 2 ** 24 +
    ((bytes[1] ?? 0) << 16) +
    ((bytes[2] ?? 0) << 8) +
    (bytes[3] ?? 0)
  );
};

const randomBigUint = (bytesLength: number): bigint => {
  const hex = Array.from(randomBytes(bytesLength), (byte) =>
    byte.toString(16).padStart(2, "0"),
  ).join("");
  return BigInt(`0x${hex}`);
};

export const randomUint64 = (): bigint => randomBigUint(8);
export const randomUint128 = (): bigint => randomBigUint(16);
export const randomUint256 = (): bigint => randomBigUint(32);
