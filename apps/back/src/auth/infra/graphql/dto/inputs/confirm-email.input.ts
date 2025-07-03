import { Field, InputType } from '@nestjs/graphql'

@InputType('ConfirmEmailInput')
export class ConfirmEmailInput {
  @Field()
  token: string
}
