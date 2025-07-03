import { UserType } from '#users/infra/types/user.type.js'
import { Field, ObjectType } from '@nestjs/graphql'

@ObjectType('authWithToken')
export class AuthWithTokenType {
  @Field({ nullable: false })
  user: UserType

  @Field({ nullable: false })
  token: string
}
