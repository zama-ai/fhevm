import { z } from 'zod'
import { chainId, meta, metaFactory, requestId, web3Address } from './shared.js'

type EventTypes =
  | 'dapp:created'
  | 'dapp:validation:requested'
  | 'dapp:validation:confirmed'
  | 'dapp:validation:failed'
  | 'address:validation:requested'
  | 'address:validation:confirmed'
  | 'address:validation:failed'
  | 'dapp:stats-requested'
  | 'dapp:stats-available'
  | 'httpz:input-proof:requested'
  | 'httpz:input-proof:completed'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `back:${key}` as `back:${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      requestId,
      ...payload,
    }),
  })
}

const schemas = [
  genSchema('dapp:created', { dAppId: z.string() }),
  genSchema('dapp:validation:requested', {
    dAppId: z.string(),
    chainId: chainId,
    address: web3Address,
  }),
  genSchema('dapp:validation:confirmed', {
    dAppId: z.string(),
    owner: web3Address.optional(),
  }),
  genSchema('dapp:validation:failed', {
    dAppId: z.string(),
    reason: z.string(),
  }),
  genSchema('address:validation:requested', {
    chainId: chainId,
    address: web3Address,
  }),
  genSchema('address:validation:confirmed', {
    chainId: chainId,
    address: web3Address,
    owner: web3Address.optional(),
  }),
  genSchema('address:validation:failed', {
    chainId: chainId,
    address: web3Address,
    reason: z.string(),
  }),
  genSchema('dapp:stats-requested', {
    dAppId: z.string(),
    chainId: chainId,
    address: web3Address,
  }),
  // Note: in case we detectet an event on the blockchain, we cannot
  // retrieve the dAppId from the event.
  genSchema('dapp:stats-available', {
    chainId: chainId,
    address: web3Address,
    name: z.string(),
    timestamp: z.string().datetime(),
    externalRef: z.string(),
  }),
  genSchema('httpz:input-proof:requested', {
    contractChainId: chainId,
    contractAddress: web3Address,
    userAddress: web3Address,
    ciphertextWithZkpok: z.string(),
  }),
  genSchema('httpz:input-proof:completed', {
    handles: z.array(z.string()),
    signatures: z.array(z.string()),
  }),
] as const

export const schema = z.discriminatedUnion('type', [...schemas]).and(
  z.object({
    meta: meta,
  }),
)
export type BackEvent = z.infer<typeof schema>

const factory = metaFactory<BackEvent>('back')

export const dappCreated = factory('dapp:created')
export const dappValidationRequested = factory('dapp:validation:requested')
export const dappValidationConfirmed = factory('dapp:validation:confirmed')
export const dappValidationFailed = factory('dapp:validation:failed')
export const addressValidationRequested = factory(
  'address:validation:requested',
)
export const addressValidationConfirmed = factory(
  'address:validation:confirmed',
)
export const addressValidationFailed = factory('address:validation:failed')
export const dappStatsRequested = factory('dapp:stats-requested')
export const dappStatsAvailable = factory('dapp:stats-available')
export const httpzInputProofRequested = factory('httpz:input-proof:requested')
export const httpzInputProofCompleted = factory('httpz:input-proof:completed')

export function isBackEvent(data: unknown): data is BackEvent {
  return schema.safeParse(data).success
}
