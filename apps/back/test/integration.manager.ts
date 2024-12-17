import { AppModule } from '@/app.module'
import { PrismaClient } from '@/prisma/client'
import { INestApplication } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Test } from '@nestjs/testing'
import gql from 'graphql-tag'
import request from 'supertest-graphql'

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
    token: string,
    name: string,
    password: string,
  ): Promise<{ token: string; user: { email: string; name: string } }> {
    const resp = await request<{
      signup: { token: string; user: { email: string; name: string } }
    }>(this.httpServer)
      .mutate(SIGN_UP)
      .variables({ token, name, password })
      .end()
    // .expectNoErrors()
    return resp.data?.signup ?? { token: '', user: { email: '', name: '' } }
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
      }
      token
    }
  }
`
