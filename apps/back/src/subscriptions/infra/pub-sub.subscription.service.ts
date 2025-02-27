import { RedisPubSub } from 'graphql-redis-subscriptions'
import { Injectable, Logger, OnModuleDestroy } from '@nestjs/common'
import { Redis, RedisOptions } from 'ioredis'
import { ConfigService } from '@nestjs/config'

import { SubscriptionTypes } from '../domain/entities/subscription.js'
import { SubscriptionId } from '../domain/entities/subscription-id.js'
import { SubscriptionService } from '../domain/services/subscription.service.js'

@Injectable()
export class PubSubSubscriptionService
  implements SubscriptionService, OnModuleDestroy
{
  #pubSub: RedisPubSub
  logger = new Logger(PubSubSubscriptionService.name)

  constructor(config: ConfigService) {
    const options: RedisOptions = {
      host: config.get('redis.host'),
      port: config.get('redis.port'),
      db: 0,
      retryStrategy: (times: number) => {
        return Math.min(times * 50, 2000)
      },
    }
    this.logger.debug(`connecting to redis: ${JSON.stringify(options)}`)
    this.#pubSub = new RedisPubSub({
      publisher: new Redis(options),
      subscriber: new Redis(options),
    })
  }

  publish<T>(topic: SubscriptionTypes, payload: T): Promise<void> {
    this.#pubSub.asyncIterableIterator(topic)
    return this.#pubSub.publish(topic, payload)
  }

  async subscribe<T>(
    topic: string,
    callback: (payload: T) => void,
  ): Promise<SubscriptionId> {
    const subId = await this.#pubSub.subscribe(topic, callback)
    this.logger.debug(`${subId} subscribed to topic ${topic}`)
    return SubscriptionId.from(subId)
  }

  unsubscribe(id: SubscriptionId): void {
    this.logger.debug(`${id.value} unsubscribed`)
    this.#pubSub.unsubscribe(id.value)
  }

  asyncIterableIterator<T>(topic: string): AsyncIterableIterator<T> {
    return this.#pubSub.asyncIterableIterator(topic)
  }

  async onModuleDestroy() {
    this.logger.debug('closing pubsub')
    await this.#pubSub.close()
  }
}
