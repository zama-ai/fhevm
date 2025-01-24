import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('Dummy')
export class DummyType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string
}
