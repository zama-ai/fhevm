import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class DeployDAppInput {
  @Field()
  dappId: `dapp_${string}`
}
