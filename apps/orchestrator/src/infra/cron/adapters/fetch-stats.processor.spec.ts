import { afterEach, beforeEach, describe, expect, Mock, test, vi } from 'vitest'
import { FetchStatsProcessor } from './fetch-stats.processor.js'
import { Test } from '@nestjs/testing'
import { IPubSub, PubSub, Task } from 'utils'
import { back, web3 } from 'messages'
import { PUBSUB } from '#constants.js'
import { Job } from 'bullmq'
import { mock, MockProxy } from 'vitest-mock-extended'
import { faker } from '@faker-js/faker'

describe(FetchStatsProcessor, () => {
  let pubsub: IPubSub<back.BackEvent | web3.Web3Event>
  let processor: FetchStatsProcessor
  let job: MockProxy<Job>
  let chainId: string

  beforeEach(async () => {
    pubsub = new PubSub()
    chainId = faker.string.numeric(5)
    job = mock({
      name: 'test',
      data: { chainId },
    })
    const moduleRef = await Test.createTestingModule({
      providers: [{ provide: PUBSUB, useValue: pubsub }, FetchStatsProcessor],
    }).compile()
    processor = moduleRef.get(FetchStatsProcessor)
  })

  describe('when the processor is invoked', () => {
    let handler: Mock

    beforeEach(() => {
      handler = vi.fn()
      handler.mockReturnValue(Task.of(void 0))
      pubsub.subscribe('*', handler)
    })

    afterEach(() => {
      vi.resetAllMocks()
    })

    describe('then it fires an event', () => {
      beforeEach(async () => {
        await processor.process(job)
      })

      test('with of the `back:dapp:stats-requested` type', async () => {
        expect(handler).toHaveBeenCalledExactlyOnceWith(
          expect.objectContaining({
            type: 'back:dapp:stats-requested',
          }),
        )
      })

      test('with the right chainId', async () => {
        expect(handler).toHaveBeenCalledExactlyOnceWith(
          expect.objectContaining({
            payload: expect.objectContaining({ chainId }),
          }),
        )
      })

      test('with filler parameters', async () => {
        expect(handler).toHaveBeenCalledExactlyOnceWith(
          expect.objectContaining({
            payload: expect.objectContaining({
              dAppId: 'cron',
              address: '0x0000000000000000000000000000000000000000',
            }),
          }),
        )
      })

      test('with a valid BackEvent', async () => {
        expect(handler).toHaveBeenCalledOnce()
        const event = handler.mock.calls[0][0]
        if (!back.isBackEvent(event)) {
          console.log(
            `failed to parse event: ${JSON.stringify(back.schema.safeParse(event))}`,
          )
        }
        expect(back.isBackEvent(event)).toBe(true)
      })
    })

    test('then it ')
  })
})
