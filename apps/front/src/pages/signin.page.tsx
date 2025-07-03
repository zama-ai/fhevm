import { useEffect } from 'react'
import { useNavigate } from 'react-router'
import { gql, useMutation } from '@apollo/client'

import { SignInMutation } from '@/__generated__/graphql.js'
import { SigninForm } from '@/components/signin-form/signin-form.js'
import { formatErrorMessage } from '@/lib/error-message.js'
import { Link } from '@/components/ui/link'
import { HStack, Text } from '@chakra-ui/react'
import { FeatureFlag } from '@/components/feature-flag/feature-flag'

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

  // TODO: move this to reusable function
  useEffect(() => {
    if (data) {
      localStorage.setItem('token', data.login.token)
      navigate('/dashboard/')
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <>
      <SigninForm
        onSubmit={variables => signInMutation({ variables })}
        loading={!!loading}
        errorMessage={errorMessage}
      />
      <HStack justifyContent="space-between">
        <Text textStyle="sm">
          <Link to="/reset-password">Forgot password?</Link>
        </Text>
        <FeatureFlag not is="invitations">
          <Text textStyle="sm">
            <Link to="/signup">Sign up</Link>
          </Text>
        </FeatureFlag>
      </HStack>
    </>
  )
}
