import { PubSub } from 'graphql-subscriptions'
import { Injectable } from '@nestjs/common'

import { SubscriptionTypes } from '../domain/entities/subscription.js'
import { SubscriptionId } from '../domain/entities/subscription-id.js'
import { SubscriptionService } from '../domain/services/subscription.service.js'

@Injectable()
export class PubSubSubscriptionService implements SubscriptionService {
  // Note that the default PubSub implementation is intended for demo purposes. It only works
  // if you have a single instance of your server and doesn't scale beyond a couple of connections.
  // For production usage you'll want to use one of the PubSub implementations backed by an
  // external store. (e.g. Redis)
  // https://github.com/apollographql/graphql-subscriptions?tab=readme-ov-file#pubsub-implementations
  // https://github.com/zama-zws/console/issues/121
  #pubSub = new PubSub()

  publish<T>(topic: SubscriptionTypes, payload: T): Promise<void> {
    this.#pubSub.asyncIterableIterator(topic)
    return this.#pubSub.publish(topic, payload)
  }

  async subscribe<T>(
    topic: string,
    callback: (payload: T) => void,
  ): Promise<SubscriptionId> {
    const subId = await this.#pubSub.subscribe(topic, callback)
    return new SubscriptionId(subId)
  }

  unsubscribe(id: SubscriptionId): void {
    this.#pubSub.unsubscribe(id.value)
  }

  asyncIterableIterator<T>(topic: string): AsyncIterableIterator<T> {
    return this.#pubSub.asyncIterableIterator(topic)
  }
}
