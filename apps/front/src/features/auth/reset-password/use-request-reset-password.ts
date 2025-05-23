import {
  RequestResetPasswordInput,
  RequestResetPasswordMutation,
} from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'
import { useCallback, useState } from 'react'

export function useRequestResetPassword() {
  const [requestResetPassword, { loading, error }] = useMutation<
    RequestResetPasswordMutation,
    RequestResetPasswordInput
  >(REQUEST_RESET_PASSWORD)

  const [completed, setCompleted] = useState(false)

  const handleRequestResetPassword = useCallback(
    async ({ email }: { email: string }) => {
      try {
        await requestResetPassword({ variables: { email } })
        setCompleted(true)
      } catch {
        // ignore
      }
    },
    [requestResetPassword],
  )

  return {
    requestResetPassword: handleRequestResetPassword,
    loading,
    completed,
    error: error?.message,
  }
}

const REQUEST_RESET_PASSWORD = gql(`
  mutation RequestResetPassword($email: String!) {
    requestResetPassword(input: { email: $email })
  }
`)
