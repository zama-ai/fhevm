import { faker } from '@faker-js/faker'
import { SetupManager, type GraphQlResponse } from './setup.manager.js'
import { GraphQl } from './graphql.js'
import { expect } from 'vitest'

type LoginOptions = {
  signup: boolean
  name: string | (() => string)
  confirm: boolean
}
const DEFAULT_LOGIN_OPTIONS = {
  signup: false,
  name: () => faker.internet.username(),
  confirm: true,
} satisfies LoginOptions

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
        secret: process.env.APP__INVITATION__SECRET!,
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
      email,
      password,
      name,
    }: {
      email: string
      password: string
      name?: string
    },
    options: { confirm?: boolean } = { confirm: true },
  ): Promise<GraphQlResponse<{ user: User }>> {
    if (this.manager.flags.invitations) {
      throw new Error('Invitations are enabled')
    }
    if (!name) {
      name = faker.person.fullName()
    }

    const result = await GraphQl.request<
      { signup: { user: User } },
      { email: string; name: string; password: string }
    >(this.httpServer)
      .mutate(SIGN_UP, { email, name, password })
      .exec('signup')

    if (options?.confirm) {
      await this.confirmUserWithDB(email)
    }

    return result
  }
  async signupWithToken(
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
    if (!this.manager.flags.invitations) {
      throw new Error('Invitations are disabled')
    }

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
      { signupWithInvitation: { token: string; user: User } },
      { invitation: string; name: string; password: string }
    >(this.httpServer)
      .mutate(SIGN_UP_WITH_TOKEN, { invitation: invitation!, name, password })
      .exec('signupWithInvitation')
  }

  async confirmEmail(
    token: string,
  ): Promise<GraphQlResponse<{ user: User; token: string }>> {
    console.log(`\x1b[32mconfirmEmail> ${token}\x1b[0m`)
    return GraphQl.request<
      { confirmEmail: { token: string; user: User } },
      { token: string }
    >(this.httpServer)
      .query(CONFIRM_EMAIL, { token })
      .exec('confirmEmail')
  }

  private async confirmUserWithDB(email: string) {
    await this.manager.prismaClient.user.update({
      where: { email },
      data: { confirmedAt: new Date() },
    })
  }

  async login(
    { email, password }: { email: string; password: string },
    opts: Partial<LoginOptions> = {},
  ): Promise<GraphQlResponse<{ user: User; token: string }>> {
    const options = Object.assign({}, DEFAULT_LOGIN_OPTIONS, opts)
    if (options.signup) {
      const name =
        typeof options.name === 'function' ? options.name() : options.name
      if (this.manager.flags.invitations) {
        await this.signupWithToken(
          {
            name,
            password,
          },
          { createInvitation: true, email },
        )
      } else {
        await this.signup({ email, password, name })
        if (options.confirm) {
          await this.confirmUserWithDB(email)
        }
      }
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

  async requestPasswordReset(email: string): Promise<GraphQlResponse<boolean>> {
    return await GraphQl.request<
      { requestPasswordReset: boolean },
      { email: string }
    >(this.httpServer)
      .mutate(REQUEST_PASSWORD_RESET, { email })
      .exec('requestPasswordReset')
  }

  async resetPassword(input: {
    token: string
    password: string
  }): Promise<GraphQlResponse<{ user: User; token: string }>> {
    return await GraphQl.request<
      { resetPassword: { user: User; token: string } },
      { token: string; password: string }
    >(this.httpServer)
      .mutate(RESET_PASSWORD, input)
      .exec('resetPassword')
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

const SIGN_UP_WITH_TOKEN = `
  mutation signup($invitation: String!, $name: String!, $password: String!) {
    signupWithInvitation(
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

const SIGN_UP = `
  mutation signup($email: String!, $name: String!, $password: String!) {
    signup(
      input: { email: $email, password: $password, name: $name }
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
    }
  }
`

const CONFIRM_EMAIL = `
  mutation confirmEmail($token: String!) {
    confirmEmail(input: { token: $token }) {
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
        }
      }
    }
  }
`

const REQUEST_PASSWORD_RESET = `
  mutation RequestPasswordReset($email: String!) {
    requestResetPassword(input: {
      email: $email
    })
  }
`

const RESET_PASSWORD = `
  mutation ResetPassword($token: String!, $password: String!) {
    resetPassword(input: {
      token: $token
      password: $password
    }) {
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
