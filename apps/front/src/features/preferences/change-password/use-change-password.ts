import {
  ChangePasswordMutation,
  ChangePasswordMutationVariables,
} from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'
import { useCallback } from 'react'

export function useChangePassword() {
  const [changePasswordMutation, { data, loading, error }] = useMutation<
    ChangePasswordMutation,
    ChangePasswordMutationVariables
  >(CHANGE_PASSWORD)

  const changePassword = useCallback(
    async (variables: { oldPassword: string; newPassword: string }) => {
      try {
        await changePasswordMutation({
          variables,
        })
      } catch {
        // ignore
      }
    },
    [changePasswordMutation],
  )

  return {
    changePassword,
    loading,
    updated: data?.changePassword,
    error: error?.message,
  }
}

const CHANGE_PASSWORD = gql(`
  mutation ChangePassword($oldPassword: String!, $newPassword: String!) {
    changePassword(input: {
      oldPassword: $oldPassword, 
      newPassword: $newPassword
    })
  }`)
