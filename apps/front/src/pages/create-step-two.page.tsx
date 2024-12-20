import { useNavigate, useParams } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'
import { useMutation } from '@apollo/client'

import { graphql } from '../__generated__/gql'
import {
  SetDappAddressMutation,
  DeployDappMutation,
} from '@/__generated__/graphql'
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

const DEPLOY_DAPP = graphql(`
  mutation DeployDapp($applicationId: String!) {
    deployDapp(input: { dappId: $applicationId }) {
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
  const [
    deployDappMutation,
    { loading: deployDappLoading, error: deployDappError },
  ] = useMutation<DeployDappMutation>(DEPLOY_DAPP, {
    variables: { applicationId: dappId },
    onCompleted() {
      navigate(`/create/3/${dappId}`)
    },
  })
  const [
    setDappAddressMutation,
    { loading: setAddressLoading, error: setAddressError },
  ] = useMutation<SetDappAddressMutation>(SET_DAPP_ADDRESS, {
    onCompleted() {
      deployDappMutation()
    },
  })

  const error = setAddressError ?? deployDappError
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
        }}
        loading={setAddressLoading || deployDappLoading}
        errorMessage={errorMessage}
      />
    </>
  )
}
