import { AppDeployment } from '#workflows/entities/app-deployment.js'
import { AppDeploymentRepository } from '#workflows/interfaces/app-deployment.repository.js'
import { DatabaseService } from '#database/database.service.js'
import { Logger } from '@nestjs/common'
import { type AppError, none, Option, some, Task, unknownError } from 'utils'

const PREFIX = 'app_dep'

function formatId(dAppId: string) {
  return `${PREFIX}:${dAppId}`
}

function extractDAppId(id: string) {
  return id.split(':')[1]
}

function formatSecondaryKey(chainId: string, address: string) {
  return `${PREFIX}:${address}/${chainId}`
}

function extractChainIdAndAddress(secondaryKey: string) {
  const [address, chainId] = secondaryKey.split(':')[0].split('/')
  return { address, chainId }
}

export class DbAppDeploymentRepository implements AppDeploymentRepository {
  logger = new Logger(DbAppDeploymentRepository.name)

  constructor(private readonly db: DatabaseService) {}

  findByDAppId = (dAppId: string): Task<Option<AppDeployment>, AppError> => {
    return new Task((resolve, reject) => {
      this.logger.debug(`requested dAppId=${dAppId}`)
      this.db.snapshot
        .findFirst({
          where: { id: formatId(dAppId) },
        })
        .then(data =>
          resolve(
            data
              ? some(
                  new AppDeployment(
                    {
                      dAppId,
                      ...extractChainIdAndAddress(data.secondaryKey),
                    },
                    data.content,
                  ),
                )
              : none(),
          ),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }

  findByChainIdAndAddress = (
    chainId: string,
    address: string,
  ): Task<Option<AppDeployment>, AppError> => {
    return new Task((resolve, reject) => {
      this.logger.debug(`requested chainId=${chainId} address=${address}`)
      this.db.snapshot
        .findFirst({
          where: { secondaryKey: formatSecondaryKey(chainId, address) },
        })
        .then(data =>
          resolve(
            data
              ? some(
                  new AppDeployment(
                    {
                      dAppId: extractDAppId(data.id),
                      chainId,
                      address,
                    },
                    data.content,
                  ),
                )
              : none(),
          ),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }

  upsert = (deployment: AppDeployment): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      const id = formatId(deployment.dAppId)
      this.logger.debug(`upserting app deployment=${id}`)
      this.db.snapshot
        .upsert({
          create: {
            id,
            secondaryKey: formatSecondaryKey(
              deployment.chainId,
              deployment.address,
            ),
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
      const id = formatId(deployment.dAppId)
      this.logger.debug(`deleting app deployment=${id}`)
      this.db.snapshot
        .delete({ where: { id } })
        .then(() => resolve(void 0))
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }
}
