import { useQuery } from '@apollo/client'
import { useParams } from 'react-router'
import { Box, Card, Heading } from '@chakra-ui/react'
import { graphql } from '@/__generated__/gql'
import { GetDappQuery } from '@/__generated__/graphql'

const GET_DAPP = graphql(`
  query GetDapp($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      name
      status
    }
  }
`)

export function DappPage() {
  const { dappId } = useParams()
  const { data, loading, error } = useQuery<GetDappQuery>(GET_DAPP, {
    variables: { dappId },
  })

  return (
    <Box>
      <Heading>{data?.dapp.name}</Heading>
      <Card.Root>stats</Card.Root>
      <Card.Root>code</Card.Root>
    </Box>
  )
}
