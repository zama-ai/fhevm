import { Injectable } from '@nestjs/common'
import { AppError, unknownError } from '@/utils/app-error'
import { Task } from '@/utils/task'

import { Dapp } from '@/dapps/domain/entities/dapp'
import { DappRepository } from '@/dapps/domain/repositories/dapp.repository'

import { PrismaService } from '../prisma.service'

@Injectable()
export class PrismaDappRepository extends DappRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  create(data: Dapp): Task<Dapp, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .create({ data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Dapp.parse(props).async())
  }

  update(data: Dapp): Task<Dapp, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .update({ where: { id: data.id }, data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Dapp.parse(props).async())
  }

  findOneByIdAndUserId(id: string, userId: string): Task<Dapp, AppError> {
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
    }).chain(props => Dapp.parse(props).async())
  }
}
