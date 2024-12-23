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

describe('fetch all teams', () => {
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

  describe('given a user belongs to many teams', () => {
    let token: string
    let teamIds: string[]

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
        // Note: I should create multiple teams, but this is not supported yet
        teamIds = resp.data.user.teams.map(t => t.id)
      }
    })

    describe('when fetching all teams', () => {
      let user: User
      beforeEach(async () => {
        const res = await manager.auth.me(token)
        expect(res.success).toBe(true)
        if (res.success) {
          user = res.data
        }
      })

      test('then it returns all teams', async () => {
        const { teams } = user
        // TODO: create another team
        expect(teams.length).toBe(1)
        teamIds.forEach(teamId => {
          expect(teams.some(team => team.id === teamId)).toBe(true)
        })
      })
    })
  })
})
