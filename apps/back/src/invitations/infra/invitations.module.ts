import { Module } from '@nestjs/common'
import { InvitationsResolver } from './invitations.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { GetInvitationByToken } from '@/invitations/use-cases/get-invitation-by-token.use-case'

@Module({
  imports: [DatabaseModule],
  providers: [InvitationsResolver, GetInvitationByToken],
})
export class InvitationsModule {}
