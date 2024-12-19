import { AppModule } from '@/app.module'
import { DAppStatus } from '@/dapps/domain/entities/dapp'
import { PrismaClient } from '@/prisma/client'
import { faker } from '@faker-js/faker'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'
import gql from 'graphql-tag'
import request from 'supertest-graphql'

export interface User {
  name: string
  email: string
  teams: { id: string; name: string }[]
}

export interface DApp {
  id: string
  name: string
  address: string | null
  status: DAppStatus
  team: {
    id: string
    name: string
  }
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

  async createDApp({
    token,
    teamId,
    name,
  }:
    | {
        token: string
        teamId: string
        name: string
      }
    | { token?: never; teamId?: never; name: string }): Promise<
    GraphQlResponse<{ dapp: DApp; token: string }>
  > {
    if (!token) {
      const result = await this.signup(
        {
          name: faker.internet.username(),
          password: faker.internet.password(),
        },
        { createInvitation: true },
      )
      if (result.success) {
        token = result.data?.token
        teamId = result.data?.user.teams[0].id
      } else {
        return result
      }
    }

    const resp = await request<{ createDapp: DApp }>(this.httpServer)
      .auth(token, { type: 'bearer' })
      .mutate(CREATE_DAPP)
      .variables({ teamId, name })

    return resp.data
      ? { success: true, data: { dapp: resp.data.createDapp, token } }
      : { success: false, errors: resp.errors! }
  }
  async updateDApp({
    token,
    dappId,
    name,
    address,
  }: {
    token: string
    dappId: string
    name?: string
    address?: string
  }): Promise<GraphQlResponse<{ dapp: DApp }>> {
    const resp = await request<{ updateDapp: DApp }>(this.httpServer)
      .auth(token, { type: 'bearer' })
      .mutate(UPDATE_DAPP)
      .variables({ appId: dappId, name, address })

    return resp.data
      ? { success: true, data: { dapp: resp.data.updateDapp } }
      : { success: false, errors: resp.errors! }
  }

  async listUsers() {
    const prisma = this.#app.get<PrismaClient>(PrismaClient)
    return await prisma.user.findMany({
      select: { id: true, name: true, email: true, teams: true },
    })
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

const UPDATE_DAPP = gql`
  mutation updateApp($appId: ID!, $name: String, $address: String) {
    updateDapp(input: { id: $appId, name: $name, address: $address }) {
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
