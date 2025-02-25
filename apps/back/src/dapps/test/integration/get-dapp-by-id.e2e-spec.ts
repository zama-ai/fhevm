import { DAppId } from '#dapps/domain/entities/value-objects.js'
import {
  DApp,
  GraphQlResponse,
  IntegrationManager,
} from '#tests/integration.manager.js'
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

describe('get-dapp-by-id', () => {
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

  describe('given a dapp is created', () => {
    let dappId: string
    let token: string
    let teamId: string

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id
      } else {
        console.log(`failed to login: ${JSON.stringify(result)}`)
      }
      expect(result.success, 'Failed to login the user').toBe(true)

      const createDappResult = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
        address: faker.string.hexadecimal({ length: 40 }),
      })
      expect(createDappResult.success).toBe(true)
      if (createDappResult.success) {
        dappId = createDappResult.data.dapp.id
      }
    })

    describe('when a logged in user gets the dapp by id', () => {
      let result: GraphQlResponse<DApp>

      beforeEach(async () => {
        result = await manager.dapp.getDapp({
          token,
          dappId,
        })
      })

      test('then the dapp is returned', () => {
        expect(result.success).toBe(true)
        if (result.success) {
          expect(result.data.id).toBe(dappId)
        }
      })
    })

    describe('when a logged in user gets the dapp by id with an invalid id', () => {
      let result: GraphQlResponse<DApp>

      beforeEach(async () => {
        result = await manager.dapp.getDapp({
          token,
          dappId: DAppId.random().value,
        })
      })

      test('then the dapp is not returned', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors).toBeDefined()
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).toContain('not found')
        }
      })
    })

    describe("when a logged in user gets somebody else's dapp", () => {
      let result: GraphQlResponse<DApp>
      let token2: string

      beforeEach(async () => {
        const loginResult = await manager.auth.login(
          {
            email: faker.internet.email(),
            password: faker.internet.password(),
          },
          { signup: true }, // signup the second user
        )
        expect(loginResult.success, 'Failed to login the user').toBe(true)
        if (loginResult.success) {
          token2 = loginResult.data.token
        }

        result = await manager.dapp.getDapp({
          token: token2,
          dappId,
        })
      })

      test('then the dapp is not returned', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors).toBeDefined()
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).toContain('not found')
        }
      })
    })

    describe('when an anonimous user gets the dapp by id', () => {
      let result: GraphQlResponse<DApp>

      beforeEach(async () => {
        result = await manager.dapp.getDapp({
          token: '',
          dappId,
        })
      })

      test('then the dapp is not returned', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors).toBeDefined()
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).toContain('Unauthorized')
        }
      })
    })
  })
})
