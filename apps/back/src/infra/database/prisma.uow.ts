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
    const tx = this.cls.get('transaction') as PrismaClient | undefined
    if (tx) {
      this.logger.verbose(`already in transaction`)
      return task
    }

    return new Task((resolve, reject) => {
      this.logger.verbose(`starting a transaction`)
      this.prisma
        .$transaction(tx => {
          this.logger.verbose('init tx')
          return this.cls.run(() => {
            this.cls.set('transaction', tx)

            return task.toPromise().then(value => {
              // NOTE: I need to clear the AsyncLocalStorage from the tx
              // in case I try to call it later.
              this.cls.set('transaction', undefined)
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
