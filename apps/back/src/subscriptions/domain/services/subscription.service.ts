import type {
  Subscription,
  SubscriptionPayload,
  SubscriptionTypes,
} from '../entities/subscription.js'
import { SubscriptionId } from '../entities/subscription-id.js'

export interface SubscriptionService {
  /**
   * Publishes a message to a specific topic.
   *
   * @template K - The type of the topic key from Subscription.
   * @param {K} topic - The topic to which the message will be published.
   * @param {Subscription[K]} payload - The payload of the message to be published.
   * @returns {Promise<void>} A promise that resolves when the message has been published.
   */
  publish(topic: SubscriptionTypes, payload: SubscriptionPayload): Promise<void>

  /**
   * Subscribes to a specific topic and registers a callback to be invoked
   * when a message is published to that topic.
   *
   * @template K - The type of the topic key from Subscription.
   * @param {K} topic - The topic to subscribe to.
   * @param {(payload: Subscription[K]) => void} callback - The callback function
   * to be executed when a message is published to the subscribed topic.
   * @returns {Promise<SubscriptionId>} A promise that resolves to a SubscriptionId
   * which can be used to unsubscribe from the topic.
   */
  subscribe<K extends keyof Subscription>(
    topic: K,
    callback: (payload: Subscription[K]) => void,
  ): Promise<SubscriptionId>

  /**
   * Unsubscribes from a specific topic.
   *
   * @param {SubscriptionId} id - The SubscriptionId to unsubscribe from.
   */
  unsubscribe(id: SubscriptionId): void

  asyncIterableIterator<K extends SubscriptionTypes>(
    topic: K,
  ): AsyncIterableIterator<SubscriptionPayload>
}

export const SUBSCRIPTION_SERVICE = Symbol('SubscriptionService')
