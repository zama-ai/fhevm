import {
  type INestApplication,
  Injectable,
  type OnModuleInit,
} from '@nestjs/common'
import { PrismaClient } from '#prisma/client/index.js'

@Injectable()
export class DatabaseService implements OnModuleInit {
  #client = new PrismaClient()

  async onModuleInit() {
    await this.#client.$connect()
  }

  async enableShutdownHooks(app: INestApplication) {
    process.on('beforeExit', async () => {
      await app.close()
    })
  }

  get snapshot() {
    return this.#client.snapshot
  }
}
