import { AppModule } from '@/app.module'
import { PrismaClient } from '@/prisma/client'
import { INestApplication } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Test } from '@nestjs/testing'

export class IntegrationManager {
  #app: INestApplication
  async beforeAll() {
    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    }).compile()

    this.#app = moduleRef.createNestApplication({ cors: true })
    await this.#app.init()

    console.log('db.url', this.#app.get(ConfigService).get('db.url'))
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
}
