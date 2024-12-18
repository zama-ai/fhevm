import { IntegrationManager } from '@/tests/integration.manager'
import { faker } from '@faker-js/faker'
import gql from 'graphql-tag'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
} from 'vitest'
import request, { SuperTestExecutionResult } from 'supertest-graphql'
import { DappStatus } from '@/dapps/infra/types/dapp.type'

const CREATE_DAPP = gql`
  mutation createDApp($teamId: String!, $name: String!) {
    createDapp(input: { teamId: $teamId, name: $name }) {
      id
      name
      address
      status
      team {
        id
        name
      }
    }
  }
`

interface DApp {
  id: string
  name: string
  address: string | null
  status: DappStatus
  team: {
    id: string
    name: string
  }
}

describe('create-dapp', () => {
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

  describe('given a user is logged in', () => {
    let token: string
    let teamId: string

    beforeEach(async () => {
      const result = await manager.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id
      }
    })

    describe('when creating a dapp', () => {
      let dapp: DApp | undefined

      beforeEach(async () => {
        const result = await request<{ createDapp: DApp }>(manager.httpServer)
          .auth(token, { type: 'bearer' })
          .mutate(CREATE_DAPP)
          .variables({ teamId, name: faker.string.alphanumeric(10) })
          .expectNoErrors()
        dapp = result.data?.createDapp
      })

      test('then it creates a DRAFT dapp', () => {
        expect(dapp).toBeDefined()
        expect(dapp?.status).toBe('DRAFT')
      })

      test('then it associates the dapp with the right team', () => {
        expect(dapp?.team.id).toBe(teamId)
      })
    })
  })

  describe('given a user is not logged in', () => {
    describe('when creating a dapp', () => {
      let result: SuperTestExecutionResult<{ createDapp: DApp }>

      beforeEach(async () => {
        result = await request<{ createDapp: DApp }>(manager.httpServer)
          .mutate(CREATE_DAPP)
          .variables({
            teamId: faker.string.uuid(),
            name: faker.string.alphanumeric(10),
          })
      })

      test('then it fails due to unauthorized error', () => {
        expect(result.errors).toBeDefined()
        expect(result.errors?.length).toBeGreaterThan(0)
        expect(result.errors![0].message).toBe('Unauthorized')
      })
    })
  })
})
