import { faker } from '@faker-js/faker'
import { Test } from '@nestjs/testing'
import { beforeEach, describe, expect, test } from 'vitest'
import { mock, type MockProxy } from 'vitest-mock-extended'
import { SQSConsumer } from './sqs.consumer.js'
import { IPubSub, Task, unknownError } from 'utils'
import { web3 } from 'messages'
import { MS_NAME, PUBSUB } from '#constants.js'
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

    describe.each([
      ['back', true],
      ['email', true],
      ['orch', true],
      ['web3', false],
    ])(
      'when it receives a message from the %s microservice',
      (sender, forward) => {
        let event: web3.Web3Event
        beforeEach(() => {
          event = {
            type: 'web3:fhe-event:requested',
            payload: {
              requestId: faker.string.uuid(),
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
    Body: JSON.stringify({
      Message: JSON.stringify(message),
      MessageAttributes: options?.sender
        ? {
            Sender: { Type: 'String', Value: options.sender },
          }
        : {},
    }),
  }
}
