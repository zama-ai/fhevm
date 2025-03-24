import {
  afterEach,
  beforeEach,
  describe,
  expect,
  MockInstance,
  test,
  vi,
} from 'vitest'
import { ProcessAddressValidation } from './process-address-validation.use-case.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { back, web3 } from 'messages'
import type { EventProducer } from '#workflows/interfaces/event.producer.js'
import { faker } from '@faker-js/faker'

describe(ProcessAddressValidation, () => {
  // let useCase: ProcessAddressValidation
  let pubsub: IPubSub<back.BackEvent | web3.Web3Event>
  let producer: EventProducer

  beforeEach(() => {
    pubsub = new PubSub()
    producer = { publish: () => Task.of(void 0) }
    new ProcessAddressValidation(pubsub, producer)
  })

  afterEach(() => {
    vi.resetAllMocks()
  })

  describe(`when receiving 'back:address:validation:requested' event`, () => {
    let spy: MockInstance<
      (event: back.BackEvent | web3.Web3Event) => Task<void, AppError>
    >
    let requestId: string
    let chainId: string
    let address: string
    let correlationId: string
    let task: Task<void, AppError>

    beforeEach(() => {
      spy = vi.spyOn(producer, 'publish')
      requestId = faker.string.uuid()
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      correlationId = faker.string.uuid()
      task = pubsub.publish(
        back.addressValidationRequested(
          { requestId, chainId, address },
          { correlationId },
        ),
      )
    })

    test(`publish a 'web3:contract:validation:requested' event`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'web3:contract:validation:requested',
        }),
      )
    })

    test(`forward the right payload`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: { requestId, chainId, address },
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

  describe(`when receiving 'web3:contract:validation:success' event`, () => {
    let spy: MockInstance<
      (event: back.BackEvent | web3.Web3Event) => Task<void, AppError>
    >
    let requestId: string
    let chainId: string
    let address: string
    let correlationId: string
    let task: Task<void, AppError>

    beforeEach(() => {
      spy = vi.spyOn(producer, 'publish')
      requestId = faker.string.uuid()
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      correlationId = faker.string.uuid()
      task = pubsub.publish(
        web3.contractValidationSuccess(
          { requestId, chainId, address },
          { correlationId },
        ),
      )
    })

    test(`publish a 'back:address:validation:confirmed' event`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:address:validation:confirmed',
        }),
      )
    })

    test(`forward the right payload`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: { requestId, chainId, address },
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

  describe(`when receiving 'web3:contract:validation:failure' event`, () => {
    let spy: MockInstance<
      (event: back.BackEvent | web3.Web3Event) => Task<void, AppError>
    >
    let requestId: string
    let chainId: string
    let address: string
    let reason: string
    let correlationId: string
    let task: Task<void, AppError>

    beforeEach(() => {
      spy = vi.spyOn(producer, 'publish')
      requestId = faker.string.uuid()
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      reason = faker.lorem.paragraph()
      correlationId = faker.string.uuid()
      task = pubsub.publish(
        web3.contractValidationFailure(
          { requestId, chainId, address, reason },
          { correlationId },
        ),
      )
    })

    test(`publish a 'back:address:validation:failed' event`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:address:validation:failed',
        }),
      )
    })

    test(`forward the right payload`, async () => {
      await task.toPromise()
      expect(spy).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: { requestId, chainId, address, reason },
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
