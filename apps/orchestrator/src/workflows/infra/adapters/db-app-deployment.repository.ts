import { AppDeployment } from '#workflows/entities/app-deployment.js'
import { AppDeploymentRepository } from '#workflows/interfaces/app-deployment.repository.js'
import { DatabaseService } from '#database/database.service.js'
import { Logger } from '@nestjs/common'
import { type AppError, Task, unknownError } from 'utils'

function formatId(chainId: string, address: string) {
  return `app-dep:${chainId}/${address}`
}
export class DbAppDeploymentRepository implements AppDeploymentRepository {
  logger = new Logger(DbAppDeploymentRepository.name)

  constructor(private readonly db: DatabaseService) {}

  findByChainIdAndAddress = (
    chainId: string,
    address: string,
  ): Task<AppDeployment, AppError> => {
    return new Task<{ content?: string }, AppError>((resolve, reject) => {
      this.logger.debug(`requested chainId=${chainId} address=${address}`)
      this.db.snapshot
        .findFirst({
          where: { id: formatId(chainId, address) },
        })
        .then(data => resolve({ content: data?.content }))
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).map(({ content }) => new AppDeployment({ chainId, address }, content))
  }

  upsert = (deployment: AppDeployment): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      const id = formatId(deployment.chainId, deployment.address)
      this.logger.debug(`upserting app deployment=${id}`)
      this.db.snapshot
        .upsert({
          create: {
            id,
            content: deployment.snapshot,
          },
          update: {
            content: deployment.snapshot,
          },
          where: { id },
        })
        .then(() => resolve(void 0))
        .catch((err: unknown) => {
          this.logger.warn(`Failed to upsert app deployment ${id}: ${err}`)
          reject(unknownError(String(err)))
        })
    })
  }

  delete = (deployment: AppDeployment): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      const id = formatId(deployment.chainId, deployment.address)
      this.logger.debug(`deleting app deployment=${id}`)
      this.db.snapshot
        .delete({ where: { id } })
        .then(() => resolve(void 0))
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }
}
