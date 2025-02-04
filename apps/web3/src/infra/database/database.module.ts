import { Module } from '@nestjs/common'
import { ClsModule } from 'nestjs-cls'
import { PrismaClient } from '#prisma/client/index.js'
import { PrismaService } from './prisma.service.js'
import { PrismaFheEventRepository } from './repositories/prisma-fhe-event.repository.js'
import { FHE_EVENT_REPOSITORY, UNIT_OF_WORK } from '#constants.js'

@Module({
  imports: [
    ClsModule.forRoot({
      middleware: {
        mount: false,
      },
    }),
  ],
  providers: [
    {
      provide: 'PrismaClient',
      useFactory: () => {
        new PrismaClient({
          log: [
            {
              emit: 'stdout',
              // TODO: create a config service to solve the configuration
              level:
                process.env.PRISMA_LOGLEVEL === 'debug' ? 'query' : 'error',
            },
          ],
        })
      },
    },
    PrismaService,
    {
      provide: FHE_EVENT_REPOSITORY,
      useClass: PrismaFheEventRepository,
    },
    {
      provide: UNIT_OF_WORK,
      useClass: PrismaService,
    },
  ],
  exports: [FHE_EVENT_REPOSITORY, UNIT_OF_WORK],
})
export class DatabaseModule {}
