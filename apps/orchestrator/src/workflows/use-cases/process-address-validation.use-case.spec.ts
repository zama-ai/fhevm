import {
  afterEach,
  beforeEach,
  describe,
  expect,
  Mocked,
  test,
  vi,
} from 'vitest'
import { ProcessAddressValidation } from './process-address-validation.use-case.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { back, web3 } from 'messages'
import type { EventProducer } from '#workflows/interfaces/event.producer.js'
import { faker } from '@faker-js/faker'
import { AddressValidationEvents } from '#workflows/entities/address-validation.js'
import { TestBed } from '@suites/unit'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'

describe(ProcessAddressValidation, () => {
  let pubsub: IPubSub<AddressValidationEvents>
  let producer: Mocked<EventProducer>

  beforeEach(async () => {
    pubsub = new PubSub()
    const { unitRef } = await TestBed.solitary(ProcessAddressValidation)
      .mock(PUBSUB)
      .impl(() => pubsub)
      .compile()

    producer = unitRef.get(EVENT_PRODUCER) as unknown as Mocked<EventProducer>
    producer.publish.mockReturnValue(Task.of(void 0))
  })

  afterEach(() => {
    vi.resetAllMocks()
  })

  describe(`when receiving 'back:address:validation:requested' event`, () => {
    let event: Extract<
      back.BackEvent,
      { type: 'back:address:validation:requested' }
    >
    let task: Task<void, AppError>

    beforeEach(() => {
      event = back.addressValidationRequested(
        {
          requestId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      )

      task = pubsub.publish(event)
    })

    test(`publish a 'web3:contract:validation:requested' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'web3:contract:validation:requested',
        }),
      )
    })

    test(`forward the right payload`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            requestId: event.payload.requestId,
            chainId: event.payload.chainId,
            address: event.payload.address,
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

  describe(`when receiving 'web3:contract:validation:success' event`, () => {
    let event: Extract<
      web3.Web3Event,
      { type: 'web3:contract:validation:success' }
    >
    let task: Task<void, AppError>

    beforeEach(() => {
      event = web3.contractValidationSuccess(
        {
          requestId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )
      task = pubsub.publish(event)
    })

    test(`publish a 'back:address:validation:confirmed' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:address:validation:confirmed',
        }),
      )
    })

    test(`forward the right payload`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            requestId: event.payload.requestId,
            chainId: event.payload.chainId,
            address: event.payload.address,
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

  describe(`when receiving 'web3:contract:validation:failure' event`, () => {
    let event: Extract<
      web3.Web3Event,
      { type: 'web3:contract:validation:failure' }
    >
    let task: Task<void, AppError>

    beforeEach(() => {
      event = web3.contractValidationFailure(
        {
          requestId: faker.string.uuid(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
          reason: faker.lorem.paragraph(),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )
      task = pubsub.publish(event)
    })

    test(`publish a 'back:address:validation:failed' event`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:address:validation:failed',
        }),
      )
    })

    test(`forward the right payload`, async () => {
      await task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            requestId: event.payload.requestId,
            chainId: event.payload.chainId,
            address: event.payload.address,
            reason: event.payload.reason,
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
})
