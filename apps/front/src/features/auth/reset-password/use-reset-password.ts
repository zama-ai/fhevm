import {
  ResetPasswordInput,
  ResetPasswordMutation,
} from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'
import { useCallback, useEffect } from 'react'
import { useNavigate } from 'react-router'

export function useResetPassword(token: string) {
  const [resetPasswordMutation, { data, loading, error }] = useMutation<
    ResetPasswordMutation,
    ResetPasswordInput
  >(RESET_PASSWORD)

  const navigate = useNavigate()

  const resetPassword = useCallback(
    async ({ password }: { password: string }) => {
      try {
        await resetPasswordMutation({
          variables: { token, password },
        })
      } catch {
        // ignore
      }
    },
    [resetPasswordMutation, token],
  )

  useEffect(() => {
    if (data) {
      localStorage.setItem('token', data.resetPassword.token)
      navigate('/dashboard/')
    }
  }, [data, navigate])

  return {
    resetPassword,
    loading,
    error: error?.message,
  }
}

const RESET_PASSWORD = gql(`
  mutation ResetPassword($token: String!, $password: String!) {
    resetPassword(input: { token: $token, password: $password }) {
      token
    }
  }
`)
