import {
  afterEach,
  beforeEach,
  describe,
  expect,
  MockInstance,
  test,
  vi,
} from 'vitest'
import { ProcessInputProof } from './process-input-proof.use-case.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { back, relayer } from 'messages'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Test } from '@nestjs/testing'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { faker } from '@faker-js/faker'

describe(ProcessInputProof, () => {
  let pubsub: IPubSub<back.BackEvent | relayer.RelayerEvent>
  let producer: EventProducer

  beforeEach(async () => {
    pubsub = new PubSub()
    producer = { publish: () => Task.of(void 0) }

    await Test.createTestingModule({
      providers: [
        {
          provide: PUBSUB,
          useValue: pubsub,
        },
        {
          provide: EVENT_PRODUCER,
          useValue: producer,
        },
        ProcessInputProof,
      ],
    }).compile()
  })

  afterEach(() => {
    vi.resetAllMocks()
  })

  describe(`when receviving 'back:httpz:input-proof:requested' event`, () => {
    let spy: MockInstance<
      (event: back.BackEvent | relayer.RelayerEvent) => Task<void, AppError>
    >
    let task: Task<void, AppError>

    let payload: Extract<
      back.BackEvent,
      { type: 'back:httpz:input-proof:requested' }
    >['payload']
    let correlationId: string

    beforeEach(() => {
      spy = vi.spyOn(producer, 'publish')
      payload = {
        requestId: faker.string.uuid(),
        contractChainId: faker.string.numeric(),
        contractAddress: faker.string.hexadecimal({ length: 40 }),
        userAddress: faker.string.hexadecimal({ length: 40 }),
        ciphertextWithInputVerification: faker.string.hexadecimal({
          length: { min: 50, max: 100 },
          prefix: '',
        }),
      }
      correlationId = faker.string.uuid()

      task = pubsub.publish(
        back.httpzInputProofRequested(payload, { correlationId }),
      )
    })

    test(`then it publishes a 'relayer:input-registration:input-registration-request' event`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'relayer:input-registration:input-registration-request',
        }),
      )
    })

    test(`then it forwards the right payload`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            ...payload,
            contractChainId: Number(payload.contractChainId),
          },
        }),
      )
    })

    test(`forward the right correlationId`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: { correlationId },
        }),
      )
    })
  })

  describe(`when receiving a 'relayer:input-registration:input-registration-response' event`, () => {
    let spy: MockInstance<
      (event: back.BackEvent | relayer.RelayerEvent) => Task<void, AppError>
    >
    let task: Task<void, AppError>

    let payload: Extract<
      relayer.RelayerEvent,
      { type: 'relayer:input-registration:input-registration-response' }
    >['payload']
    let correlationId: string

    beforeEach(() => {
      spy = vi.spyOn(producer, 'publish')
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
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:httpz:input-proof:completed',
        }),
      )
    })

    test(`then it forwards the right payload`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload,
        }),
      )
    })

    test(`forward the right correlationId`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: { correlationId },
        }),
      )
    })
  })
})
