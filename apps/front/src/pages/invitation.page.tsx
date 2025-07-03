import { useEffect } from 'react'
import { useLoaderData } from 'react-router'
import { useNavigate } from 'react-router'
import { gql, useMutation } from '@apollo/client'

import {
  InvitationTokenQuery,
  SignUpWithInvitationTokenMutation,
} from '@/__generated__/graphql.js'
import { SignupInvitationForm } from '@/components/signup-form/signup-invitation-form.js'
import { formatErrorMessage } from '@/lib/error-message.js'

const SIGN_UP = gql`
  mutation SignUpWithInvitationToken(
    $name: String!
    $password: String!
    $invitationToken: String!
  ) {
    signupWithInvitation(
      input: {
        name: $name
        password: $password
        invitationToken: $invitationToken
      }
    ) {
      token
      user {
        id
        email
        name
      }
    }
  }
`

export function InvitationPage() {
  const {
    invitation: { token, email },
  } = useLoaderData<InvitationTokenQuery>()
  const [signUpMutation, { data, loading, error }] =
    useMutation<SignUpWithInvitationTokenMutation>(SIGN_UP)

  const navigate = useNavigate()

  useEffect(() => {
    if (data) {
      localStorage.setItem('token', data.signupWithInvitation.token)
      navigate('/dashboard/')
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <SignupInvitationForm
      onSubmit={variables => signUpMutation({ variables })}
      loading={loading}
      invitationToken={token}
      email={email}
      errorMessage={errorMessage}
    />
  )
}
