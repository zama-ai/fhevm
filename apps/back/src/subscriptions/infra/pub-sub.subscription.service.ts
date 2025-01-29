import { RedisPubSub } from 'graphql-redis-subscriptions'
import { Injectable } from '@nestjs/common'
import { Redis } from 'ioredis'

const options = {
  host: process.env.REDIS_HOST,
  port: parseInt(process.env.REDIS_PORT ?? '6379', 10),
  password: process.env.REDIS_PASSWORD,
  db: 0,
  retryStrategy: (times: number) => {
    return Math.min(times * 50, 2000)
  },
}
import { SubscriptionTypes } from '../domain/entities/subscription.js'
import { SubscriptionId } from '../domain/entities/subscription-id.js'
import { SubscriptionService } from '../domain/services/subscription.service.js'

@Injectable()
export class PubSubSubscriptionService implements SubscriptionService {
  #pubSub = new RedisPubSub({
    publisher: new Redis(options),
    subscriber: new Redis(options),
  })

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
