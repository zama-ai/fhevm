import { z } from 'zod'
import { chainId, meta, metaFactory, requestId, web3Address } from './shared.js'

type EventTypes =
  | 'contract:validation:requested'
  | 'contract:validation:success'
  | 'contract:validation:failure'
  | 'fhe-event:requested'
  | 'fhe-event:detected'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `web3:${key}` as `web3:${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      requestId,
      chainId,
      address: web3Address,
      ...payload,
    }),
  })
}

const schemas = [
  genSchema('contract:validation:requested', {}),
  genSchema('contract:validation:success', {
    owner: web3Address.optional(),
  }),
  genSchema('contract:validation:failure', {
    reason: z.string().optional(),
  }),
  genSchema('fhe-event:requested', {}),
  genSchema('fhe-event:detected', {
    id: z.string(),
    name: z.string(),
    timestamp: z.string().datetime(),
  }),
] as const

export const schema = z.discriminatedUnion('type', [...schemas]).and(
  z.object({
    meta: meta,
  }),
)
export type Web3Event = z.infer<typeof schema>

const factory = metaFactory<EventTypes, Web3Event>('web3')

export const contractValidationRequested = factory(
  'contract:validation:requested',
)
export const contractValidationSuccess = factory('contract:validation:success')
export const contractValidationFailure = factory('contract:validation:failure')
export const fheRequested = factory('fhe-event:requested')
export const fheDetected = factory('fhe-event:detected')

export function isWeb3Event(data: unknown): data is Web3Event {
  return schema.safeParse(data).success
}
