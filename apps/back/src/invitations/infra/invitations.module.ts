import { Module } from '@nestjs/common'
import { InvitationsResolver } from './invitations.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { GetInvitationByToken } from '@/invitations/use-cases/get-invitation-by-token.use-case'
import { CreateInvitation } from '../use-cases/create-invitation.use-case'

@Module({
  imports: [DatabaseModule],
  providers: [InvitationsResolver, GetInvitationByToken, CreateInvitation],
})
export class InvitationsModule {}
