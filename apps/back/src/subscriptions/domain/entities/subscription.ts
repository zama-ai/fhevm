import { DAppProps } from '#dapps/domain/entities/dapp.js'

export type SubscriptionTypes = 'dappCreated' | 'dappUpdated' | 'dappDeleted'

// TODO: use discriminatedUnion instead of calling types directly
export type SubscriptionDappUpdatedPayload = {
  dappUpdated: DAppProps
}

export type SubscriptionPayload = SubscriptionDappUpdatedPayload
