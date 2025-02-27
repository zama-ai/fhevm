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
    return new Task((resolve, reject) => {
      this.logger.debug(`requested requestId=${requestId}`)
      this.db.snapshot
        .findFirst({
          where: { id: requestId },
        })
        .then(data =>
          resolve(
            data
              ? some(new AppDeployment({ requestId }, data.content))
              : none(),
          ),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }

  upsert = (requestId: string, status: string): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      this.logger.debug(`upserting app deployment=${requestId}`)
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
        .then(() => resolve(void 0))
        .catch((err: unknown) => {
          this.logger.warn(
            `Failed to upsert app deployment ${requestId}: ${err}`,
          )
          reject(unknownError(String(err)))
        })
    })
  }

  delete = (requestId: string): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      this.logger.debug(`deleting app deployment=${requestId}`)
      this.db.snapshot
        .delete({ where: { id: requestId } })
        .then(() => resolve(void 0))
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }
}
