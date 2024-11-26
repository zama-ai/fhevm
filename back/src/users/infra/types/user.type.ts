import { Field, ID, ObjectType } from '@nestjs/graphql'
import { TeamType } from './team.type'

@ObjectType('User')
export class UserType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  email: string

  @Field(() => [TeamType], { nullable: false, description: 'User teams' })
  teams: TeamType[]
}
