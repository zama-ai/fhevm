import { INestApplication, Injectable, OnModuleInit } from '@nestjs/common'
import { PrismaClient } from '@prisma/client'

@Injectable()
export class PrismaService implements OnModuleInit {
  #client = new PrismaClient({
    log: [
      {
        emit: 'stdout',
        // TODO: create a config service to solve the configuration
        level: process.env.PRISMA_LOGLEVEL === 'debug' ? 'query' : 'error',
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
