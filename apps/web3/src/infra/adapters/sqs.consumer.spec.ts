import { faker } from '@faker-js/faker'
import { Test } from '@nestjs/testing'
import { beforeEach, describe, expect, test } from 'vitest'
import { mock, type MockProxy } from 'vitest-mock-extended'
import { SQSConsumer } from './sqs.consumer.js'
import { IPubSub, Task, unknownError } from 'utils'
import { web3 } from 'messages'
import { PUBSUB } from '#constants.js'
import { DiscoverContract } from '#use-cases/discover-contract.use-case.js'
import { Message } from '@aws-sdk/client-sqs'

describe('SQSConsumer', () => {
  describe(`given it's listening to a queue`, () => {
    let pubsub: MockProxy<IPubSub<web3.Web3Event>>
    let consumer: SQSConsumer

    beforeEach(async () => {
      pubsub = mock<IPubSub<web3.Web3Event>>()
      const moduleRef = await Test.createTestingModule({
        providers: [
          SQSConsumer,
          { provide: PUBSUB, useValue: pubsub },
          { provide: DiscoverContract, useValue: mock() },
        ],
      }).compile()

      consumer = moduleRef.get(SQSConsumer)
    })

    describe(`when it receives a 'web3:fhe-event:requested' event`, () => {
      test('should forward it to the pubsub', async () => {
        pubsub.publish.mockReturnValue(Task.of(void 0))
        const event = getFheEventRequested()
        const message = encodeMessage(event)

        await consumer.handleMessage(message)
        expect(pubsub.publish).toBeCalledWith(event)
      })

      test('should forward errors', async () => {
        pubsub.publish.mockReturnValue(
          Task.reject(unknownError('Mocked error')),
        )
        const event = getFheEventRequested()
        const message = encodeMessage(event)

        await expect(consumer.handleMessage(message)).rejects.toThrow(
          'Mocked error',
        )
      })
    })

    describe(`when it receives a 'web3:fhe-event:detected' event`, () => {
      test('should drop it', async () => {
        const event = getFheEventDetected()
        const message = encodeMessage(event)

        await consumer.handleMessage(message)
        expect(pubsub.publish).not.toBeCalled()
      })
    })
  })
})

function encodeMessage(message: object): Message {
  return {
    Body: JSON.stringify({
      Message: JSON.stringify(message),
    }),
  }
}

function getFheEventRequested() {
  return web3.fheRequested(
    {
      chainId: faker.string.numeric(5),
      address: faker.string.hexadecimal({ length: 40 }),
    },
    { correlationId: faker.string.uuid() },
  )
}

function getFheEventDetected() {
  return web3.fheDetected(
    {
      chainId: faker.string.numeric(5),
      address: faker.string.hexadecimal({ length: 40 }),
      name: faker.string.alphanumeric(10),
      timestamp: faker.date.anytime().toISOString(),
      id: faker.string.alphanumeric(10),
    },
    { correlationId: faker.string.uuid() },
  )
}
