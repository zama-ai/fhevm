import { Field, ID, ObjectType, registerEnumType } from '@nestjs/graphql'

export enum DappStatus {
  DRAFT = 'DRAFT',
  DEPLOYING = 'DEPLOYING',
  LIVE = 'LIVE',
  DELETED = 'DELETED',
}

registerEnumType(DappStatus, {
  name: 'DappStatus',
  valuesMap: {
    DRAFT: {
      description: 'Still being worked on',
    },
    DEPLOYING: {
      description: 'We are deploying it',
    },
    LIVE: {
      description: 'You can use it now',
    },
    DELETED: {
      deprecationReason: 'Not implmented yet',
    },
  },
})

@ObjectType('Dapp')
export class DappType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string

  @Field(() => DappStatus, { nullable: false })
  status: DappStatus

  @Field({ nullable: true })
  address: string

  // TODO: Uncomment this line after creating the TeamType
  // @Field(() => TeamType, { nullable: false })
}
