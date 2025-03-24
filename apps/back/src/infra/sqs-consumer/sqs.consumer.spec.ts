import { Test } from '@nestjs/testing'
import { beforeEach, describe, expect, test } from 'vitest'
import { SQSConsumer } from './sqs.consumer.js'
import { PUBSUB } from '#constants.js'
import { mock, MockProxy } from 'vitest-mock-extended'
import { Task, unknownError, type IPubSub } from 'utils'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { Message } from '@aws-sdk/client-sqs'

describe('SqsConsumer', () => {
  describe(`given it's listening to a queue`, () => {
    let pubsub: MockProxy<IPubSub<back.BackEvent>>
    let consumer: SQSConsumer

    beforeEach(async () => {
      pubsub = mock<IPubSub<back.BackEvent>>()
      const moduleRef = await Test.createTestingModule({
        providers: [
          SQSConsumer,
          {
            provide: PUBSUB,
            useValue: pubsub,
          },
        ],
      }).compile()

      consumer = moduleRef.get(SQSConsumer)
    })

    describe('when it receives a message', () => {
      let event: back.BackEvent
      beforeEach(() => {
        event = {
          type: 'back:dapp:stats-requested',
          payload: {
            requestId: faker.string.uuid(),
            dAppId: faker.string.uuid(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          meta: {
            correlationId: faker.string.uuid(),
          },
        }
      })

      test(`should forward it to the pubsub`, async () => {
        pubsub.publish.mockReturnValue(Task.of(void 0))
        const message = encodeMessage(event)

        await consumer.handleMessage(message)
        expect(pubsub.publish).toBeCalledWith(event)
      })

      test('should forward errors', async () => {
        pubsub.publish.mockReturnValue(
          Task.reject(unknownError('Mocked error')),
        )
        const messageId = faker.string.uuid()
        const message = encodeMessage(event)
        message.MessageId = messageId

        const result = await consumer.handleMessage(message)
        expect(result).toEqual({
          batchItemFailures: [{ itemIdentifier: messageId }],
        })
      })
    })
  })
})

function encodeMessage(message: object): Message {
  return {
    Body: JSON.stringify(message),
  }
}
