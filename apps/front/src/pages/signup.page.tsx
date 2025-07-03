import { useEffect } from 'react'
import { useNavigate } from 'react-router'
import { gql, useMutation } from '@apollo/client'

import { SignUpMutation } from '@/__generated__/graphql.js'
import { SignupForm } from '@/components/signup-form/signup-form.js'
import { formatErrorMessage } from '@/lib/error-message.js'

const SIGN_UP = gql`
  mutation SignUp($name: String!, $password: String!, $email: String!) {
    signup(input: { name: $name, password: $password, email: $email }) {
      user {
        id
        email
        name
      }
    }
  }
`

export function SignupPage() {
  const [signUpMutation, { data, loading, error }] =
    useMutation<SignUpMutation>(SIGN_UP)

  const navigate = useNavigate()

  useEffect(() => {
    if (data) {
      navigate('/check-email/')
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <SignupForm
      onSubmit={variables => signUpMutation({ variables })}
      loading={loading}
      errorMessage={errorMessage}
    />
  )
}
