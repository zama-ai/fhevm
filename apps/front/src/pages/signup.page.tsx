import { useLoaderData } from 'react-router'
import { InvitationTokenQuery } from '@/__generated__/graphql'

import { SignupForm } from '@/components/signup-form/signup-form'

export function SignupPage() {
  const {
    invitation: { token, email },
  } = useLoaderData<InvitationTokenQuery>()
  return (
    <SignupForm
      onSubmit={() => {}}
      loading={false}
      invitationToken={token}
      email={email}
    />
  )
}
