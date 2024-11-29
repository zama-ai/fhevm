import { Query, Resolver } from '@nestjs/graphql'
import { GetInvitationByToken } from '@/invitations/use-cases/get-invitation-by-token.use-case'
import { InvitationType } from './types/invitations.type'

@Resolver(() => InvitationType)
export class InvitationsResolver {
  constructor(private readonly getInvitationByTokenUC: GetInvitationByToken) {}

  @Query(() => InvitationType, { name: 'invitation' })
  invitation(@Args('token') token: string) {
    return this.getInvitationByTokenUC.execute(token).toPromise()
  }
}
