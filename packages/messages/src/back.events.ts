import { z } from 'zod'
import { chainId, meta, Meta, web3Address } from './shared.js'

type EventTypes = 'dapp:stats-requested' | 'dapp:stats-available'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `back:${key}` as `back:${Key}`
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
  'dapp:stats-requested': genSchema('dapp:stats-requested', {}),
  'dapp:stats-available': genSchema('dapp:stats-available', {
    name: z.string(),
    timestamp: z.string().datetime(),
    externalRef: z.string(),
  }),
} as const
type EventMap = typeof eventMap

export const schema = z
  .discriminatedUnion('type', [
    eventMap['dapp:stats-requested'],
    eventMap['dapp:stats-available'],
  ])
  .and(
    z.object({
      $meta: meta,
    }),
  )
export type BackEvent = z.infer<typeof schema>

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<K extends keyof EventMap>(type: K) {
  return function (payload: z.infer<EventMap[K]>['payload'], $meta: Meta) {
    return {
      type: `back:${type}`,
      payload,
      $meta,
    } as BackEvent
  }
}

export const dappStatsRequested = factory('dapp:stats-requested')
export const dappStatsAvailable = factory('dapp:stats-available')

export function isBackEvent(data: unknown): data is BackEvent {
  return schema.safeParse(data).success
}
