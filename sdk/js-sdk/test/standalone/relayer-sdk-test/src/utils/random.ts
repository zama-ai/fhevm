import { type Hex, getAddress } from "viem"

/** Returns a random hex string of length chars. */
const randomHexString = (length: number): string => {
    return Array.from({ length }, () =>
        Math.round(Math.random() * 0xf).toString(16)
    ).join("")
}

/** Returns a random BigInt of n bits in length. */
export const randomBigIntN = (n: number): bigint => {
    return BigInt.asUintN(n, BigInt(`0x${randomHexString(Math.ceil(n / 4))}`))
}

/** Returns a uniformly distributed 8-bit unsigned integer. */
export const randomUint8 = (): number => Number(randomBigIntN(8))

/** Returns a uniformly distributed 16-bit unsigned integer. */
export const randomUint16 = (): number => Number(randomBigIntN(16))

/** Returns a uniformly distributed 32-bit unsigned integer. */
export const randomUint32 = (): number => Number(randomBigIntN(32))

/** Returns a uniformly distributed 64-bit unsigned integer. */
export const randomUint64 = (): number => Number(randomBigIntN(64))

/** Returns a uniformly distributed 128-bit unsigned integer. */
export const randomUint128 = (): bigint => randomBigIntN(128)

/** Returns a uniformly distributed 256-bit unsigned integer. */
export const randomUint256 = (): bigint => randomBigIntN(256)

/** Returns a uniformly distributed 160-bit unsigned integer. */
export const randomAddress = (): Hex => getAddress(`0x${randomHexString(40)}`)
