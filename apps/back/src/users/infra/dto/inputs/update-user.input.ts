import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class UpdateUserInput {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string
}
