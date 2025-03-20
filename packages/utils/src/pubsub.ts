import { AppError } from './app-error.js'
import { Task } from './task.js'

type EventObject = { type: string; payload: object }
export type GetPayload<
  T extends EventObject,
  TKey extends EventDescriptor<T>,
> = T extends any
  ? TKey extends EventDescriptor<T>
    ? 'payload' extends keyof T
      ? T['payload']
      : undefined
    : never
  : never
type PartialEventDescriptor<TEventType extends string> =
  TEventType extends `${infer TLeading}:${infer TTail}`
    ? `${TLeading}:*` | `${TLeading}:${PartialEventDescriptor<TTail>}`
    : never

export type EventDescriptor<TEvent extends EventObject> =
  | TEvent['type']
  | PartialEventDescriptor<TEvent['type']>
  | '*'

export type ISubscriber<
  TEvent extends EventObject,
  // TType extends EventDescriptor<TEvent>,
> = (event: TEvent /*, key: TType*/) => Task<void, AppError>

type ISubscriberMap<TEvent extends EventObject> = {
  // [K in EventDescriptor<TEvent>]: Subscriber<TEvent, K>[]
  [K in EventDescriptor<TEvent>]: ISubscriber<TEvent>[]
}

export interface IPubSub<TEvent extends EventObject> {
  subscribe<TKey extends EventDescriptor<TEvent>>(
    descriptor: TKey,
    subscriber: ISubscriber<TEvent>,
  ): void

  unsubscribe<TKey extends EventDescriptor<TEvent>>(
    descriptor: TKey,
    subscriber: ISubscriber<TEvent>,
  ): void

  once<TKey extends EventDescriptor<TEvent>>(
    descriptor: TKey,
    subscriber: ISubscriber<TEvent>,
  ): void

  publish(event: TEvent): Task<void, AppError>
}

export class PubSub<TEvent extends EventObject> implements IPubSub<TEvent> {
  #subscribers: Partial<ISubscriberMap<TEvent>> = {}

  subscribe<TKey extends EventDescriptor<TEvent>>(
    descriptor: TKey,
    subscriber: ISubscriber<TEvent>,
  ) {
    if (!this.#subscribers[descriptor]) {
      this.#subscribers[descriptor] = []
    }
    this.#subscribers[descriptor].push(subscriber)
  }

  once<TKey extends EventDescriptor<TEvent>>(
    descriptor: TKey,
    subscriber: ISubscriber<TEvent>,
  ) {
    if (!this.#subscribers[descriptor]) {
      this.#subscribers[descriptor] = []
    }
    // I wrap the subscriber with a proxy, so I can remove it after
    // the first call
    const handler: ISubscriber<TEvent> = event => {
      this.#subscribers[descriptor] = this.#subscribers[descriptor]!.filter(
        s => s !== handler,
      )
      return subscriber(event)
    }
    this.#subscribers[descriptor].push(handler)
  }

  unsubscribe<TKey extends EventDescriptor<TEvent>>(
    descriptor: TKey,
    subscriber: ISubscriber<TEvent>,
  ) {
    this.#subscribers[descriptor] = this.#subscribers[descriptor]?.filter(
      s => s !== subscriber,
    )
  }

  publish(event: TEvent): Task<void, AppError> {
    // const handlers: Subscriber<TEvent, TEvent['type']>[] = Object.entries(
    //   this.#subscribers,
    // )
    const handlers: ISubscriber<TEvent>[] = Object.entries(this.#subscribers)
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      .filter(([key, _handlers]) => {
        if (key === '*') {
          return true
        }

        if (key.endsWith(':*')) {
          return event.type.startsWith(key.slice(0, -2))
        }

        return key === event.type
      })
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      .flatMap(([_key, handlers]) => handlers)

    return Task.all<AppError, void>(
      handlers.map(handler => handler(event)),
    ).map(() => void 0)
  }
}
