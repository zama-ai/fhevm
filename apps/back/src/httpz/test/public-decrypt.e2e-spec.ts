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

describe('public decrypt', () => {
  describe('when API_KEYS flag is enabled', () => {
    const manager = new IntegrationManager({ apiKeys: true })

    beforeAll(async () => {
      await manager.beforeAll()
    }, 30_000)

    afterAll(async () => {
      await manager.afterAll()
    })

    afterEach(async () => {
      await manager.afterEach()
    })

    describe('given a user has a valid API key', () => {
      let apiKey: string
      let dappId: string
      const chainId = 11155111 // Sepolia
      let address: string

      beforeEach(async () => {
        address = faker.string.hexadecimal({ length: 40 })

        const login = await manager.auth.login(
          {
            email: faker.internet.email(),
            password: faker.internet.password(),
          },
          {
            signup: true,
            confirm: true,
          },
        )
        let token = ''
        let teamId = ''
        if (login.success) {
          token = login.data.token
          teamId = login.data.user.teams[0].id
        } else {
          console.log(`failed to login: ${JSON.stringify(login)}`)
          expect(login.success).toBe(true)
        }

        const createDapp = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
          chainId,
          address,
        })
        if (createDapp.success) {
          dappId = createDapp.data.id
        } else {
          console.log(`failed to create dapp: ${JSON.stringify(createDapp)}`)
          expect(createDapp.success).toBe(true)
        }
        const createApiKey = await manager.httpz.createApiKey({
          token,
          dappId,
        })
        if (createApiKey.success) {
          apiKey = createApiKey.data.token
        } else {
          console.log(
            `failed to create api key: ${JSON.stringify(createApiKey)}`,
          )
          expect(createApiKey.success).toBe(true)
        }
      })

      describe('when they request for a public decrypt', () => {
        let ciphertextHandles: string[]
        let decryptedValue: string
        let signatures: string[]

        beforeEach(() => {
          ciphertextHandles = [faker.string.hexadecimal({ length: 64 })]
          decryptedValue = faker.string.hexadecimal({ length: 64 })
          signatures = [faker.string.hexadecimal({ length: 64 })]
        })

        test('then the server responde successfully', async () => {
          const promise = new Promise<Response>((resolve, reject) => {
            request(manager.httpServer)
              .post('/v1/public-decrypt')
              .send({
                ciphertextHandles,
              })
              .set('Content-Type', 'application/json')
              .set('Accept', 'application/json')
              .set('x-api-key', apiKey)
              .end((err, res) => {
                if (err) {
                  console.error(
                    `failed to send /public-decrypt request: ${err}`,
                  )
                  return reject(err)
                }
                return resolve(res)
              })
          })

          await vi.waitUntil(async () => {
            const event = await manager.getMessageFromOrchQueue(
              'back:httpz:public-decrypt:requested',
            )
            return event !== undefined
          })
          const event = await manager.getMessageFromOrchQueue(
            'back:httpz:public-decrypt:requested',
          )
          if (!back.isBackEvent(event)) {
            expect(false, 'event is not a BackEvent').toBeTruthy()
            return
          }
          const requestId = event.payload.requestId

          await manager.sendMessage(
            back.httpzPublicDecryptCompleted(
              { requestId, response: [{ decryptedValue, signatures }] },
              { correlationId: faker.string.uuid() },
            ),
          )

          const httpResponse = await promise
          expect(httpResponse.status).toBe(201)
          expect(httpResponse.body).toEqual({
            response: [
              {
                decrypted_value: decryptedValue,
                signatures,
              },
            ],
          })
        }, 60_000)
      })
    })
  })

  describe('when API_KEYS flag is disabled', () => {
    const manager = new IntegrationManager({ apiKeys: false })

    beforeAll(async () => {
      await manager.beforeAll()
    }, 30_000)

    afterAll(async () => {
      await manager.afterAll()
    })

    afterEach(async () => {
      await manager.afterEach()
    })

    describe('given an anonymous user', () => {
      describe('when they request for an input proof', () => {
        let ciphertextHandles: string[]
        let decryptedValue: string
        let signatures: string[]

        beforeEach(() => {
          ciphertextHandles = [faker.string.hexadecimal({ length: 64 })]
          decryptedValue = faker.string.hexadecimal({ length: 64 })
          signatures = [faker.string.hexadecimal({ length: 64 })]
        })

        test('then the server responde successfully', async () => {
          const promise = new Promise<Response>((resolve, reject) => {
            request(manager.httpServer)
              .post('/v1/public-decrypt')
              .send({ ciphertextHandles })
              .set('Content-Type', 'application/json')
              .set('Accept', 'application/json')
              .end((err, res) => {
                if (err) {
                  console.error(
                    `failed to send /public-decrypt request: ${err}`,
                  )
                  return reject(err)
                }
                return resolve(res)
              })
          })

          await vi.waitUntil(async () => {
            const event = await manager.getMessageFromOrchQueue(
              'back:httpz:public-decrypt:requested',
            )
            return event !== undefined
          })
          const event = await manager.getMessageFromOrchQueue(
            'back:httpz:public-decrypt:requested',
          )
          if (!back.isBackEvent(event)) {
            expect(false, 'event is not a BackEvent').toBeTruthy()
            return
          }
          const requestId = event.payload.requestId

          const responseMessage = back.httpzPublicDecryptCompleted(
            { requestId, response: [{ decryptedValue, signatures }] },
            { correlationId: faker.string.uuid() },
          )
          await manager.sendMessage(responseMessage)

          const httpResponse = await promise
          expect(httpResponse.status).toBe(201)
          expect(httpResponse.body).toEqual({
            response: [
              {
                decrypted_value: decryptedValue,
                signatures,
              },
            ],
          })
        }, 60_000)
      })
    })
  })
})
