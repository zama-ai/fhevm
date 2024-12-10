import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('Dapp')
export class DappType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string

  @Field({ nullable: true })
  address: string
}
