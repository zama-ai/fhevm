import { z } from 'zod'
import { schema as dAppSchema } from '#dapps/domain/entities/dapp.js'

export type SubscriptionTypes =
  | 'dummy'
  | 'dappCreated'
  | 'dappUpdated'
  | 'dappDeleted'

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
  dummy: genSchema('dummy', {
    dummy: z.object({
      id: z.string(),
      name: z.string(),
    }),
  }),
}

export const schema = z.discriminatedUnion('type', [
  subscriptionMap['dummy'],
  subscriptionMap['dappUpdated'],
])
export type Subscription = z.infer<typeof schema>
export type SubscriptionPayload = Extract<
  Subscription,
  { type: SubscriptionTypes }
>['payload']
