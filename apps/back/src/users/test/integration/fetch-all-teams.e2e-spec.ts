import { IntegrationManager, User } from '#tests/integration.manager.js'
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
  }, 30000)

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
        expect(res.success, 'Failed to query ME').toBe(true)
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

  describe('given a team has been deleted', () => {
    let token: string
    let teamIds: string[]

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
        teamIds = login.data.user.teams.map(t => t.id)
        // TODO: move to a GraphQL when implemented
        await manager.prismaClient.team.update({
          data: { deletedAt: new Date() },
          where: { id: teamIds[0] },
        })
      } else {
        console.log(`login: ${JSON.stringify(login)}`)
        expect(login.success, 'it should succeed').toBe(true)
      }
    })

    describe('when fetching all teams', () => {
      let user: User
      beforeEach(async () => {
        const me = await manager.auth.me(token)
        if (me.success) {
          user = me.data
        } else {
          console.log(`me: ${JSON.stringify(me)}`)
          expect(me.success, 'Failed to query ME').toBe(true)
        }
      })

      test('then it returns all not-deleted teams', async () => {
        const { teams } = user
        // TODO: create another team
        expect(teams.length).toBe(0)
      })
    })
  })
})
