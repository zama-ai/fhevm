import { Injectable, Logger } from '@nestjs/common'
import { PrismaClient } from '#prisma/client/index.js'
import { ClsService } from 'nestjs-cls'
import { Task, UnitOfWork } from 'utils'

@Injectable()
export class PrismaUOW implements UnitOfWork {
  logger = new Logger(PrismaUOW.name)

  constructor(
    private readonly prisma: PrismaClient,
    private readonly cls: ClsService,
  ) {}

  exec<A, E>(task: Task<A, E>): Task<A, E> {
    this.logger.debug('creating the wrapping task')
    return new Task((resolve, reject) => {
      this.prisma
        .$transaction(tx => {
          this.logger.debug('init tx')
          return this.cls.run(() => {
            this.cls.set('transaction', tx)

            return task.toPromise().then(value => {
              this.logger.debug('committing tx')
              resolve(value)
            })
          })
        })
        .catch((err: unknown) => {
          this.logger.warn(`failed: ${err} - rolling back tx`)
          reject(err as E)
        })
    })
  }
}
