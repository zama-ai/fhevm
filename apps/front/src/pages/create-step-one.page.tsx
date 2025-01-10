import { useContext, useEffect } from 'react'
import { useNavigate, useParams } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'
import { useMutation } from '@apollo/client'

import { graphql } from '../__generated__/gql.js'
import { CreateDappMutation } from '@/__generated__/graphql.js'
import { formatErrorMessage } from '@/lib/error-message.js'
import { CreatorName } from '@/components/creator/creator-name.js'
import { CreatorStepper } from '@/components/creator-stepper/creator-stepper.js'
import { TitleContext } from '@/components/title-context/title-context.js'
import { GET_ME } from '@/queries.js'

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
