import { AppError, Task, unknownError, UseCase } from 'utils'

import { DApp } from '#dapps/domain/entities/dapp.js'
import { SubscriptionService } from '../domain/services/subscription.service.js'
import { BRAND } from 'zod'

export class NotifyDappUpdated implements UseCase<DApp, void> {
  constructor(private readonly service: SubscriptionService) {}

  execute(dapp: DApp): Task<void, AppError> {
    return new Task((resolve, reject) => {
      console.log('📢 notify dapp updated use case')
      this.service
        .publish('dappUpdated', {
          dapp: {
            id: dapp.id.value as `dapp_${string}` & BRAND<'DAppId'>,
            teamId: dapp.teamId,
            createdAt: dapp.createdAt.value as Date & BRAND<'CreatedAt'>,
            status: dapp.status,
            name: dapp.name,
            address: dapp.address,
          },
        })

        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    })
  }
}
