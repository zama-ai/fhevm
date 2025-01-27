import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class DeployedDAppInput {
  @Field(() => ID, { nullable: false })
  id: `dapp_${string}`
}
