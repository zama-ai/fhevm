import { faker } from '@faker-js/faker'
import gql from 'graphql-tag'
import request from 'supertest-graphql'
import { SetupManager, type GraphQlResponse } from './setup.manager.js'

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
  async createInvitation(email: string): Promise<string> {
    const resp = await request<{ createInvitation: { token: string } }>(
      this.httpServer,
    )
      .mutate(CREATE_INVITATION)
      .variables({ email, secret: process.env.INVITATION_SECRET })
      .expectNoErrors()
    return resp.data!.createInvitation.token
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
      invitation = await this.createInvitation(
        options.email ?? faker.internet.email(),
      )
    }

    const resp = await request<{
      signup: { token: string; user: User }
    }>(this.httpServer)
      .mutate(SIGN_UP)
      .variables({ invitation, name, password })
      .end()

    return resp.data
      ? { success: true, data: resp.data?.signup }
      : { success: false, errors: resp.errors! }
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

    const resp = await request<{ login: { token: string; user: User } }>(
      this.httpServer,
    )
      .mutate(LOGIN)
      .variables({ email, password })
      .end()

    return resp.data
      ? { success: true, data: resp.data.login }
      : { success: false, errors: resp.errors! }
  }

  async me(token: string): Promise<GraphQlResponse<User>> {
    const resp = await request<{ me: User }>(this.httpServer)
      .auth(token, { type: 'bearer' })
      .query(ME)
      .end()

    return resp.data
      ? { success: true, data: resp.data.me }
      : { success: false, errors: resp.errors! }
  }
}

const CREATE_INVITATION = gql`
  mutation CreateInvitation($email: String!, $secret: String!) {
    createInvitation(input: { email: $email, secret: $secret }) {
      token
    }
  }
`

const SIGN_UP = gql`
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

const LOGIN = gql`
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

const ME = gql`
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
