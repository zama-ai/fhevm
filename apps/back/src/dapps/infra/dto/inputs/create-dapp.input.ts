import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class CreateDappInput {
  @Field()
  name: string

  @Field()
  teamId: `t_${string}`

  @Field({
    description:
      'Your smart contract address, it should start with 0x and have 42 characters',
    nullable: true,
  })
  address: string
}
