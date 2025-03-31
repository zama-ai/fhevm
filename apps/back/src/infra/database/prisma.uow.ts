import { Injectable, Logger } from '@nestjs/common'
import { PrismaClient } from '#prisma/client/index.js'
import { ClsService } from 'nestjs-cls'
import { isAppError, Task, UnitOfWork } from 'utils'

@Injectable()
export class PrismaUOW implements UnitOfWork {
  logger = new Logger(PrismaUOW.name)

  constructor(
    private readonly prisma: PrismaClient,
    private readonly cls: ClsService,
  ) {}

  exec<A, E>(task: Task<A, E>): Task<A, E> {
    this.logger.verbose('creating the wrapping task')
    return new Task((resolve, reject) => {
      this.prisma
        .$transaction(tx => {
          this.logger.verbose('init tx')
          return this.cls.run(() => {
            this.cls.set('transaction', tx)

            return task.toPromise().then(value => {
              this.logger.verbose('committing tx')
              resolve(value)
            })
          })
        })
        .catch((err: unknown) => {
          this.logger.warn(
            `failed: ${isAppError(err) ? err.message : err} - rolling back tx`,
          )
          reject(err as E)
        })
    })
  }
}
