import { User } from '@/users/domain/entities/user'
import { type AppError, Task, UnitOfWork, type UseCase } from 'utils'
import { DApp } from '../domain/entities/dapp'
import { AppDeploymentProducer } from '../domain/services/app-deployment.producer'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { UNIT_OF_WORK } from '@/constants'
import { Inject } from '@nestjs/common'
import { UpdateDapp } from './update-dapp.use-case'
import { requested } from 'messages'
import { randomUUID } from 'crypto'

interface Input {
  user: User // to check if they can deploy
  applicationId: string
  // deploymentId: string  // it will be random uuid for now
  // chainId: string // for now it's sepolia
  // address will be fetch from the dapp entity
}

interface Output {
  dApp: DApp // the updated dapp
}

export class DeployDAppUseCase implements UseCase<Input, Output> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly dappRepository: DAppRepository,
    private readonly producer: AppDeploymentProducer,
    private readonly updateDappUC: UpdateDapp,
  ) {}
  execute(input: Input): Task<Output, AppError> {
    // check if the user can deploy by checking if the user belongs to the team that owns the dapp
    return this.uow
      .exec(
        this.dappRepository
          .findOneByIdAndUserId(input.applicationId, input.user.id.value)
          .chain(dapp =>
            Task.all([
              this.producer.publish(
                requested({
                  applicationId: input.applicationId,
                  deploymentId: randomUUID(),
                  address: dapp.address!,
                  // TODO: move it into a constants file
                  chainId: '11155111', // sepolia
                }),
              ),
              this.updateDappUC.execute({
                dapp: {
                  id: input.applicationId,
                  status: 'DEPLOYING',
                },
                user: input.user,
              }),
            ]),
          ),
      )
      .map(([, dapp]) => ({ dApp: dapp }))
  }
}
