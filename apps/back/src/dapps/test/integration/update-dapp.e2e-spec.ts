import { DAppStatus } from '@/dapps/domain/entities/dapp'
import { IntegrationManager } from '@/tests/integration.manager'
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
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a dapp exists', () => {
    let token: string
    let dapp: DApp

    beforeEach(async () => {
      const result = await manager.createDApp({
        name: faker.string.alphanumeric(10),
      })
      if (result.success) {
        ;({ token, dapp } = result.data)
      }
    })

    describe('when updating the dapp', () => {
      let updated: DApp
      let name: string
      let address: string
      beforeEach(async () => {
        name = faker.string.alphanumeric(10)
        address = faker.string.hexadecimal({ length: 40 })

        const result = await manager.updateDApp({
          token,
          dappId: dapp.id,
          name,
          address,
        })
        expect(result.success, 'Failed to update dapp').toBe(true)
        if (result.success) {
          updated = result.data.dapp
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
        const result = await manager.signup(
          {
            name: faker.string.alphanumeric(10),
            password: faker.internet.password(),
          },
          { createInvitation: true },
        )

        expect(result.success, 'Failed to sign up a new user').toBe(true)
        if (result.success) {
          token2 = result.data.token
        }
      })

      test('then it rejects the update', async () => {
        const result = await manager.updateDApp({
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
