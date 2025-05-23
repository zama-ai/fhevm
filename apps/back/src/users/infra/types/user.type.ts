import { TeamType } from '#teams/infra/grapqhl/types/team.type.js'
import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('User')
export class UserType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  email: string

  @Field({ nullable: false })
  name: string

  @Field(() => [TeamType], { nullable: false, description: 'User teams' })
  teams: TeamType[]
}
