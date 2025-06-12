import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { back, relayer } from 'messages'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { faker } from '@faker-js/faker'
import { TestBed } from '@suites/unit'
import { ProcessPublicDecrypt } from './process-public-decrypt.use-case.js'
import { PublicDecryptEvents } from '#workflows/entities/public-decrypt.js'

describe(ProcessPublicDecrypt, () => {
  let pubsub: IPubSub<PublicDecryptEvents>
  let producer: Mocked<EventProducer>

  beforeEach(async () => {
    pubsub = new PubSub()
    const { unitRef } = await TestBed.solitary(ProcessPublicDecrypt)
      .mock(PUBSUB)
      .impl(() => pubsub)
      .compile()

    producer = unitRef.get(EVENT_PRODUCER) as unknown as Mocked<EventProducer>
    producer.publish.mockReturnValue(Task.of(void 0))
  })

  describe(`when receviving 'back:httpz:public-decrypt:requested' event`, () => {
    let task: Task<void, AppError>

    let event: Extract<
      back.BackEvent,
      { type: 'back:httpz:public-decrypt:requested' }
    >

    beforeEach(() => {
      event = back.httpzPublicDecryptRequested(
        {
          requestId: faker.string.uuid(),
          ciphertextHandles: [faker.string.hexadecimal({ length: 64 })],
        },
        {
          correlationId: faker.string.uuid(),
        },
      )

      task = pubsub.publish(event)
    })

    test(`then it publishes a 'relayer:public-decryption:operation-request' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'relayer:public-decryption:operation-request',
        }),
      )
    })

    test(`then it forwards the right payload`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: event.payload,
        }),
      )
    })
  })

  describe(`when receiving a 'relayer:public-decryption:operation-response' event`, () => {
    let task: Task<void, AppError>

    let payload: Extract<
      relayer.RelayerEvent,
      { type: 'relayer:public-decryption:operation-response' }
    >['payload']

    beforeEach(() => {
      payload = {
        requestId: faker.string.uuid(),
        response: [
          {
            decryptedValue: faker.string.hexadecimal(),
            signatures: [faker.string.hexadecimal()],
          },
        ],
      }

      task = pubsub.publish(relayer.publicDecryptionOperationResponse(payload))
    })

    test(`then it publishes a 'back:httpz:public-decrypt:completed' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:httpz:public-decrypt:completed',
        }),
      )
    })

    test(`then it forwards the right payload`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload,
        }),
      )
    })

    test(`forward the right correlationId`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: { correlationId: payload.requestId },
        }),
      )
    })
  })
})
