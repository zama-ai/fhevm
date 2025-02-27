import { useContext, useEffect } from 'react'
import { useNavigate, useParams } from 'react-router'
import { Heading } from '@chakra-ui/react'
import { useMutation, useLazyQuery } from '@apollo/client'

import { graphql } from '../__generated__/gql.js'
import {
  CreateDappMutation,
  ValidateAddressQuery,
} from '@/__generated__/graphql.js'
import { formatErrorMessage } from '@/lib/error-message.js'
import { CreatorName } from '@/components/creator/creator-name.js'
import { TitleContext } from '@/components/title-context/title-context.js'
import { GET_ME } from '@/queries.js'

// TODO https://codesandbox.io/p/sandbox/apollo-3-playground-3-5-x-ryhg3x?file=%2Fsrc%2FApp.js%3A26%2C9-26%2C19

const VALIDATE_ADDRESS = graphql(`
  query ValidateAddress($chainId: String!, $address: String!) {
    validateAddress(input: { chainId: $chainId, address: $address }) {
      check
      message
    }
  }
`)

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
  const { teamId } = useParams()

  const [createDappMutation, { loading, error }] =
    useMutation<CreateDappMutation>(CREATE_DAPP, {
      refetchQueries: [GET_ME],
      onCompleted(data) {
        navigate(`/create/2/${data?.createDapp.id}`)
      },
    })

  const [
    validateAddressQuery,
    { data: addressData, loading: addressLoading, error: addressError },
  ] = useLazyQuery<ValidateAddressQuery>(VALIDATE_ADDRESS)

  const navigate = useNavigate()
  const { setTitle } = useContext(TitleContext)

  // Reset the title in context when the component is unmounted
  useEffect(() => {
    return () => {
      setTitle('')
    }
  }, [setTitle])

  const errorMessage = error ? formatErrorMessage(error.message) : undefined
  return (
    <>
      <Heading mb="5">Create a new dApp</Heading>
      <CreatorName
        onValidateAddress={({ chainId, address }) => {
          console.log('address', address)
          validateAddressQuery({
            variables: {
              chainId,
              address,
            },
          })
        }}
        addressLoading={addressLoading}
        addressError={
          addressError
            ? formatErrorMessage(addressError.message)
            : addressData?.validateAddress.message
              ? addressData.validateAddress.message
              : ''
        }
        onSubmit={({ name }) => {
          createDappMutation({
            variables: {
              teamId,
              name,
            },

            onCompleted: data => {
              navigate(`/create/2/${data.createDapp.id}`)
            },
          })
        }}
        onUpdateTitle={setTitle}
        loading={loading}
        errorMessage={errorMessage}
      />
    </>
  )
}
