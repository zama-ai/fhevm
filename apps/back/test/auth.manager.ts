import { faker } from '@faker-js/faker'
import { SetupManager, type GraphQlResponse } from './setup.manager.js'
import { GraphQl } from './graphql.js'
import { expect } from 'vitest'

export interface User {
  name: string
  email: string
  teams: { id: string; name: string; dapps?: { id: string; name: string }[] }[]
}

export class AuthManager {
  constructor(private readonly manager: SetupManager) {}

  get httpServer() {
    return this.manager.httpServer
  }
  createInvitation(email: string): Promise<GraphQlResponse<{ token: string }>> {
    return GraphQl.request<
      { createInvitation: { token: string } },
      { email: string; secret: string }
    >(this.httpServer)
      .mutate(CREATE_INVITATION, {
        email,
        secret: process.env.INVITATION_SECRET!,
      })
      .exec('createInvitation')
  }

  getInvitation(token: string) {
    return GraphQl.request<
      { invitation: { id: string; email: string } },
      { token: string }
    >(this.httpServer)
      .query(GET_INVITATION_BY_TOKEN, { token })
      .exec('invitation')
  }

  async signup(
    {
      invitation,
      name,
      password,
    }: {
      invitation?: string
      name: string
      password: string
    },
    options?: { createInvitation?: boolean; email?: string },
  ): Promise<GraphQlResponse<{ token: string; user: User }>> {
    if (!invitation && options?.createInvitation) {
      const created = await this.createInvitation(
        options.email ?? faker.internet.email(),
      )
      expect(created.success).toBe(true)
      if (created.success) {
        invitation = created.data.token
      }
    }

    return GraphQl.request<
      { signup: { token: string; user: User } },
      { invitation: string; name: string; password: string }
    >(this.httpServer)
      .mutate(SIGN_UP, { invitation: invitation!, name, password })
      .exec('signup')
  }

  async login(
    { email, password }: { email: string; password: string },
    options?: { signup?: boolean; name?: string },
  ): Promise<GraphQlResponse<{ user: User; token: string }>> {
    if (options?.signup) {
      await this.signup(
        {
          name: options.name ?? faker.internet.username(),
          password,
        },
        { createInvitation: true, email },
      )
    }

    return GraphQl.request<
      { login: { token: string; user: User } },
      { email: string; password: string }
    >(this.httpServer)
      .mutate(LOGIN, { email, password })
      .exec('login')
  }

  async me(token: string): Promise<GraphQlResponse<User>> {
    return await GraphQl.request<{ me: User }>(this.httpServer)
      .auth(token)
      .mutate(ME)
      .exec('me')
  }
}

const CREATE_INVITATION = `
  mutation CreateInvitation($email: String!, $secret: String!) {
    createInvitation(input: { email: $email, secret: $secret }) {
      token
    }
  }
`

const GET_INVITATION_BY_TOKEN = `
  query GetInvitationByToken($token: String!) {
    invitation(token: $token) {
      id
      email
    }
  }
`

const SIGN_UP = `
  mutation signup($invitation: String!, $name: String!, $password: String!) {
    signup(
      input: { invitationToken: $invitation, password: $password, name: $name }
    ) {
      user {
        id
        email
        name
        teams {
          id
          name
        }
      }
      token
    }
  }
`

const LOGIN = `
  mutation login($email: String!, $password: String!) {
    login(input: { email: $email, password: $password }) {
      user {
        id
        email
        name
        teams {
          id
          name
        }
      }
      token
    }
  }
`

const ME = `
  query me {
    me {
      id
      email
      name
      teams {
        id
        name
        dapps {
          id
          name
          status
        }
      }
    }
  }
`
