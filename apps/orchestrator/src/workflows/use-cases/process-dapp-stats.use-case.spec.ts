import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { ProcessDAppStats } from './process-dapp-stats.use-case.js'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { back, operationNames, web3 } from 'messages'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { faker } from '@faker-js/faker'
import { DAppStatsEvents } from '#workflows/entities/dapp-stats.js'
import { TestBed } from '@suites/unit'

describe(ProcessDAppStats, () => {
  let pubsub: IPubSub<DAppStatsEvents>
  let producer: Mocked<EventProducer>

  beforeEach(async () => {
    pubsub = new PubSub()
    const { unitRef } = await TestBed.solitary(ProcessDAppStats)
      .mock(PUBSUB)
      .impl(() => pubsub)
      .compile()

    producer = unitRef.get(EVENT_PRODUCER) as unknown as Mocked<EventProducer>
    producer.publish.mockReturnValue(Task.of(void 0))
  })

  describe(`when pubsub publishes an 'back:dapp:stats-requested' event`, () => {
    let task: Task<void, AppError>
    let event: Extract<back.BackEvent, { type: 'back:dapp:stats-requested' }>

    beforeEach(() => {
      event = back.dappStatsRequested(
        {
          requestId: faker.string.uuid(),
          dAppId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      )
      task = pubsub.publish(event)
    })

    test(`then it publishes a 'web3:fhe-event:requested' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'web3:fhe-event:requested',
        }),
      )
    })

    test('then it publishes the right payload', async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            chainId: event.payload.chainId,
            address: event.payload.address,
            dAppId: event.payload.dAppId,
            requestId: event.payload.requestId,
          },
        }),
      )
    })

    test('then it publishes the right meta', async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: { correlationId: event.meta.correlationId },
        }),
      )
    })
  })

  describe(`when pubsub publishes an 'web3:fhe-event:detected' event`, () => {
    let task: Task<void, AppError>
    let event: Extract<web3.Web3Event, { type: 'web3:fhe-event:detected' }>

    beforeEach(() => {
      event = web3.fheDetected(
        {
          requestId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
          events: [
            {
              id: faker.string.alphanumeric(10),
              name: faker.helpers.arrayElement(operationNames),
              timestamp: faker.date.past().toISOString(),
            },
          ],
        },
        { correlationId: faker.string.uuid() },
      )
      task = pubsub.publish(event)
    })

    test(`then it publishes an 'back:dapp:stats-available' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({ type: 'back:dapp:stats-available' }),
      )
    })

    test('then it publishes the right payload', async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: expect.objectContaining({
            requestId: event.payload.requestId,
            chainId: event.payload.chainId,
            address: event.payload.address,
            events: expect.arrayContaining([
              expect.objectContaining({
                externalRef: event.payload.events[0].id,
                name: event.payload.events[0].name,
                timestamp: event.payload.events[0].timestamp,
              }),
            ]),
          }),
        }),
      )
    })

    test('then it publishes the right meta', async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: expect.objectContaining({
            correlationId: event.meta.correlationId,
          }),
        }),
      )
    })
  })

  describe.each([
    {
      event: back.dappStatsAvailable(
        {
          requestId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
          events: [
            {
              name: faker.helpers.arrayElement(operationNames),
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
          ],
        },
        { correlationId: faker.string.uuid() },
      ),
    },
    {
      event: web3.fheRequested(
        {
          requestId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ),
    },
  ])('when pubsub publishes an $event.type event', ({ event }) => {
    let task: Task<void, AppError>

    beforeEach(() => {
      task = (pubsub as IPubSub<back.BackEvent | web3.Web3Event>).publish(event)
    })

    test('then it ignores it', async () => {
      await task.toPromise()
      expect(producer.publish).not.toHaveBeenCalled()
    })
  })
})
