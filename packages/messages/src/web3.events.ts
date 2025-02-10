import { z } from 'zod'
import { chainId, Meta, meta, web3Address } from './shared.js'

type EventTypes = 'fhe-event:requested' | 'fhe-event:detected'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `web3:${key}` as `web3:${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      chainId,
      address: web3Address,
      ...payload,
    }),
  })
}

const eventMap = {
  'fhe-event:requested': genSchema('fhe-event:requested', {}),
  'fhe-event:detected': genSchema('fhe-event:detected', {
    id: z.string(),
    name: z.string(),
    timestamp: z.string().datetime(),
  }),
} as const
type EventMap = typeof eventMap

export const schema = z
  .discriminatedUnion('type', [
    eventMap['fhe-event:requested'],
    eventMap['fhe-event:detected'],
  ])
  .and(
    z.object({
      meta: meta,
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
  return function (payload: z.infer<EventMap[K]>['payload'], meta: Meta) {
    return {
      type: `web3:${type}`,
      payload,
      meta,
    } as Web3Event
  }
}

export const fheRequested = factory('fhe-event:requested')
export const fheDetected = factory('fhe-event:detected')

export function isWeb3Event(data: unknown): data is Web3Event {
  return schema.safeParse(data).success
}
