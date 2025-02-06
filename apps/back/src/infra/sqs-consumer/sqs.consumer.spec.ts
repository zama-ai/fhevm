import { Test } from '@nestjs/testing'
import { beforeEach, describe, expect, test } from 'vitest'
import { SQSConsumer } from './sqs.consumer.js'
import { PUBSUB } from '#constants.js'
import { mock, MockProxy } from 'vitest-mock-extended'
import { Task, unknownError, type IPubSub } from 'utils'
import { back } from 'messages'
import { AppDeploymentRequested } from '#dapps/use-cases/app-deployment-requested.use-case.js'
import { AppDeploymentEnded } from '#dapps/use-cases/app-deployment-ended.use-case.js'
import { ScDiscovered } from './use-cases/sc-discovered.use-case.js'
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
          {
            provide: AppDeploymentRequested,
            useValue: mock<AppDeploymentRequested>(),
          },
          {
            provide: AppDeploymentEnded,
            useValue: mock<AppDeploymentEnded>,
          },
          {
            provide: ScDiscovered,
            useValue: mock<ScDiscovered>(),
          },
        ],
      }).compile()

      consumer = moduleRef.get(SQSConsumer)
    })

    describe.each([
      ['back', false],
      ['orch', true],
      ['web3', true],
    ])(
      'when it receives a message from the %s microservice',
      (sender, forward) => {
        let event: back.BackEvent
        beforeEach(() => {
          event = {
            type: 'back:dapp:stats-requested',
            payload: {
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
            expect(pubsub.publish).toBeCalledWith(event)
          } else {
            expect(pubsub.publish).not.toBeCalled()
          }
        })

        if (forward) {
          test('should forward errors', async () => {
            pubsub.publish.mockReturnValue(
              Task.reject(unknownError('Mocked error')),
            )
            const message = encodeMessage(event)

            await expect(consumer.handleMessage(message)).rejects.toThrow(
              'Mocked error',
            )
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
