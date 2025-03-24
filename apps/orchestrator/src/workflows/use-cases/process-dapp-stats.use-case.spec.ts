import { beforeEach, describe, expect, test } from 'vitest'
import { ProcessDAppStats } from './process-dapp-stats.use-case.js'
import { Test, TestingModule } from '@nestjs/testing'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { AppError, PubSub, Task } from 'utils'
import { back, web3 } from 'messages'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { CalledWithMock, mockFn } from 'vitest-mock-extended'
import { faker } from '@faker-js/faker'

describe(ProcessDAppStats, () => {
  let moduleRef: TestingModule
  let publish: CalledWithMock<
    Task<void, AppError>,
    [back.BackEvent | web3.Web3Event]
  >

  beforeEach(async () => {
    publish = mockFn<EventProducer['publish']>().mockReturnValue(
      Task.of(void 0),
    )

    moduleRef = await Test.createTestingModule({
      providers: [
        {
          provide: PUBSUB,
          useValue: new PubSub(),
        },
        {
          provide: EVENT_PRODUCER,
          useValue: {
            publish,
          } satisfies EventProducer,
        },
        {
          provide: ProcessDAppStats,
          inject: [PUBSUB, EVENT_PRODUCER],
          useFactory: (
            pubsub: PubSub<back.BackEvent | web3.Web3Event>,
            producer: EventProducer,
          ) => new ProcessDAppStats(pubsub, producer),
        },
      ],
    }).compile()
  })

  describe(`when pubsub publishes an 'back:dapp:stats-requested' event`, () => {
    let pubsub: PubSub<back.BackEvent | web3.Web3Event>
    let task: Task<void, AppError>
    let event: Extract<back.BackEvent, { type: 'back:dapp:stats-requested' }>

    beforeEach(() => {
      pubsub = moduleRef.get(PUBSUB)
      event = back.dappStatsRequested(
        {
          requestId: faker.string.uuid(),
          dAppId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ) as Extract<back.BackEvent, { type: 'back:dapp:stats-requested' }>
      task = pubsub.publish(event)
    })

    test(`then it publishes a 'web3:fhe-event:requested' event`, async () => {
      await task.toPromise()
      expect(publish).toHaveBeenCalledOnce()
      const { type } = publish.mock.calls[0][0]
      expect(type).toBe('web3:fhe-event:requested')
    })

    test('then it publishes the right payload', async () => {
      await task.toPromise()
      expect(publish).toHaveBeenCalledOnce()
      const { payload } = publish.mock.calls[0][0]
      expect((payload as any).chainId, 'Wrong chainId').toBe(
        event.payload.chainId,
      )
      expect((payload as any).address, 'Wrong address').toBe(
        event.payload.address,
      )
    })

    test('then it publishes the right meta', async () => {
      await task.toPromise()
      expect(publish).toHaveBeenCalledOnce()
      const { meta } = publish.mock.calls[0][0]
      expect(meta).toEqual(event.meta)
    })
  })

  describe(`when pubsub publishes an 'web3:fhe-event:detected' event`, () => {
    let pubsub: PubSub<back.BackEvent | web3.Web3Event>
    let task: Task<void, AppError>
    let event: Extract<web3.Web3Event, { type: 'web3:fhe-event:detected' }>

    beforeEach(() => {
      pubsub = moduleRef.get(PUBSUB)
      event = web3.fheDetected(
        {
          requestId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          events: [
            {
              id: faker.string.alphanumeric(10),
              name: faker.string.alphanumeric(10),
              timestamp: faker.date.past().toISOString(),
            },
          ],
        },
        { correlationId: faker.string.uuid() },
      ) as Extract<web3.Web3Event, { type: 'web3:fhe-event:detected' }>
      task = pubsub.publish(event)
    })

    test(`then it publishes an 'back:dapp:stats-available' event`, async () => {
      await task.toPromise()
      expect(publish).toHaveBeenCalledOnce()
      const { type } = publish.mock.calls[0][0]
      expect(type).toBe('back:dapp:stats-available')
    })

    test('then it publishes the right payload', async () => {
      await task.toPromise()
      expect(publish).toHaveBeenCalledOnce()
      const { payload } = publish.mock.calls[0][0]
      expect(payload.requestId, 'Wrong requestId').toBe(event.payload.requestId)
      expect((payload as any).chainId, 'Wrong chainId').toBe(
        event.payload.chainId,
      )
      expect((payload as any).address, 'Wrong address').toBe(
        event.payload.address,
      )
      expect((payload as any).events[0].name, 'Wrong name').toBe(
        event.payload.events[0].name,
      )
      expect((payload as any).events[0].timestamp, 'Wrong timestamp').toBe(
        event.payload.events[0].timestamp,
      )
      expect((payload as any).events[0].externalRef, 'Wrong externalREf').toBe(
        event.payload.events[0].id,
      )
    })

    test('then it publishes the right meta', async () => {
      await task.toPromise()
      expect(publish).toHaveBeenCalledOnce()
      const { meta } = publish.mock.calls[0][0]
      expect(meta).toEqual(event.meta)
    })
  })

  describe.each([
    {
      event: back.dappStatsAvailable(
        {
          requestId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          events: [
            {
              name: faker.string.alphanumeric(10),
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
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ),
    },
  ])('when pubsub publishes an $event.type event', ({ event }) => {
    let pubsub: PubSub<back.BackEvent | web3.Web3Event>
    let task: Task<void, AppError>

    beforeEach(() => {
      pubsub = moduleRef.get(PUBSUB)
      task = pubsub.publish(event)
    })

    test('then it ignores it', async () => {
      await task.toPromise()
      expect(publish).not.toBeCalled()
    })
  })
})
