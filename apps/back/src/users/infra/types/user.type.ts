import { Field, ID, ObjectType } from '@nestjs/graphql'
import { TeamType } from './team.type.js'

@ObjectType('User')
export class UserType {
  @Field(() => ID, { nullable: false })
  id: `user_${string}`

  @Field({ nullable: false })
  email: string

  @Field({ nullable: false })
  name: string

  @Field(() => [TeamType], { nullable: false, description: 'User teams' })
  teams: TeamType[]
}
