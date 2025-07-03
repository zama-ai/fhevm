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
  describe('when API_KEYS flag is enabled', () => {
    const manager = new IntegrationManager({
      apiKeys: true,
    })

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

      describe('when they request for an input proof', () => {
        let contractChainId: string
        let userAddress: string
        let ciphertextWithInputVerification: string
        let handles: string[]
        let signatures: string[]

        beforeEach(() => {
          contractChainId = `0x${chainId.toString(16)}`
          userAddress = faker.string.hexadecimal({ length: 40 })
          ciphertextWithInputVerification = faker.string.hexadecimal({
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
                contractAddress: address,
                userAddress,
                ciphertextWithInputVerification,
              })
              .set('Content-Type', 'application/json')
              .set('Accept', 'application/json')
              .set('x-api-key', apiKey)
              .end((err, res) => {
                if (err) {
                  console.error(`failed to send /input-proof request: ${err}`)
                  return reject(err)
                }
                return resolve(res)
              })
          })

          await vi.waitUntil(async () => {
            const event = await manager.getMessageFromOrchQueue(
              'back:httpz:input-proof:requested',
            )
            return event !== undefined
          })
          const event = await manager.getMessageFromOrchQueue(
            'back:httpz:input-proof:requested',
          )
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

  describe('when API_KEYS flag is disabled', () => {
    const manager = new IntegrationManager({
      apiKeys: false,
    })

    beforeAll(async () => {
      await manager.beforeAll()
    }, 30000)

    afterAll(async () => {
      await manager.afterAll()
    })

    afterEach(async () => {
      await manager.afterEach()
    })

    describe('given an anonymous user', () => {
      describe('when they request for an input proof', () => {
        let contractChainId: string
        let contractAddress: string
        let userAddress: string
        let ciphertextWithInputVerification: string
        let handles: string[]
        let signatures: string[]

        beforeEach(() => {
          contractChainId = `0x${faker.number
            .int({ min: 1, max: 100_000 })
            .toString(16)}`
          contractAddress = faker.string.hexadecimal({ length: 40 })
          userAddress = faker.string.hexadecimal({ length: 40 })
          ciphertextWithInputVerification = faker.string.hexadecimal({
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
                ciphertextWithInputVerification,
              })
              .set('Content-Type', 'application/json')
              .set('Accept', 'application/json')
              .end((err, res) => {
                if (err) {
                  console.error(`failed to send /input-proof request: ${err}`)
                  return reject(err)
                }
                return resolve(res)
              })
          })

          await vi.waitUntil(async () => {
            const event = await manager.getMessageFromOrchQueue(
              'back:httpz:input-proof:requested',
            )
            return event !== undefined
          })
          const event = await manager.getMessageFromOrchQueue(
            'back:httpz:input-proof:requested',
          )
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
})
