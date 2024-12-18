import { AppModule } from '@/app.module'
import { PrismaClient } from '@/prisma/client'
import { faker } from '@faker-js/faker'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'
import gql from 'graphql-tag'
import request from 'supertest-graphql'

export interface User {
  name: string
  email: string
  teams: { id: number; name: string }[]
}

export type GraphQlResponse<T> =
  | {
      success: true
      data: T
    }
  | {
      success: false
      errors: ReadonlyArray<{ message: string }>
    }

export class IntegrationManager {
  #app: INestApplication
  async beforeAll() {
    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    }).compile()

    this.#app = moduleRef.createNestApplication({ cors: true })
    await this.#app.init()
  }

  async afterAll() {
    await this.#app.close()
  }

  async afterEach() {
    // Clear the database
    const prisma = this.#app.get<PrismaClient>(PrismaClient)
    await prisma.$transaction([
      prisma.user.deleteMany(),
      prisma.team.deleteMany(),
      prisma.invitation.deleteMany(),
      prisma.dapp.deleteMany(),
    ])
  }

  get httpServer(): any {
    return this.#app.getHttpServer()
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
      token,
      name,
      password,
    }: {
      token: string
      name: string
      password: string
    },
    options?: { createInvitation?: boolean; email?: string },
  ): Promise<GraphQlResponse<{ token: string; user: User }>> {
    if (options?.createInvitation) {
      token = await this.createInvitation(
        options.email ?? faker.internet.email(),
      )
    }

    const resp = await request<{
      signup: { token: string; user: User }
    }>(this.httpServer)
      .mutate(SIGN_UP)
      .variables({ token, name, password })
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
          token: '',
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
}

const CREATE_INVITATION = gql`
  mutation CreateInvitation($email: String!, $secret: String!) {
    createInvitation(input: { email: $email, secret: $secret }) {
      token
    }
  }
`

const SIGN_UP = gql`
  mutation signup($token: String!, $name: String!, $password: String!) {
    signup(
      input: { invitationToken: $token, password: $password, name: $name }
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
