import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class QueryChainInput {
  @Field(() => ID, { description: 'Chain ID', nullable: false })
  id: string
}
