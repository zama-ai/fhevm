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

    describe(`when it receives a 'back:dapp:stats-available' event`, () => {
      test('should forward it to the pubsub', async () => {
        pubsub.publish.mockReturnValue(Task.of(void 0))
        const event = genStatsAvailable()
        const message = encodeMessage(event)

        await consumer.handleMessage(message)
        expect(pubsub.publish).toBeCalledWith(event)
      })

      test('should forward errors', async () => {
        pubsub.publish.mockReturnValue(
          Task.reject(unknownError('Mocked error')),
        )
        const event = genStatsAvailable()
        const message = encodeMessage(event)

        await expect(consumer.handleMessage(message)).rejects.toThrow(
          'Mocked error',
        )
      })
    })

    describe(`when it receives a 'back:dapp:stats-requested' event`, () => {
      test('should drop it', async () => {
        const event = genStatsRequested()
        const message = encodeMessage(event)

        await consumer.handleMessage(message)
        expect(pubsub.publish).not.toBeCalled()
      })
    })
  })
})

function genStatsAvailable() {
  return back.dappStatsAvailable(
    {
      chainId: faker.string.numeric(5),
      address: faker.string.hexadecimal({ length: 40 }),
      name: faker.string.alphanumeric(10),
      timestamp: faker.date.anytime().toISOString(),
      externalRef: faker.string.alphanumeric(10),
    },
    {
      correlationId: faker.string.uuid(),
    },
  )
}

function genStatsRequested() {
  return back.dappStatsRequested(
    {
      chainId: faker.string.numeric(5),
      address: faker.string.hexadecimal({ length: 40 }),
    },
    {
      correlationId: faker.string.uuid(),
    },
  )
}

function encodeMessage(message: object): Message {
  return {
    Body: JSON.stringify({
      Message: JSON.stringify(message),
    }),
  }
}
