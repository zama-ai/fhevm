import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('Chain')
export class ChainType {
  @Field(() => ID, { description: 'Chain ID' })
  id: string

  @Field(() => String, { description: 'Chain name', nullable: false })
  name: string

  @Field(() => String, { description: 'Chain description', nullable: true })
  description: string
}
