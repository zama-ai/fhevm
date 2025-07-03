import { TestBed } from '@suites/unit'
import { ProcessAuth } from './process-auth.use-case.js'
import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { IPubSub, PubSub, Task } from 'utils'
import { PUBSUB } from '#constants.js'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import {
  EVENT_PRODUCER,
  type EventProducer,
} from '#workflows/interfaces/event.producer.js'
import { AuthEvents } from '#workflows/entities/auth.js'

describe(ProcessAuth, () => {
  let useCase: ProcessAuth
  let pubsub: IPubSub<AuthEvents>
  let producer: Mocked<EventProducer>

  beforeEach(async () => {
    pubsub = new PubSub<AuthEvents>()
    const { unit, unitRef } = await TestBed.solitary(ProcessAuth)
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

  describe("when receiving a 'back:user:created' event", () => {
    let event: Extract<back.BackEvent, { type: 'back:user:created' }>

    beforeEach(async () => {
      event = back.userCreated({
        requestId: faker.string.uuid(),
        userId: faker.string.uuid(),
        email: faker.internet.email(),
        name: faker.person.fullName(),
        token: faker.string.alphanumeric(10),
      })
      await pubsub.publish(event).toPromise()
    })

    test("then it should publish an 'email:user:created' event", () => {
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'email:user:created',
        }),
      )
    })

    test('then it should forward the right payload', () => {
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
  })

  describe("when receiving a 'back:password-reset:requested' event", () => {
    let event: Extract<
      back.BackEvent,
      { type: 'back:password-reset:requested' }
    >

    beforeEach(async () => {
      event = back.passwordResetRequested(
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
  })

  describe("when receiving a 'back:password-reset:completed' event", () => {
    let event: Extract<
      back.BackEvent,
      { type: 'back:password-reset:completed' }
    >

    beforeEach(async () => {
      event = back.passwordResetCompleted(
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
  })
})
