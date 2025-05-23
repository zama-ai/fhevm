import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('Invitation')
export class InvitationType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  email: string

  @Field({ nullable: false })
  token: string

  @Field({ nullable: false })
  expiresAt: number
}
