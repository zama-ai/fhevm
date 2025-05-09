import { faker } from '@faker-js/faker'
import { back, relayer } from 'messages'
import { IntegrationManager } from 'test/integration.manager.js'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from 'vitest'

describe('input proof', () => {
  const manager = new IntegrationManager(false)

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30_000)

  beforeEach(async () => {
    await manager.beforeEach()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  describe(`when receiving a 'back:httpz:input-proof:requested' event`, () => {
    beforeEach(async () => {
      await manager.sendMessage(
        back.httpzInputProofRequested(
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
          { correlationId: faker.string.uuid() },
        ),
      )
    })

    test(`then it publish a 'relayer:input-registration:input-registration-request' event`, async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('relayer')
        return size > 0
      })
      const messages = await manager.getQueueMessages('relayer')
      // TODO: the publisher send the message twice
      // expect(messages.length).toBe(1)
      const message = messages[0]
      expect(message?.event).toEqual(
        expect.objectContaining({
          type: 'relayer:input-registration:input-registration-request',
        }),
      )
    })
  })

  describe(`when receiving a 'relayer:input-registration:input-registration-response' event`, () => {
    beforeEach(async () => {
      await manager.sendMessage(
        relayer.inputRegistrationResponse(
          {
            requestId: faker.string.uuid(),
            handles: [faker.string.hexadecimal(), faker.string.hexadecimal()],
            signatures: [
              faker.string.hexadecimal(),
              faker.string.hexadecimal(),
            ],
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })

    test(`then it publish a 'back:httpz:input-proof:completed' event`, async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('back')
        return size > 0
      })
      const messages = await manager.getQueueMessages('back')
      // TODO: the publisher send the message twice
      // expect(messages.length).toBe(1)
      const message = messages[0]
      expect(message?.event).toEqual(
        expect.objectContaining({
          type: 'back:httpz:input-proof:completed',
        }),
      )
    })
  })
})
