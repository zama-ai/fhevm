import { Module } from '@nestjs/common'
import { ClsModule } from 'nestjs-cls'
import { PrismaService } from './prisma.service.js'
import { DAPP_REPOSITORY } from '#dapps/domain/repositories/dapp.repository.js'
import { USER_REPOSITORY } from '#users/domain/repositories/user.repository.js'
import { PrismaUserRepository } from './repositories/prisma-user.repository.js'
import { PrismaDAppRepository } from './repositories/prisma-dapp.repository.js'
import { PrismaClient } from '#prisma/client/index.js'
import { UNIT_OF_WORK } from '#constants.js'
import { PrismaUOW } from './prisma.uow.js'

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
      provide: PrismaClient,
      useFactory: () =>
        new PrismaClient({
          log: [
            {
              emit: 'stdout',
              // TODO: create a config service to solve the configuration
              level:
                process.env.PRISMA_LOGLEVEL === 'debug' ? 'query' : 'error',
            },
          ],
        }),
    },
    PrismaService,
    {
      provide: USER_REPOSITORY,
      useClass: PrismaUserRepository,
    },
    {
      provide: DAPP_REPOSITORY,
      useClass: PrismaDAppRepository,
    },
    {
      provide: UNIT_OF_WORK,
      useClass: PrismaUOW,
    },
  ],
  exports: [PrismaService, USER_REPOSITORY, DAPP_REPOSITORY, UNIT_OF_WORK],
})
export class DatabaseModule {}
