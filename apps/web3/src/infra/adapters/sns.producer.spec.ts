import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test } from 'vitest'
import { SnsProducer } from './sns.producer.js'
import { MS_NAME, PUBSUB } from '#constants.js'
import { LOCAL_FHEVM_CHAIN_ID, PubSub } from 'utils'
import { configModule } from '#app.module.js'
import { web3 } from 'messages'
import { faker } from '@faker-js/faker'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { mockClient } from 'aws-sdk-client-mock'

const client = mockClient(SNSClient)

describe('SnsProducer', () => {
  let producer: SnsProducer
  let pubsub: PubSub<web3.Web3Event>

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
    let event: web3.Web3Event

    beforeEach(async () => {
      client.on(PublishCommand).resolves({ MessageId: LOCAL_FHEVM_CHAIN_ID })
      event = web3.fheRequested(
        {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )
      await producer.sendMessage(event).toPromise()
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
      client.on(PublishCommand).resolves({ MessageId: LOCAL_FHEVM_CHAIN_ID })
    })

    test.each([
      {
        event: web3.contractValidationRequested(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.contractValidationSuccess(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            owner: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.contractValidationFailure(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.fheRequested(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.fheDetected(
          {
            id: faker.string.alphanumeric(10),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            name: faker.string.alphanumeric(10),
            timestamp: faker.date.past().toISOString(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
    ])('then it processes the $event.type event', async ({ event }) => {
      await pubsub.publish(event).toPromise()
      // if (publish) {
      expect(client).toHaveReceivedCommandWith(PublishCommand, {
        Message: JSON.stringify(event),
      })
      // } else {
      //   expect(client).not.toHaveReceivedAnyCommand()
      // }
    })
  })
})
