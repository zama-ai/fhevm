import { Field, InputType, Int } from '@nestjs/graphql'

@InputType()
export class CreateDappInput {
  @Field()
  name: string

  @Field()
  teamId: string

  @Field(() => Int, {
    description: 'Your smart contract chain ID',
    nullable: true,
  })
  chainId: number

  @Field({
    description:
      'Your smart contract address, it should start with 0x and have 42 characters',
    nullable: true,
  })
  address: string
}
