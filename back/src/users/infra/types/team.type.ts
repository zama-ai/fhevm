import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('Team')
export class TeamType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string
}
