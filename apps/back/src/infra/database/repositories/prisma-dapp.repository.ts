import { Injectable } from '@nestjs/common'
import type { AppError } from 'utils'
import { Task, unknownError } from 'utils'

import { DApp } from '@/dapps/domain/entities/dapp'
import { DAppRepository } from '@/dapps/domain/repositories/dapp.repository'

import { PrismaService } from '../prisma.service'

@Injectable()
export class PrismaDAppRepository extends DAppRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  create(data: DApp): Task<DApp, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .create({ data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  update(data: DApp): Task<DApp, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .update({ where: { id: data.id }, data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  findOneByIdAndUserId(id: string, userId: string): Task<DApp, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .findUnique({
          where: {
            id,
            team: {
              users: {
                some: { id: { equals: userId } },
              },
            },
          },
        })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }
}
