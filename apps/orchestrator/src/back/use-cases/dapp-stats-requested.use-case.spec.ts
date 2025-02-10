import { beforeEach, describe, expect, test } from 'vitest'
import { CalledWithMock, mockFn } from 'vitest-mock-extended'
import { Test, TestingModule } from '@nestjs/testing'
import { PUBSUB } from '#constants.js'
import { AppError, ISubscriber, PubSub, Task } from 'utils'
import { back, web3 } from 'messages'
import { faker } from '@faker-js/faker'
import { DAppStatsRequested } from './dapp-stats-requested.use-case.js'

describe(DAppStatsRequested, () => {
  let moduleRef: TestingModule

  beforeEach(async () => {
    moduleRef = await Test.createTestingModule({
      providers: [
        {
          provide: PUBSUB,
          useValue: new PubSub(),
        },
        {
          provide: DAppStatsRequested,
          inject: [PUBSUB],
          useFactory: (pubsub: PubSub<back.BackEvent | web3.Web3Event>) =>
            new DAppStatsRequested(pubsub),
        },
      ],
    }).compile()
  })

  describe(`when pubsub publishes an 'back:dapp:stats-requested' event`, () => {
    let pubsub: PubSub<back.BackEvent | web3.Web3Event>
    let handler: CalledWithMock<
      Task<void, AppError>,
      [back.BackEvent | web3.Web3Event]
    >
    let task: Task<void, AppError>
    let event: Extract<back.BackEvent, { type: 'back:dapp:stats-requested' }>

    beforeEach(() => {
      pubsub = moduleRef.get(PUBSUB)
      handler = mockFn<
        ISubscriber<back.BackEvent | web3.Web3Event>
      >().mockReturnValue(Task.of(void 0))
      pubsub.subscribe(
        'web3:fhe-event:requested',
        handler as ISubscriber<back.BackEvent | web3.Web3Event>,
      )
      event = back.dappStatsRequested(
        {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ) as Extract<back.BackEvent, { type: 'back:dapp:stats-requested' }>
      task = pubsub.publish(event)
    })

    test(`then it publishes a 'web3:fhe-event:requested' event`, async () => {
      await task.toPromise()
      expect(handler).toHaveBeenCalledOnce()
      const { type } = handler.mock.calls[0][0]
      expect(type).toBe('web3:fhe-event:requested')
    })

    test('then it publishes the right payload', async () => {
      await task.toPromise()
      expect(handler).toHaveBeenCalledOnce()
      const { payload } = handler.mock.calls[0][0]
      expect(payload.chainId, 'Wrong chainId').toBe(event.payload.chainId)
      expect(payload.address, 'Wrong address').toBe(event.payload.address)
    })

    test('then it publishes the right meta', async () => {
      await task.toPromise()
      expect(handler).toHaveBeenCalledOnce()
      const { meta } = handler.mock.calls[0][0]
      expect(meta).toEqual(event.meta)
    })
  })

  describe.each([
    {
      event: back.dappStatsAvailable(
        {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(10),
          timestamp: faker.date.past().toISOString(),
          externalRef: faker.string.alphanumeric(10),
        },
        { correlationId: faker.string.uuid() },
      ),
    },
    {
      event: web3.fheDetected(
        {
          id: faker.string.alphanumeric(10),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(10),
          timestamp: faker.date.past().toISOString(),
        },
        { correlationId: faker.string.uuid() },
      ),
    },
    {
      event: web3.fheRequested(
        {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ),
    },
  ])('when pubsub publishes an $event.type event', ({ event }) => {
    let pubsub: PubSub<back.BackEvent | web3.Web3Event>
    let handler: CalledWithMock<
      Task<void, AppError>,
      [back.BackEvent | web3.Web3Event]
    >
    let task: Task<void, AppError>

    beforeEach(() => {
      pubsub = moduleRef.get(PUBSUB)
      handler = mockFn<
        ISubscriber<back.BackEvent | web3.Web3Event>
      >().mockReturnValue(Task.of(void 0))
      pubsub.subscribe(
        'back:dapp:stats-requested',
        handler as ISubscriber<back.BackEvent | web3.Web3Event>,
      )
      task = pubsub.publish(event)
    })

    test('then it ignores it', async () => {
      await task.toPromise()
      expect(handler).not.toBeCalled()
    })
  })
})
