import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppDeploymentEvent } from 'messages'
import { AppError, Task, unknownError, UseCase } from 'utils'

type Input = Extract<
  AppDeploymentEvent,
  | { type: 'app-deployment.sc-discovered' }
  | { type: 'app-deployment.sc-discovery-failed' }
>
@Injectable()
export class ScDiscovered implements UseCase<Input, void> {
  constructor(
    private readonly getUserById: GetUserById,
    private readonly updateDappUC: UpdateDapp,
    @Inject(SUBSCRIPTION_SERVICE)
    private readonly subscriptions: SubscriptionService,
  ) {}
  execute({ type, payload, meta }: Input): Task<void, AppError> {
    if (!meta?.userId) {
      return Task.reject(unknownError('Missing user id'))
    }
    const id = UserId.parse(meta.userId)
    if (!id) {
      return Task.reject(unknownError('Badly formatted user id'))
    }
    return this.getUserById
      .execute(id)
      .chain(user =>
        this.updateDappUC.execute({
          dapp: {
            id: new DAppId(DAppId.parse(payload.applicationId)),
            status: type === 'app-deployment.sc-discovered' ? 'LIVE' : 'DRAFT',
          },
          user,
        }),
      )
      .chain(
        dapp =>
          new Task((resolve, reject) =>
            this.subscriptions
              // .publish('dappUpdated', { dapp: dapp.toJSON() })
              .publish('dummy', {
                dummy: { id: dapp.toJSON().id, name: dapp.toJSON().name },
              })
              .then(resolve)
              .catch(reject),
          ),
      )
  }
}
