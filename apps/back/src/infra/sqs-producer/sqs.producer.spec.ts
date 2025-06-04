import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, Mocked, test } from 'vitest'
import { SqsProducer } from './sqs.producer.js'
import { configModule } from '#app.module.js'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { mockClient } from 'aws-sdk-client-mock'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { ConfigService } from '@nestjs/config'
import { TestBed } from '@suites/unit'

const client = mockClient(SQSClient)

function getConfig(key: string): string | undefined {
  console.log(`get config for ${key}`)
  switch (key) {
    case 'aws.endpoint':
      return 'http://localhost:4566'
    case 'aws.region':
      return 'us-east-1'
    case 'aws.accessKeyId':
      return 'test'
    case 'aws.secretAccessKey':
      return 'test'
    case 'aws.back.queueUrl':
      return 'http://localhost:4566/queue/back'
    case 'aws.email.queueUrl':
      return 'http://localhost:4566/queue/email'
    case 'aws.web3.queueUrl':
      return 'http://localhost:4566/queue/web3'
    case 'aws.relayer.queueUrl':
      return 'http://localhost:4566/queue/relayer'
    case 'aws.orchestrator.queueUrl':
      return 'http://localhost:4566/queue/orchestrator'
  }
}

describe('SqsProducer', () => {
  let producer: SqsProducer
  let config: Mocked<ConfigService>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(SqsProducer).compile()

    producer = unit
    config = unitRef.get(ConfigService) as unknown as Mocked<ConfigService>
    config.get.mockImplementation(getConfig)
    config.getOrThrow.mockImplementation(getConfig)
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
          chainId: faker.number.int({ min: 1, max: 100_000 }),
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
