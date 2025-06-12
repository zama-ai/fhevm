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

describe('public decrypt', () => {
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

  describe(`when receiving a 'back:httpz:public-decrypt:requested' event`, () => {
    beforeEach(async () => {
      await manager.sendMessage(
        back.httpzPublicDecryptRequested(
          {
            requestId: faker.string.uuid(),
            ciphertextHandles: [faker.string.hexadecimal({ length: 64 })],
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })

    test(`then it publish a 'relayer:public-decryption:operation-request' event`, async () => {
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
          type: 'relayer:public-decryption:operation-request',
        }),
      )
    })
  })

  describe(`when receiving a 'relayer:public-decryption:operation-response' event`, () => {
    beforeEach(async () => {
      await manager.sendMessage(
        relayer.publicDecryptionOperationResponse({
          requestId: faker.string.uuid(),
          response: [
            {
              decryptedValue: faker.string.hexadecimal(),
              signatures: [
                faker.string.hexadecimal(),
                faker.string.hexadecimal(),
              ],
            },
          ],
        }),
      )
    })

    test(`then it publish a 'back:httpz:public-decrypt:completed' event`, async () => {
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
          type: 'back:httpz:public-decrypt:completed',
        }),
      )
    })
  })
})
