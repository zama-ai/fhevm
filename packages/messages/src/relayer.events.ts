import { z } from 'zod'
import { chainId, Meta, meta, requestId, web3Address } from './shared.js'

const hexEncoded = z.string().startsWith('0x').and(z.custom<`0x${string}`>())

type EventTypes =
  | 'public-decryption:authorization-request'
  | 'public-decryption:authorization-response'
  | 'private-decryption:operation-request'
  | 'private-decryption:operation-response'
  | 'input-registration:input-registration-request'
  | 'input-registration:input-registration-response'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `relayer:${key}` as `relayer:${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      requestId,
      ...payload,
    }),
  })
}

const schemas = [
  genSchema('public-decryption:authorization-request', {
    callerAddress: web3Address,
  }),
  genSchema('public-decryption:authorization-response', {
    authorized: z.boolean(),
  }),
  genSchema('private-decryption:operation-request', {
    ctHandles: z.array(hexEncoded),
    publicKey: hexEncoded,
    chainId,
  }),
  genSchema('private-decryption:operation-response', {
    ctValues: z.array(hexEncoded),
    signatures: z.array(hexEncoded),
  }),
  genSchema('input-registration:input-registration-request', {
    contractChainId: chainId,
    contractAddress: web3Address,
    userAddress: web3Address,
    ciphertextWithZkpok: z.string(),
  }),
  genSchema('input-registration:input-registration-response', {
    handles: z.array(z.string()),
    signatures: z.array(z.string()),
  }),
] as const

export const schema = z
  .discriminatedUnion('type', [...schemas])
  .and(z.object({ meta }))
export type RelayerEvent = z.infer<typeof schema>

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<
  K extends EventTypes,
  Event extends { type: `relayer:${K}`; payload: object; meta: Meta } = Extract<
    RelayerEvent,
    { type: `relayer:${K}` }
  >,
>(type: K) {
  return function (payload: Event['payload'], meta: Meta) {
    return {
      type: `relayer:${type}`,
      payload,
      meta,
    } as Event
  }
}

export const publicDecryptionAuthorizationRequest = factory(
  'public-decryption:authorization-request',
)
export const publicDecryptionAuthorizationResponse = factory(
  'public-decryption:authorization-response',
)
export const privateDecryptionOperationRequest = factory(
  'private-decryption:operation-request',
)
export const privateDecryptionOperationResponse = factory(
  'private-decryption:operation-response',
)
export const inputRegistrationRequest = factory(
  'input-registration:input-registration-request',
)
export const inputRegistrationResponse = factory(
  'input-registration:input-registration-response',
)

export function isRelayerEvent(data: unknown): data is RelayerEvent {
  return schema.safeParse(data).success
}
