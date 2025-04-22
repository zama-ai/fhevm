import { AppDeployment } from '#workflows/entities/app-deployment.js'
import { AppDeploymentRepository } from '#workflows/interfaces/app-deployment.repository.js'
import { DatabaseService } from '#database/database.service.js'
import { Logger } from '@nestjs/common'
import { type AppError, none, Option, some, Task, unknownError } from 'utils'
export class DbAppDeploymentRepository implements AppDeploymentRepository {
  logger = new Logger(DbAppDeploymentRepository.name)

  constructor(private readonly db: DatabaseService) {}

  findByRequestId = (
    requestId: string,
  ): Task<Option<AppDeployment>, AppError> => {
    return Task.fromPromise(
      this.db.snapshot
        .findFirst({ where: { id: requestId } })
        .then(data =>
          data ? some(new AppDeployment({ requestId }, data.content)) : none(),
        )
        .catch((err: unknown) => {
          this.logger.warn(`Failed to find app deployment ${requestId}: ${err}`)
          throw unknownError(String(err))
        }),
    )
  }

  upsert = (requestId: string, status: string): Task<void, AppError> => {
    return Task.fromPromise(
      this.db.snapshot
        .upsert({
          create: {
            id: requestId,
            content: status,
          },
          update: {
            content: status,
          },
          where: { id: requestId },
        })
        .then(() => void 0)
        .catch((err: unknown) => {
          this.logger.warn(
            `Failed to upsert app deployment ${requestId}: ${err}`,
          )
          throw unknownError(String(err))
        }),
    )
  }

  delete = (requestId: string): Task<void, AppError> => {
    this.logger.debug(`deleting app deployment=${requestId}`)
    return Task.fromPromise(
      this.db.snapshot
        .delete({ where: { id: requestId } })
        .then(() => void 0)
        .catch((err: unknown) => {
          this.logger.warn(
            `Failed to delete app deployment ${requestId}: ${err}`,
          )
          throw unknownError(String(err))
        }),
    )
  }
}
