import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { ProcessInputProof } from './process-input-proof.use-case.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { back, relayer } from 'messages'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { faker } from '@faker-js/faker'
import { InputProofEvents } from '#workflows/entities/input-proof.js'
import { TestBed } from '@suites/unit'

describe(ProcessInputProof, () => {
  let pubsub: IPubSub<InputProofEvents>
  let producer: Mocked<EventProducer>

  beforeEach(async () => {
    pubsub = new PubSub()
    const { unitRef } = await TestBed.solitary(ProcessInputProof)
      .mock(PUBSUB)
      .impl(() => pubsub)
      .compile()

    producer = unitRef.get(EVENT_PRODUCER) as unknown as Mocked<EventProducer>
    producer.publish.mockReturnValue(Task.of(void 0))
  })

  describe(`when receviving 'back:httpz:input-proof:requested' event`, () => {
    let task: Task<void, AppError>

    let event: Extract<
      back.BackEvent,
      { type: 'back:httpz:input-proof:requested' }
    >

    beforeEach(() => {
      event = back.httpzInputProofRequested(
        {
          requestId: faker.string.uuid(),
          contractChainId: faker.number.int({ min: 1, max: 100_000 }),
          contractAddress: faker.string.hexadecimal({ length: 40 }),
          userAddress: faker.string.hexadecimal({ length: 40 }),
          ciphertextWithInputVerification: faker.string.hexadecimal({
            length: { min: 50, max: 100 },
            prefix: '',
          }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )

      task = pubsub.publish(event)
    })

    test(`then it publishes a 'relayer:input-registration:input-registration-request' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'relayer:input-registration:input-registration-request',
        }),
      )
    })

    test(`then it forwards the right payload`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            ...event.payload,
            contractChainId: Number(event.payload.contractChainId),
          },
        }),
      )
    })

    test(`forward the right correlationId`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: { correlationId: event.meta.correlationId },
        }),
      )
    })
  })

  describe(`when receiving a 'relayer:input-registration:input-registration-response' event`, () => {
    let task: Task<void, AppError>

    let payload: Extract<
      relayer.RelayerEvent,
      { type: 'relayer:input-registration:input-registration-response' }
    >['payload']
    let correlationId: string

    beforeEach(() => {
      payload = {
        requestId: faker.string.uuid(),
        handles: [faker.string.hexadecimal(), faker.string.hexadecimal()],
        signatures: [faker.string.hexadecimal(), faker.string.hexadecimal()],
      }
      correlationId = faker.string.uuid()

      task = pubsub.publish(
        relayer.inputRegistrationResponse(payload, { correlationId }),
      )
    })

    test(`then it publishes a 'back:httpz:input-proof:completed' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:httpz:input-proof:completed',
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
          meta: { correlationId },
        }),
      )
    })
  })
})
