import { back } from 'messages'
import { IPubSub, Task, unknownError } from 'utils'
import { beforeEach, describe, expect, test } from 'vitest'
import { mock, MockProxy } from 'vitest-mock-extended'
import { SQSConsumer } from './sqs.consumer.js'
import { Test } from '@nestjs/testing'
import { MS_NAME, PUBSUB } from '#constants.js'
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

    describe.each([
      ['back', true],
      ['email', true],
      ['orch', false],
      ['web3', true],
    ])(
      'when it receives a message from the %s microservice',
      (sender, forward) => {
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

        test(`should ${forward ? 'forward it to the pubsub' : 'stop the propagation'}`, async () => {
          pubsub.publish.mockReturnValue(Task.of(void 0))
          const message = encodeMessage(event, { sender })

          await consumer.handleMessage(message)
          if (forward) {
            expect(pubsub.publish).toBeCalledWith({
              ...event,
              meta: {
                ...event.meta,
                [`${MS_NAME}-dir`]: 'in',
              },
            })
          } else {
            expect(pubsub.publish).not.toBeCalled()
          }
        })

        if (forward) {
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
        }
      },
    )
  })
})

function encodeMessage(
  message: object,
  options?: { sender?: string },
): Message {
  return {
    Body: JSON.stringify(message),
    MessageAttributes: options?.sender
      ? {
          Sender: { DataType: 'String', StringValue: options.sender },
        }
      : {},
  }
}
