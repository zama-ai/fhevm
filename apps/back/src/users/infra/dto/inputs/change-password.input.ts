import { Field, ID, InputType } from '@nestjs/graphql'

@InputType('ChangePasswordInput')
export class ChangePasswordInput {
  @Field(() => String, { nullable: false, description: 'Current password' })
  oldPassword: string

  @Field(() => String, { nullable: false, description: 'New password' })
  newPassword: string
}
