import { z } from 'zod'
import { v7 as uuid } from 'uuid'

export const MS_PREFIXES = ['back', 'orch', 'relayer', 'web3'] as const
export type MSPrifix = (typeof MS_PREFIXES)[number]

export const meta = z.record(z.string(), z.union([z.string(), z.number()])).and(
  z.object({
    correlationId: z.string().uuid(),
  }),
)

export type Meta = z.infer<typeof meta>

export const chainId = z.string().refine(
  v => {
    const n = Number(v)
    return !isNaN(n) && n > 0
  },
  { message: 'Invalid Chain Id' },
)

export const web3Address = z
  .string()
  .length(42, 'blockchain address must be exactly 42 charaxters long')
  .startsWith('0x', 'sepolia address must start with 0x')

export const requestId = z.string().uuid()
export function generateRequestId() {
  return uuid()
}

export function metaFactory<
  Key extends string,
  Events extends {
    type: `${Prefix}:${Key}`
    payload: object
    meta: Meta
  },
  Prefix extends MSPrifix = MSPrifix,
  Event extends {
    type: `${Prefix}:${Key}`
    payload: object
    meta: Meta
  } = Extract<Events, { type: `${Prefix}:${Key}` }>,
>(prefix: Prefix) {
  return function (key: Key) {
    return function (payload: Event['payload'], meta: Event['meta']) {
      return {
        type: `${prefix}:${key}`,
        payload,
        meta,
      } as Event
    }
  }
}
