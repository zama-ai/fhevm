import { gql } from 'graphql-tag'
import request from 'supertest-graphql'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
} from 'vitest'
import { faker } from '@faker-js/faker'
import { IntegrationManager } from '@/tests/integration.manager'

const GET_INVITATION_BY_TOKEN = gql`
  query GetInvitationByToken($token: String!) {
    invitation(token: $token) {
      id
      email
    }
  }
`

describe('createInvitation', () => {
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

  describe('given no invitation exists', () => {
    describe('when creating an invitation', () => {
      let email: string
      let token: string

      beforeEach(async () => {
        email = faker.internet.email()
        token = await manager.auth.createInvitation(email)
      })

      test('then it returns a new invitation token', () => {
        expect(token).toBeDefined()
      })

      test('then it creates a new invitation with the right email', async () => {
        const resp = await request<{
          invitation: { id: string; email: string }
        }>(manager.httpServer)
          .query(GET_INVITATION_BY_TOKEN)
          .variables({ token })
          .expectNoErrors()
        expect(resp.data!.invitation.email).toBe(email)
      })
    })
  })
})
