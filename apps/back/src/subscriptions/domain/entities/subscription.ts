import { z } from 'zod'
import { DApp, schema as dAppSchema } from '#dapps/domain/entities/dapp.js'

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
    dappUpdated: dAppSchema,
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

// TODO: use deiscriminatedUnion instead of calling types directly
export type SubscriptionDummyPayload = {
  dummy: {
    id: string
    name: string
  }
}

export type SubscriptionDappUpdatedPayload = {
  dappUpdated: DApp
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
