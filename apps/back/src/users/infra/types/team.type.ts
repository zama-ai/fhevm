import { DappType } from '@/dapps/infra/types/dapp.type'
import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('Team')
export class TeamType {
  @Field(() => ID, { nullable: false })
  id: `team_${string}`

  @Field({ nullable: false })
  name: string

  @Field(() => [DappType], { nullable: false })
  dapps: DappType[]
}
