import { Field, ID, InputType, Int } from '@nestjs/graphql'

@InputType()
export class UpdateDappInput {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: true })
  name: string

  @Field(() => Int, {
    description: 'Your smart contract chain ID',
    nullable: true,
  })
  chainId: number | null

  @Field(() => String, {
    description:
      'Your smart contract address, it should start with 0x and have 42 characters',
    nullable: true,
  })
  address: string | null
}
