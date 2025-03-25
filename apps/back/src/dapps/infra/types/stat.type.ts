import { Field, ID, ObjectType } from '@nestjs/graphql'

// @ObjectType('DailyDappStats')
// export class DailyDappStatsType {
//   @Field(() => ID, { nullable: false, description: 'The id of the day' })
//   id: string

//   @Field({ nullable: false })
//   day: string

//   @Field({ nullable: false })
//   total: number

//   @Field({ nullable: false })
//   FheAdd: number
// }

@ObjectType('CumulativeDappStats')
export class CumulativeDappStatsType {
  @Field({ nullable: false })
  total: number

  @Field({ nullable: false })
  FheAdd: number
}

@ObjectType('DappStats')
export class DappStatsType {
  @Field(() => ID, { nullable: false, description: 'The id of the dapp' })
  id: string

  // @Field(() => CumulativeDappStatsType, { nullable: false })
  // cumulative: CumulativeDappStatsType

  //   @Field({ nullable: false })
  //   byDay: DailyDappStatsType[]
}
