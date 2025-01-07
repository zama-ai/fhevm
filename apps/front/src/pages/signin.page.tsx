import { useEffect } from 'react'
import { useNavigate } from 'react-router'
import { gql, useMutation } from '@apollo/client'

import { SignInMutation } from '#__generated__/graphql.js'
import { SigninForm } from '#components/signin-form/signin-form.js'
import { formatErrorMessage } from '#lib/error-message.js'

const SIGN_IN = gql`
  mutation SignIn($email: String!, $password: String!) {
    login(input: { email: $email, password: $password }) {
      token
      user {
        id
        email
        name
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

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <SigninForm
      onSubmit={variables => signInMutation({ variables })}
      loading={!!loading}
      errorMessage={errorMessage}
    />
  )
}
