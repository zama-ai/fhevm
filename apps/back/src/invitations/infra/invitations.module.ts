import { Module } from '@nestjs/common'
import { InvitationsResolver } from './invitations.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { CreateInvitation } from '#invitations/use-cases/create-invitation.use-case.js'

@Module({
  imports: [DatabaseModule],
  providers: [InvitationsResolver, GetInvitationByToken, CreateInvitation],
})
export class InvitationsModule {}
