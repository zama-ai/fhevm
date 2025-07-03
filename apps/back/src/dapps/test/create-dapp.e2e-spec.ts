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

describe('create-dapp', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30000)

  beforeEach(async () => {
    await manager.beforeEach()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a user is logged in', () => {
    let token: string
    let teamId: string

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
    })

    describe('when creating a dapp', () => {
      let dapp: DApp | undefined

      beforeEach(async () => {
        const createDApp = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
          chainId: 11155111, // Sepolia
          address: faker.string.hexadecimal({ length: 40 }),
        })
        if (createDApp.success) {
          dapp = createDApp.data
        } else {
          console.log(`failed to create dapp: ${JSON.stringify(createDApp)}`)
          expect(createDApp.success, 'Failed to create dApp').toBe(true)
        }
      })

      test('then it associates the dapp with the right team', () => {
        expect(dapp?.team.id).toBe(teamId)
      })
    })
  })

  describe('given a user is not logged in', () => {
    describe('when creating a dapp', () => {
      let result: GraphQlResponse<DApp>

      beforeEach(async () => {
        result = await manager.dapp.createDApp({
          token: faker.string.uuid(),
          teamId: faker.string.uuid(),
          name: faker.string.alphanumeric(10),
          chainId: 11155111, // Sepolia
          address: faker.string.hexadecimal({ length: 40 }),
        })
      })

      test('then it raises unauthorized error', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors).toBeDefined()
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).toBe('Unauthorized')
        }
      })
    })
  })
})
