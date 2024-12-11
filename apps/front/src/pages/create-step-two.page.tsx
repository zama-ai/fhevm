import { useEffect } from 'react'
import { useNavigate, useParams } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'
import { useMutation } from '@apollo/client'

import { graphql } from '../__generated__/gql'
import { SetDappAddressMutation } from '@/__generated__/graphql'
import { formatErrorMessage } from '@/lib/error-message'
import { CreatorAddress } from '@/components/creator/creator-address'
import { CreatorStepper } from '@/components/creator-stepper/creator-stepper'

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

export function CreateStepTwoPage() {
  const navigate = useNavigate()
  const { dappId } = useParams()
  const [setDappAddressMutation, { data, loading, error }] =
    useMutation<SetDappAddressMutation>(SET_DAPP_ADDRESS)

  useEffect(() => {
    if (data && data.updateDapp.id) {
      navigate(`/create/2/${data.updateDapp.id}`)
    }
  }, [data, navigate])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <>
      <Heading mb="5">Create a new dApp</Heading>
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
          })
          navigate(`/create/3/${dappId}`)
        }}
        loading={loading}
        errorMessage={errorMessage}
      />
    </>
  )
}
