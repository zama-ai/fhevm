import { UserType } from '@/users/infra/types/user.type'
import { Field, ObjectType } from '@nestjs/graphql'

@ObjectType('auth')
export class AuthType {
  @Field({ nullable: false })
  user: UserType

  @Field({ nullable: false })
  token: string
}
