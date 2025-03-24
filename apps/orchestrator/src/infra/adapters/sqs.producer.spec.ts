import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test } from 'vitest'
import { configModule } from '#app.module.js'
import { back, web3 } from 'messages'
import { faker } from '@faker-js/faker'
import { mockClient } from 'aws-sdk-client-mock'
import { SQSProducer } from './sqs.producer.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'

const client = mockClient(SQSClient)

describe('SqsProducer', () => {
  let producer: SQSProducer

  beforeEach(async () => {
    const moduleRef = await Test.createTestingModule({
      imports: [configModule],
      providers: [SQSProducer],
    }).compile()

    producer = moduleRef.get(SQSProducer)
  })

  afterEach(() => {
    client.reset()
  })

  describe.each([
    {
      event: back.addressValidationRequested(
        {
          requestId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ),
      queue: 'back',
    },
    {
      event: back.dappStatsRequested(
        {
          requestId: faker.string.uuid(),
          dAppId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      ),
      queue: 'back',
    },
    {
      event: back.dappStatsRequested(
        {
          requestId: faker.string.uuid(),
          dAppId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      ),
      queue: 'back',
    },
    {
      event: back.dappStatsAvailable(
        {
          requestId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(10),
          timestamp: faker.date.past().toISOString(),
          externalRef: faker.string.alphanumeric(10),
        },
        { correlationId: faker.string.uuid() },
      ),
      queue: 'back',
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
      queue: 'web3',
    },
    {
      event: web3.fheDetected(
        {
          requestId: faker.string.uuid(),
          id: faker.string.alphanumeric(10),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(10),
          timestamp: faker.date.past().toISOString(),
        },
        { correlationId: faker.string.uuid() },
      ),
      queue: 'web3',
    },
  ])('when publish is called with an $event.type event', ({ event, queue }) => {
    beforeEach(async () => {
      client.on(SendMessageCommand).resolves({ MessageId: faker.string.uuid() })

      await producer.publish(event).toPromise()
    })

    test('then it publishes a message successfully', () => {
      expect(client).toHaveReceivedCommand(SendMessageCommand)
    })

    test('then it publishes the right content', async () => {
      expect(client).toHaveReceivedCommandWith(SendMessageCommand, {
        MessageBody: JSON.stringify(event),
      })
    })

    test('then it publishes to the right queue', async () => {
      expect(client).toHaveReceivedCommandWith(SendMessageCommand, {
        QueueUrl: `http://localhost:4566/000000000000/${queue}-queue`,
      })
    })
  })
})
