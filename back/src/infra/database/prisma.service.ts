import { INestApplication, Injectable, OnModuleInit } from '@nestjs/common'
import { PrismaClient } from '@prisma/client'

@Injectable()
export class PrismaService implements OnModuleInit {
  #client = new PrismaClient({
    log: [
      {
        emit: 'stdout',
        level: process.env.NODE_ENV === 'production' ? 'error' : 'query',
      },
    ],
  })

  async onModuleInit() {
    await this.#client.$connect()
  }

  async enableShutdownHooks(app: INestApplication) {
    process.on('beforeExit', async () => {
      await app.close()
    })
  }

  get user() {
    return this.#client.user
  }

  get team() {
    return this.#client.team
  }
}
