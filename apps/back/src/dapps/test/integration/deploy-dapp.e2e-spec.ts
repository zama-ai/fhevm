import { DApp, IntegrationManager } from '@/tests/integration.manager'
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
import { DAppStatus } from '@/dapps/domain/entities/dapp'

describe('deploy-dapp', () => {
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

  describe('given a dapp is created', () => {
    let token: string
    let teamId: string
    let dappId: string

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      expect(result.success, 'Failed to login the user').toBe(true)
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id

        const dappResult = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
          address: faker.string.hexadecimal({ length: 40 }),
        })
        expect(dappResult.success).toBe(true)
        if (dappResult.success) {
          dappId = dappResult.data.dapp.id
        }
      }
    })

    describe('when deploying a dapp', () => {
      let status: DAppStatus

      beforeEach(async () => {
        const result = await manager.dapp.deployDApp({
          token,
          dappId,
        })
        expect(result.success).toBe(true)
        if (result.success) {
          status = result.data.dapp.status
        }
      })

      test('then the dapp status is deploying', () => {
        expect(status).toBe('DEPLOYING')
      })
    })
  })
})
