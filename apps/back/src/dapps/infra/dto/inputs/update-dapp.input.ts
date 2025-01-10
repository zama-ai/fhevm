import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class UpdateDappInput {
  @Field(() => ID, { nullable: false })
  id: `dapp_${string}`

  @Field({ nullable: true })
  name: string

  @Field({
    description:
      'Your smart contract address, it should start with 0x and have 42 characters',
    nullable: true,
  })
  address: string
}
