import { TestBed } from '@suites/unit'
import { ProcessPasswordReset } from './process-password-reset.use-case.js'
import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { IPubSub, PubSub, Task } from 'utils'
import { PasswordResetEvents } from '#workflows/entities/password-reset.js'
import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { EventProducer } from '#workflows/interfaces/event.producer.js'

describe(ProcessPasswordReset, () => {
  let useCase: ProcessPasswordReset
  let pubsub: IPubSub<PasswordResetEvents>
  let producer: Mocked<EventProducer>

  beforeEach(async () => {
    pubsub = new PubSub<PasswordResetEvents>()
    const { unit, unitRef } = await TestBed.solitary(ProcessPasswordReset)
      .mock(PUBSUB)
      .impl(() => pubsub)
      .compile()

    useCase = unit
    producer = unitRef.get(EVENT_PRODUCER) as unknown as Mocked<EventProducer>
    producer.publish.mockReturnValue(Task.of(void 0))
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe("when receiving a 'back:user:password-reset:requested' event", () => {
    let event: Extract<
      back.BackEvent,
      { type: 'back:user:password-reset:requested' }
    >

    beforeEach(async () => {
      event = back.userPasswordResetRequested(
        {
          requestId: faker.string.uuid(),
          email: faker.internet.email(),
          token: faker.string.alphanumeric(10),
        },
        { correlationId: faker.string.uuid() },
      )
      await pubsub.publish(event).toPromise()
    })

    test("then it should publish an 'email:password-reset:requested' event", () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'email:password-reset:requested',
        }),
      )
    })

    test('then it should foward the right payload', () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            email: event.payload.email,
            requestId: event.payload.requestId,
            token: event.payload.token,
          },
        }),
      )
    })

    test('then it shoud forward the correlation id', () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: expect.objectContaining({
            correlationId: event.meta.correlationId,
          }),
        }),
      )
    })
  })

  describe("when receiving a 'back:user:password-reset:completed' event", () => {
    let event: Extract<
      back.BackEvent,
      { type: 'back:user:password-reset:completed' }
    >

    beforeEach(async () => {
      event = back.userPasswordResetCompleted(
        {
          requestId: faker.string.uuid(),
          email: faker.internet.email(),
        },
        { correlationId: faker.string.uuid() },
      )
      await pubsub.publish(event).toPromise()
    })

    test("then it should publish an 'email:password-reset:completed' event", () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'email:password-reset:completed',
        }),
      )
    })

    test('then it should foward the right payload', () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          payload: {
            email: event.payload.email,
            requestId: event.payload.requestId,
          },
        }),
      )
    })

    test('then it shoud forward the correlation id', () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          meta: expect.objectContaining({
            correlationId: event.meta.correlationId,
          }),
        }),
      )
    })
  })
})
