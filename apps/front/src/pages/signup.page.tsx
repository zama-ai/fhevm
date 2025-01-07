import { useEffect } from 'react'
import { useLoaderData } from 'react-router'
import { useNavigate } from 'react-router'
import { gql, useMutation } from '@apollo/client'

import { InvitationTokenQuery, SignUpMutation } from '#__generated__/graphql.js'
import { SignupForm } from '#components/signup-form/signup-form.js'
import { formatErrorMessage } from '#lib/error-message.js'

const SIGN_UP = gql`
  mutation SignUp(
    $name: String!
    $password: String!
    $invitationToken: String!
  ) {
    signup(
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

export function SignupPage() {
  const {
    invitation: { token, email },
  } = useLoaderData<InvitationTokenQuery>()
  const [signUpMutation, { data, loading, error }] =
    useMutation<SignUpMutation>(SIGN_UP)

  const navigate = useNavigate()

  useEffect(() => {
    if (data) {
      localStorage.setItem('token', data.signup.token)
      navigate('/dashboard/')
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <SignupForm
      onSubmit={variables => signUpMutation({ variables })}
      loading={loading}
      invitationToken={token}
      email={email}
      errorMessage={errorMessage}
    />
  )
}
