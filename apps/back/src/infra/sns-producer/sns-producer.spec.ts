import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test } from 'vitest'
import { SnsProducer } from './sns-producer.js'
import { MS_NAME, PUBSUB } from '#constants.js'
import { PubSub } from 'utils'
import { configModule } from '#app.module.js'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { mockClient } from 'aws-sdk-client-mock'

const client = mockClient(SNSClient)

describe('SnsProducer', () => {
  let producer: SnsProducer
  let pubsub: PubSub<back.BackEvent>

  beforeEach(async () => {
    const moduleRef = await Test.createTestingModule({
      imports: [configModule],
      providers: [SnsProducer, { provide: PUBSUB, useValue: new PubSub() }],
    }).compile()

    producer = moduleRef.get(SnsProducer)
    pubsub = moduleRef.get(PUBSUB)
  })

  afterEach(() => {
    client.reset()
  })

  describe('when publish is called', () => {
    let event: back.BackEvent

    beforeEach(async () => {
      client.on(PublishCommand).resolves({ MessageId: '12345' })
      event = back.dappStatsRequested(
        {
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
      expect(client).toHaveReceivedCommand(PublishCommand)
    })

    test('then it publishes the right content', async () => {
      expect(client).toHaveReceivedCommandWith(PublishCommand, {
        Message: JSON.stringify(event),
      })
    })

    test('then it publishes the sender attribute', async () => {
      expect(client).toHaveReceivedCommandWith(PublishCommand, {
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
      client.on(PublishCommand).resolves({ MessageId: '12345' })
    })

    test.each([
      {
        event: back.dappStatsRequested(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
        publish: true,
      },
      {
        event: back.dappStatsAvailable(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            name: faker.string.alphanumeric(10),
            timestamp: faker.date.past().toISOString(),
            externalRef: faker.string.alphanumeric(10),
          },
          { correlationId: faker.string.uuid() },
        ),
        publish: false,
      },
    ])(
      'then it processes the $event.type event',
      async ({ event, publish }) => {
        await pubsub.publish(event).toPromise()
        if (publish) {
          expect(client).toHaveReceivedCommandWith(PublishCommand, {
            Message: JSON.stringify(event),
          })
        } else {
          expect(client).not.toHaveReceivedAnyCommand()
        }
      },
    )
  })
})
