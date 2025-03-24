import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test } from 'vitest'
import { SqsProducer } from './sqs.producer.js'
import { LOCAL_FHEVM_CHAIN_ID } from 'utils'
import { configModule } from '#app.module.js'
import { web3 } from 'messages'
import { faker } from '@faker-js/faker'
import { mockClient } from 'aws-sdk-client-mock'
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
    let event: web3.Web3Event

    beforeEach(async () => {
      client
        .on(SendMessageCommand)
        .resolves({ MessageId: LOCAL_FHEVM_CHAIN_ID })
      event = web3.fheRequested(
        {
          requestId: faker.string.uuid(),
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
      expect(client).toHaveReceivedCommand(SendMessageCommand)
    })

    test('then it publishes the right content', async () => {
      expect(client).toHaveReceivedCommandWith(SendMessageCommand, {
        MessageBody: JSON.stringify(event),
      })
    })
  })
})
