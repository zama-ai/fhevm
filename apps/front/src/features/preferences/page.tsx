import { Heading, Separator } from '@chakra-ui/react'
import { useMutation, useQuery } from '@apollo/client'

import { graphql } from '../../__generated__/gql.js'
import {
  ChangeUserNameMutation,
  PreferencesQuery,
} from '@/__generated__/graphql.js'
import { SkeletonText } from '@/components/ui/skeleton.js'
import { DisplaySettings } from '@/components/display-settings/display-settings.js'
import { Account } from '@/components/account/account.js'
import { ChangePassword } from './change-password/change-password.js'

const GET_PREFERENCES = graphql(`
  query Preferences {
    me {
      id
      email
      name
      teams {
        id
        name
      }
    }
  }
`)

const UPDATE_PREFERENCES = graphql(`
  mutation ChangeUserName($id: ID!, $name: String!) {
    updateUser(input: { id: $id, name: $name }) {
      id
      name
    }
  }
`)

export function PreferencesPage() {
  const { loading, error, data } = useQuery<PreferencesQuery>(GET_PREFERENCES)
  const [updatePreferences, { loading: saving, error: mutationError }] =
    useMutation<ChangeUserNameMutation>(UPDATE_PREFERENCES)
  return (
    <>
      <Heading mb="5">Preferences</Heading>
      {loading ? (
        <SkeletonText noOfLines={4} gap="4" />
      ) : error ? (
        <p>Error: {error.message}</p>
      ) : !data ? (
        <p>No data</p>
      ) : (
        <Account
          email={data.me.email}
          name={data.me.name}
          loading={saving}
          errorMessage={mutationError?.message}
          onSubmit={({ name }) => {
            updatePreferences({
              variables: { id: data.me.id, name },
            })
          }}
        />
      )}
      <Separator my="9" />
      <ChangePassword />
      <Separator my="9" />
      <DisplaySettings />
    </>
  )
}
