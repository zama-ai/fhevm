import { IntegrationManager } from '#tests/integration.manager.js'
import { faker } from '@faker-js/faker'
import { back } from 'messages'
import request, { Response } from 'supertest'
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
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30000)

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a user has a valid API key', () => {
    // NOTE: We don't have API Keys implemented yet
    describe('when they request for an input proof', () => {
      let contractChainId: string
      let contractAddress: string
      let userAddress: string
      let ciphertextWithZkpok: string
      let handles: string[]
      let signatures: string[]

      beforeEach(() => {
        contractChainId = faker.string.numeric(5)
        contractAddress = faker.string.hexadecimal({ length: 40 })
        userAddress = faker.string.hexadecimal({ length: 40 })
        ciphertextWithZkpok = faker.string.hexadecimal({
          length: { min: 40, max: 100 },
          prefix: '',
        })
        handles = [faker.string.hexadecimal({ length: 40, prefix: '' })]
        signatures = [faker.string.hexadecimal({ length: 40 })]
      })

      test('then the server responde successfully', async () => {
        const promise = new Promise<Response>((resolve, reject) => {
          request(manager.httpServer)
            .post('/v1/input-proof')
            .send({
              contractChainId,
              contractAddress,
              userAddress,
              ciphertextWithZkpok,
            })
            .set('Content-Type', 'application/json')
            .set('Accept', 'application/json')
            .end((err, res) => {
              if (err) {
                console.error(`err: ${err}`)
                return reject(err)
              }
              return resolve(res)
            })
        })

        await vi.waitUntil(async () => {
          const size = await manager.getOrchQueueSize()
          return size > 0
        })
        const message = await manager.getMessageFromOrchQueue()
        const event = JSON.parse(message!)
        if (!back.isBackEvent(event)) {
          expect(false, 'event is not a BackEvent').toBeTruthy()
          return
        }
        const requestId = event.payload.requestId

        await manager.sendMessage(
          back.httpzInputProofCompleted(
            { requestId, handles, signatures },
            { correlationId: faker.string.uuid() },
          ),
        )

        const response = await promise
        expect(response.status).toBe(201)
        expect(response.body).toEqual({ response: { handles, signatures } })
      })
    })
  })
})
