import {
  INestApplication,
  Injectable,
  Logger,
  OnModuleInit,
} from '@nestjs/common'
import { PrismaClient } from '@prisma/client'
import { ClsService } from 'nestjs-cls'
@Injectable()
export class PrismaService implements OnModuleInit {
  logger = new Logger(PrismaService.name)

  constructor(
    private readonly prisma: PrismaClient,
    private readonly cls: ClsService,
  ) {}

  async onModuleInit() {
    await this.prisma.$connect()
  }

  async enableShutdownHooks(app: INestApplication) {
    process.on('beforeExit', async () => {
      await app.close()
    })
  }

  private get client(): PrismaClient {
    const tx = this.cls.get('transaction') as PrismaClient
    this.logger.debug(`in a trasaction? ${typeof tx !== undefined}`)
    return tx ?? this.prisma
  }

  get user() {
    return this.client.user
  }

  get team() {
    return this.client.team
  }

  get invitation() {
    return this.client.invitation
  }

  get dapp() {
    return this.#client.dapp
  }
}
