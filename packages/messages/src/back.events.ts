import { z } from 'zod'
import { chainId, meta, Meta, web3Address } from './shared.js'

type EventTypes =
  | 'dapp:created'
  | 'dapp:confirmed'
  | 'dapp:failed'
  | 'dapp:stats-requested'
  | 'dapp:stats-available'

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

const schemas = [
  genSchema('dapp:created', { dAppId: z.string() }),
  genSchema('dapp:confirmed', { dAppId: z.string() }),
  genSchema('dapp:failed', {
    dAppId: z.string(),
    reason: z.string(),
  }),
  genSchema('dapp:stats-requested', {}),
  genSchema('dapp:stats-available', {
    name: z.string(),
    timestamp: z.string().datetime(),
    externalRef: z.string(),
  }),
] as const

export const schema = z.discriminatedUnion('type', [...schemas]).and(
  z.object({
    meta: meta,
  }),
)
export type BackEvent = z.infer<typeof schema>

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<
  K extends EventTypes,
  Event extends { type: `back:${K}`; payload: object; meta: Meta } = Extract<
    BackEvent,
    { type: `back:${K}` }
  >,
>(type: K) {
  return function (payload: Event['payload'], meta: Meta) {
    return {
      type: `back:${type}`,
      payload,
      meta,
    } as Event
  }
}

export const dappCreated = factory('dapp:created')
export const dappConfirmed = factory('dapp:confirmed')
export const dappFailed = factory('dapp:failed')
export const dappStatsRequested = factory('dapp:stats-requested')
export const dappStatsAvailable = factory('dapp:stats-available')

export function isBackEvent(data: unknown): data is BackEvent {
  return schema.safeParse(data).success
}
