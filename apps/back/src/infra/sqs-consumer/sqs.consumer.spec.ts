import { beforeEach, describe, expect, test } from 'vitest'
import { SQSConsumer } from './sqs.consumer.js'
import { PUBSUB } from '#constants.js'
import { Task, unknownError, type IPubSub } from 'utils'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { Message } from '@aws-sdk/client-sqs'
import { Mocked } from '@suites/doubles.vitest'
import { TestBed } from '@suites/unit'

describe('SqsConsumer', () => {
  describe(`given it's listening to a queue`, () => {
    let pubsub: Mocked<IPubSub<back.BackEvent>>
    let consumer: SQSConsumer

    beforeEach(async () => {
      const { unit, unitRef } = await TestBed.solitary(SQSConsumer).compile()

      consumer = unit
      pubsub = unitRef.get(PUBSUB) as unknown as Mocked<IPubSub<back.BackEvent>>
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
