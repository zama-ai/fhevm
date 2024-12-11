import { useEffect, useState } from 'react'
import { useLoaderData, useNavigate } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'
import { useMutation } from '@apollo/client'

import { graphql } from '../__generated__/gql'
import { CreateDappMutation, MeTeamDappsQuery } from '@/__generated__/graphql'
import { formatErrorMessage } from '@/lib/error-message'
import { getPersonalTeam } from '@/lib/personal-team'
import { CreatorName } from '@/components/creator/creator-name'
import { CreatorStepper } from '@/components/creator-stepper/creator-stepper'

const CREATE_DAPP = graphql(`
  mutation CreateDapp($teamId: String!, $name: String!) {
    createDapp(input: { teamId: $teamId, name: $name }) {
      id
      name
      address
      status
    }
  }
`)

export function CreateStepOnePage() {
  const [teamId, setTeamId] = useState<string | null>(null)
  const [createDappMutation, { data, loading, error }] =
    useMutation<CreateDappMutation>(CREATE_DAPP)
  const { me } = useLoaderData<MeTeamDappsQuery>()
  const navigate = useNavigate()

  useEffect(() => {
    if (me) {
      setTeamId(getPersonalTeam(me.teams).id)
    }
  }, [me])

  useEffect(() => {
    if (data && data.createDapp.id) {
      navigate(`/create/2/${data.createDapp.id}`)
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <>
      <Heading mb="5">Create a new dApp</Heading>
      <Box display="flex" justifyContent="start" mb="5">
        <CreatorStepper currentStep={0} />
      </Box>
      <CreatorName
        onSubmit={({ name }) => {
          createDappMutation({
            variables: {
              teamId,
              name,
            },
          })
        }}
        loading={loading}
        errorMessage={errorMessage}
      />
    </>
  )
}
