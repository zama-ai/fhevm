import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class CreateDappInput {
  @Field()
  name: string

  @Field()
  teamId: string

  @Field({ nullable: true })
  address: string
}
