import { Field, ObjectType } from '@nestjs/graphql'
import { UserType } from '../../../users/dto/types/user.type'

@ObjectType('auth')
export class AuthType {
  @Field({ nullable: false })
  user: UserType

  @Field({ nullable: false })
  token: string
}
