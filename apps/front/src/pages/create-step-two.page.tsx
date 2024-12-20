import { useEffect } from 'react'
import { useNavigate, useParams } from 'react-router'
import { Heading, Box, Stack, Skeleton } from '@chakra-ui/react'
import { useMutation, useQuery } from '@apollo/client'

import { graphql } from '../__generated__/gql'
import { GetDappQuery, SetDappAddressMutation } from '@/__generated__/graphql'
import { formatErrorMessage } from '@/lib/error-message'
import { CreatorAddress } from '@/components/creator/creator-address'
import { CreatorStepper } from '@/components/creator-stepper/creator-stepper'
import { apolloClient } from '@/providers/apollo'
import { DappStatus } from '@/components/dapp-status/dapp-status'

const SET_DAPP_ADDRESS = graphql(`
  mutation SetDappAddress($id: ID!, $address: String!) {
    updateDapp(input: { id: $id, address: $address }) {
      id
      name
      address
      status
    }
  }
`)

const GET_DAPP = graphql(`
  query GetDapp($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      name
      status
    }
  }
`)

export function CreateStepTwoPage() {
  const navigate = useNavigate()
  const { dappId } = useParams()
  const { data: loadData } = useQuery<GetDappQuery>(GET_DAPP, {
    variables: { dappId },
  })
  const [setDappAddressMutation, { data, loading, error }] =
    useMutation<SetDappAddressMutation>(SET_DAPP_ADDRESS)

  useEffect(() => {
    if (data?.updateDapp.id) {
      navigate(`/create/2/${data.updateDapp.id}`)
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <>
      {loadData ? (
        <Stack direction="row" align="center">
          <Heading my="5">{loadData.dapp.name}</Heading>
          <DappStatus status={loadData.dapp.status} ml="2" size="xs" />
        </Stack>
      ) : (
        <Skeleton height="5" my="5" width="30rem" />
      )}
      <Box display="flex" justifyContent="start" mb="5">
        <CreatorStepper currentStep={1} />
      </Box>

      <CreatorAddress
        onSubmit={({ address }) => {
          setDappAddressMutation({
            variables: {
              id: dappId,
              address,
            },
            onCompleted: () => {
              apolloClient.refetchQueries({
                include: ['MeTeamDapps'],
                // include: [MeDocument.definitions[0].name.value],
              })
            },
          })
          navigate(`/create/3/${dappId}`)
        }}
        loading={loading}
        errorMessage={errorMessage}
      />
    </>
  )
}
