import { z } from 'zod'

type EventTypes = 'fhe-event:requested' | 'fhe-event:detected'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `web3:${key}` as `web3:${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      chainId: z.string().refine(
        v => {
          const n = Number(v)
          return !isNaN(n) && n > 0
        },
        { message: 'Invalid Chain Id' },
      ),
      address: z
        .string()
        .length(42, 'blockchain address must be exactly 42 charaxters long')
        .startsWith('0x', 'sepolia address must start with 0x'),
      ...payload,
    }),
  })
}

const eventMap = {
  'fhe-event:requested': genSchema('fhe-event:requested', {}),
  'fhe-event:detected': genSchema('fhe-event:detected', {
    name: z.string(),
    timestamp: z.date(),
  }),
} as const
type EventMap = typeof eventMap

const schema = z
  .discriminatedUnion('type', [
    eventMap['fhe-event:requested'],
    eventMap['fhe-event:detected'],
  ])
  .and(
    z.object({
      $meta: z.record(z.string(), z.union([z.string(), z.number()])).optional(),
    }),
  )
export type Web3Event = z.infer<typeof schema>

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<K extends keyof EventMap>(type: K) {
  return function (
    payload: z.infer<EventMap[K]>['payload'],
    $meta?: Record<string, string | number>,
  ) {
    return {
      type: `web3:${type}`,
      payload,
      $meta,
    } as Web3Event
  }
}

export const fheRequested = factory('fhe-event:requested')
export const fheDetected = factory('fhe-event:detected')

export function isWeb3Event(data: unknown): data is Web3Event {
  return schema.safeParse(data).success
}
