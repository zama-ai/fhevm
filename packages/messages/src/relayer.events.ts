import { z } from 'zod'
import { chainId, meta, metaFactory, requestId, web3Address } from './shared.js'

type EventTypes =
  | 'public-decryption:authorization-request'
  | 'public-decryption:authorization-response'
  | 'private-decryption:operation-request'
  | 'rivate-decryption:operation-response'

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

function hexString(options?: { length?: number; prefix?: boolean }) {
  const prefix = options?.prefix ? '0x' : ''
  const length = options?.length ? `{${options.length}}` : '+'
  const regex = `^${prefix}[\\da-f]${length}$`
  return z.string().regex(new RegExp(regex, 'i'))
}

const schemas = [
  genSchema('public-decryption:authorization-request', {
    callerAddress: web3Address,
  }),
  genSchema('public-decryption:authorization-response', {
    result: z.string(),
    authorized: z.boolean(),
  }),
  genSchema('private-decryption:operation-request', {
    ctHandles: z.array(hexString({ prefix: true })),
    publicKey: hexString({ prefix: true }),
    chainId,
  }),
  genSchema('rivate-decryption:operation-response', {
    ctValues: z.array(hexString({ prefix: true })),
    signatures: z.array(hexString({ prefix: true })),
  }),
] as const

export const schema = z.discriminatedUnion('type', [...schemas]).and(
  z.object({
    meta,
  }),
)
export type RelayerEvent = z.infer<typeof schema>

const factory = metaFactory<RelayerEvent>('relayer')
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
  'rivate-decryption:operation-response',
)

export function isRelayerEvent(data: unknown): data is RelayerEvent {
  return schema.safeParse(data).success
}
