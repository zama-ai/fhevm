import { DAppStatus } from '#dapps/domain/entities/dapp.js'
import { IntegrationManager } from '#tests/integration.manager.js'
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

interface DApp {
  id: string
  name: string
  address: string | null
  status: DAppStatus
  team: {
    id: string
    name: string
  }
}

describe('update-dapp', () => {
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

  describe('given a user is logged in and a dapp exists', () => {
    let token: string
    let teamId: string
    let dapp: DApp

    beforeEach(async () => {
      const login = await manager.auth.login(
        {
          email: faker.internet.email(),
          password: faker.internet.password(),
        },
        { signup: true },
      )
      if (login.success) {
        token = login.data.token
        teamId = login.data.user.teams[0].id
        const createDapp = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
        })
        if (createDapp.success) {
          dapp = createDapp.data
        } else {
          console.log(`createDapp: ${JSON.stringify(createDapp)}`)
          expect(createDapp.success, 'Failed to create dapp').toBe(true)
        }
      } else {
        console.log(`login: ${JSON.stringify(login)}`)
        expect(login.success, 'Failed to login the user').toBe(true)
      }
    })

    describe('when updating the dapp', () => {
      let updated: DApp
      let name: string
      let address: string
      beforeEach(async () => {
        name = faker.string.alphanumeric(10)
        address = faker.string.hexadecimal({ length: 40 })

        const updateDApp = await manager.dapp.updateDApp({
          token,
          dappId: dapp.id,
          name,
          address,
        })
        if (updateDApp.success) {
          updated = updateDApp.data
        } else {
          console.log(`updateDApp: ${JSON.stringify(updateDApp)}`)
          expect(updateDApp.success, 'Failed to update dapp').toBe(true)
        }
      })

      test('then the dapp address is updated', () => {
        expect(updated, 'dApp should be updated').toBeDefined()
        expect(updated.address).toBe(address)
      })

      test('then the dapp name is updated', () => {
        expect(updated, 'dApp should be updated').toBeDefined()
        expect(updated.name).toBe(name)
      })
    })

    describe('when updating somebody else dapp', () => {
      let token2: string

      beforeEach(async () => {
        const result = await manager.auth.login(
          {
            email: faker.internet.email(),
            password: faker.internet.password(),
          },
          { signup: true },
        )

        expect(result.success, 'Failed to sign up a new user').toBe(true)
        if (result.success) {
          token2 = result.data.token
        }
      })

      test('then it rejects the update', async () => {
        const result = await manager.dapp.updateDApp({
          token: token2,
          dappId: dapp.id,
          name: faker.string.alphanumeric(10),
          address: faker.string.hexadecimal({ length: 40 }),
        })

        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors[0].message).toContain('Forbidden')
        }
      })
    })
  })
})
