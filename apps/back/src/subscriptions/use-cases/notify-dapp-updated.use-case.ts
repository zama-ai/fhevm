import { AppError, Task, unknownError, UseCase } from 'utils'

import { DApp } from '#dapps/domain/entities/dapp.js'
import { SubscriptionService } from '../domain/services/subscription.service.js'

export class NotifyDappUpdated implements UseCase<DApp, void> {
  constructor(private readonly service: SubscriptionService) {}

  execute(dapp: DApp): Task<void, AppError> {
    return new Task((resolve, reject) => {
      console.log('📢 notify dapp updated use case')
      this.service
        .publish('dummy', {
          dummy: {
            id: dapp.id.value,
            name: dapp.name,
          },
        })
        // .publish('dappUpdated', {
        //   dapp: {
        //     id: dapp.id.value as `dapp_${string}` & BRAND<'DAppId'>,
        //     teamId: dapp.teamId,
        //     createdAt: dapp.createdAt.value as Date & BRAND<'CreatedAt'>,
        //     status: dapp.status,
        //     name: dapp.name,
        //     address: dapp.address,
        //   },
        // })

        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    })
  }
}
