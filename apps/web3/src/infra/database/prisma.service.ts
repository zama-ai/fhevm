import { PrismaClient } from '#prisma/client/index.js'
import {
  INestApplication,
  Injectable,
  Logger,
  OnModuleInit,
} from '@nestjs/common'
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
    this.logger.debug(`in a trasaction? ${typeof tx !== 'undefined'}`)
    return tx ?? this.prisma
  }

  get fheEvent() {
    return this.client.fheEvent
  }
}
