import { z } from 'zod'
import { chainId, Meta, meta, requestId, web3Address, requestValidity, ctHandleContractPairs } from './shared.js'

// prefixed hex encoded
// const hexEncoded = z.string().startsWith('0x').and(z.custom<`0x${string}`>())

type EventTypes =
  | 'public-decryption:authorization-request'
  | 'public-decryption:authorization-response'
  // NOTE: add once spec is ready
  // | 'public-decryption:operation-request'
  // | 'public-decryption:operation-response'
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
    contractsChainId: chainId,
    ctHandleContractPairs: z.array(ctHandleContractPairs),
    requestValidity: requestValidity,
    contractsAddresses: z.array(web3Address),
    userAddress: web3Address,
    signature: z.string(),
    publicKey: z.string(),
  }),
  genSchema('private-decryption:operation-response', {
    gatewayRequestId: z.number(),
    decryptedValue: z.string(),
    signatures: z.array(z.string()),
  }),
  genSchema('input-registration:input-registration-request', {
    contractChainId: chainId,
    contractAddress: web3Address,
    userAddress: web3Address,
    ciphertextWithInputVerification: z.string(),
  }),
  genSchema('input-registration:input-registration-response', {
    handles: z.array(z.string()),
    signatures: z.array(z.string()),
  }),
] as const

export const schema = z
  .discriminatedUnion('type', [...schemas])
  .and(z.object({ meta: meta.optional() }))
export type RelayerEvent = z.infer<typeof schema>

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<
  K extends EventTypes,
  Event extends {
    type: `relayer:${K}`
    payload: object
    meta?: Meta
  } = Extract<RelayerEvent, { type: `relayer:${K}` }>,
>(type: K) {
  return function(payload: Event['payload'], meta?: Meta) {
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
