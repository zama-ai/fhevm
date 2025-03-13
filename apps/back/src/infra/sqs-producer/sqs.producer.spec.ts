import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test } from 'vitest'
import { SqsProducer } from './sqs.producer.js'
import { configModule } from '#app.module.js'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { mockClient } from 'aws-sdk-client-mock'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'

const client = mockClient(SQSClient)

describe('SqsProducer', () => {
  let producer: SqsProducer

  beforeEach(async () => {
    const moduleRef = await Test.createTestingModule({
      imports: [configModule],
      providers: [SqsProducer],
    }).compile()

    producer = moduleRef.get(SqsProducer)
  })

  afterEach(() => {
    client.reset()
  })

  describe('when publish is called', () => {
    let event: back.BackEvent

    beforeEach(async () => {
      client.on(SendMessageCommand).resolves({ MessageId: faker.string.uuid() })
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
  })
})
