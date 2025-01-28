import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class QueryDappInput {
  @Field(() => ID, { nullable: false })
  id: string
}
