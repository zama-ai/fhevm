import { CreateApiKeyResult } from '#tests/httpz.manager.js'
import { IntegrationManager } from '#tests/integration.manager.js'
import { GraphQlResponse } from '#tests/setup.manager.js'
import { faker } from '@faker-js/faker'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
} from 'vitest'

describe('create-api-key', () => {
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

  describe('given a dapp exists', () => {
    let token: string
    let teamId: string
    let dappId: string

    beforeEach(async () => {
      const login = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      if (login.success) {
        token = login.data.token
        teamId = login.data.user.teams[0].id
      } else {
        console.log(`login failed: ${JSON.stringify(login.errors)}`)
        expect(login.success, 'Failed to login the user').toBe(true)
      }

      const dappResult = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
      })
      if (dappResult.success) {
        dappId = dappResult.data.id
      } else {
        console.log(`dappResult failed: ${JSON.stringify(dappResult.errors)}`)
        expect(dappResult.success, 'Failed to create dApp').toBe(true)
      }
    })

    describe('when creating an API key', () => {
      let createApiKey: GraphQlResponse<CreateApiKeyResult>

      beforeEach(async () => {
        createApiKey = await manager.httpz.createApiKey({
          token,
          dappId,
        })
      })

      test('then the API key is created', async () => {
        if (createApiKey.success) {
          const apiKey = await manager.httpz.getApiKey({
            token,
            id: createApiKey.data.apiKey.id,
          })
          if (apiKey.success) {
            expect(apiKey.success).toBe(true)
            expect(apiKey.data.id).toBe(createApiKey.data.apiKey.id)
            expect(apiKey.data.dappId).toBe(dappId)
            expect(apiKey.data.name).toBe(createApiKey.data.apiKey.name)
          } else {
            console.log(`getApiKey failed: ${JSON.stringify(apiKey.errors)}`)
            expect(apiKey.success).toBe(true)
          }
        } else {
          console.log(
            `failed to create api key: ${JSON.stringify(createApiKey.errors)}`,
          )
          expect(createApiKey.success).toBe(true)
        }
      })
    })
  })
})
