import { IntegrationManager, User } from '@/tests/integration.manager'
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

describe('fetch all dapps', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
    // adding some time to debug
    await new Promise(r => setTimeout(r, 200))
  })

  describe('given a user has many dapps', () => {
    let token: string
    let teamId: string
    let dappIds: string[]

    beforeEach(async () => {
      const resp = await manager.auth.login(
        {
          email: faker.internet.email(),
          password: faker.internet.password(),
        },
        { signup: true },
      )
      expect(resp.success).toBe(true)
      if (resp.success) {
        token = resp.data.token
        teamId = resp.data.user.teams[0].id
      }

      dappIds = (
        await Promise.all([
          manager.dapp.createDApp({
            token,
            teamId,
            name: faker.string.alphanumeric(10),
            address: faker.string.hexadecimal({ length: 40 }),
          }),
          manager.dapp.createDApp({
            token,
            teamId,
            name: faker.string.alphanumeric(10),
            address: faker.string.hexadecimal({ length: 40 }),
          }),
        ])
      )
        .filter(r => {
          return r.success
        })
        .map(r => r.data.dapp.id)
      expect(dappIds.length).toBe(2)
    })

    describe('when fetching all dapps', () => {
      let user: User
      beforeEach(async () => {
        const res = await manager.auth.me(token)
        expect(res.success).toBe(true)
        if (res.success) {
          user = res.data
        }
      })

      test('then it returns all dapps', async () => {
        const dapps = user.teams[0].dapps
        expect(dapps?.length).toBe(2)
        dappIds.forEach(dappId => {
          expect(dapps?.some(dapp => dapp.id === dappId)).toBe(true)
        })
      })
    })
  })
})
