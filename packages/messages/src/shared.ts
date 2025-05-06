import { z } from 'zod'
import { v7 as uuid } from 'uuid'

export const operationNames = [
  'FheAdd',
  'FheSub',
  'FheMul',
  'FheDiv',
  'FheRem',
  'FheBitAnd',
  'FheBitOr',
  'FheBitXor',
  'FheShl',
  'FheShr',
  'FheRotl',
  'FheRotr',
  'FheEq',
  'FheEqBytes',
  'FheNe',
  'FheNeBytes',
  'FheGe',
  'FheGt',
  'FheLe',
  'FheLt',
  'FheMin',
  'FheMax',
  'FheNeg',
  'FheNot',
  'VerifyCiphertext',
  'Cast',
  'TrivialEncrypt',
  'TrivialEncryptBytes',
  'FheIfThenElse',
  'FheRand',
  'FheRandBounded',
] as const

export const operationEnum = z.enum(operationNames)

export type operationName = z.infer<typeof operationEnum>

export const MS_PREFIXES = ['back', 'orch', 'relayer', 'web3'] as const
export type MSPrefix = (typeof MS_PREFIXES)[number]

export const meta = z.record(z.string(), z.union([z.string(), z.number()])).and(
  z.object({
    correlationId: z.string().uuid(),
  }),
)

export type Meta = z.infer<typeof meta>

export const chainId = z.union(
  [
    z.string().regex(/^0x[\da-f]+$/i),
    z
      .string()
      .regex(/^[\d]+$/i)
      .refine(v => parseInt(v, 10) > 0),
    z.number().int().positive(),
  ],
  { message: 'Chain ID should be a string, an integer or a hex string' },
)

export const web3Address = z
  .string()
  .regex(/^0x[\da-f]{40}$/i, 'Address should be an hex string of length 42')

export const ctHandleContractPairs = z.object({
  ctHandle: z.string(),
  contractAddress: z.string(),
})

export const requestValidity = z.object({
  startTimestamp: z.string(),
  durationDays: z.string(),
})

export const requestId = z.string().uuid()
export function generateRequestId() {
  return uuid()
}

export function metaFactory<
  Events extends {
    type: string
    payload: object
    meta: Meta
  },
  Prefix extends string = MSPrefix,
>(prefix: Prefix) {
  return function <
    Key extends string,
    Event extends {
      type: `${Prefix}:${Key}`
      payload: object
      meta: Meta
    } = Extract<Events, { type: `${Prefix}:${Key}` }>,
  >(key: Key) {
    return function(payload: Event['payload'], meta: Event['meta']) {
      return {
        type: `${prefix}:${key}`,
        payload,
        meta,
      } as Event
    }
  }
}
