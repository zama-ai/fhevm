import { useEffect } from 'react'
import { useNavigate } from 'react-router'
import { gql, useMutation } from '@apollo/client'

import { SignInMutation } from '@/__generated__/graphql'
import { SigninForm } from '@/components/signin-form/signin-form'

const SIGN_IN = gql`
  mutation SignIn($email: String!, $password: String!) {
    login(input: { email: $email, password: $password }) {
      token
      user {
        id
        email
        teams {
          id
        }
      }
    }
  }
`

export function SigninPage() {
  const [signInMutation, { data, loading, error }] =
    useMutation<SignInMutation>(SIGN_IN)
  const navigate = useNavigate()

  useEffect(() => {
    if (data) {
      localStorage.setItem('token', data.login.token)
      navigate('/dashboard/')
    }
  }, [data, navigate])

  // TODO: tweak when backend returns proper GraphqlError messages
  const errorMessage = error?.message.replace('GraphQL error: ', '')
  return (
    <SigninForm
      onSubmit={variables => {
        signInMutation({ variables })
      }}
      loading={!!loading}
      errorMessage={errorMessage}
    />
  )
}
