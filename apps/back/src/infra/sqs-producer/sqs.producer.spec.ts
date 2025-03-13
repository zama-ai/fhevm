import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test } from 'vitest'
import { SqsProducer } from './sqs.producer.js'
import { MS_NAME, PUBSUB } from '#constants.js'
import { LOCAL_FHEVM_CHAIN_ID, PubSub } from 'utils'
import { configModule } from '#app.module.js'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { mockClient } from 'aws-sdk-client-mock'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'

const client = mockClient(SQSClient)

describe('SqsProducer', () => {
  let producer: SqsProducer
  let pubsub: PubSub<back.BackEvent>

  beforeEach(async () => {
    const moduleRef = await Test.createTestingModule({
      imports: [configModule],
      providers: [SqsProducer, { provide: PUBSUB, useValue: new PubSub() }],
    }).compile()

    producer = moduleRef.get(SqsProducer)
    pubsub = moduleRef.get(PUBSUB)
  })

  afterEach(() => {
    client.reset()
  })

  describe('when publish is called', () => {
    let event: back.BackEvent

    beforeEach(async () => {
      client
        .on(SendMessageCommand)
        .resolves({ MessageId: LOCAL_FHEVM_CHAIN_ID })
      event = back.dappStatsRequested(
        {
          requestId: faker.string.uuid(),
          dAppId: DAppId.random().value,
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )
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

    test('then it publishes the sender attribute', async () => {
      expect(client).toHaveReceivedCommandWith(SendMessageCommand, {
        MessageAttributes: {
          Sender: {
            DataType: 'String',
            StringValue: MS_NAME,
          },
        },
      })
    })
  })

  describe('when pubsub pubslish an event', () => {
    beforeEach(() => {
      client
        .on(SendMessageCommand)
        .resolves({ MessageId: LOCAL_FHEVM_CHAIN_ID })
    })

    test.each([
      {
        event: back.dappStatsRequested(
          {
            requestId: faker.string.uuid(),
            dAppId: DAppId.random().value,
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
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
          {
            correlationId: faker.string.uuid(),
            [`${MS_NAME}-dir`]: 'in',
          },
        ),
      },
    ])('then it processes the $event.type event', async ({ event }) => {
      await pubsub.publish(event).toPromise()
      const publish = event.meta[`${MS_NAME}-dir`] !== 'in'
      if (publish) {
        expect(client).toHaveReceivedCommandWith(SendMessageCommand, {
          MessageBody: JSON.stringify(event),
        })
      } else {
        expect(client).not.toHaveReceivedAnyCommand()
      }
    })
  })
})
