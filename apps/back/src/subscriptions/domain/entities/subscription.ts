import { z } from 'zod'
import { schema as dAppSchema } from '#dapps/domain/entities/dapp.js'

export type SubscriptionTypes = 'dappCreated' | 'dappUpdated' | 'dappDeleted'

function genSchema<
  Key extends SubscriptionTypes,
  Payload extends z.ZodRawShape,
>(type: Key, payload: Payload) {
  return z.object({
    type: z.literal(type),
    payload: z.object(payload),
  })
}

const subscriptionMap = {
  dappCreated: genSchema('dappCreated', {
    dapp: dAppSchema,
  }),
  dappUpdated: genSchema('dappUpdated', {
    dapp: dAppSchema,
  }),
  dappDeleted: genSchema('dappDeleted', {
    dapp: dAppSchema,
  }),
}

export const schema = z.discriminatedUnion('type', [
  subscriptionMap['dappUpdated'],
])
export type Subscription = z.infer<typeof schema>
