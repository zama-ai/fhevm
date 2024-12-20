import { Card, Text, Flex } from '@chakra-ui/react'
import { DappStatus } from '../dapp-status/dapp-status'

type OwnProps = {
  dapp: {
    id: string
    name: string
    status: string
    createdAt: string
    updatedAt: string
  }
}
export function DappsListItem({ dapp }: OwnProps) {
  let lastmodified = `Created ${dapp.createdAt}`
  if (dapp.updatedAt !== dapp.createdAt) {
    lastmodified = `Updated ${dapp.updatedAt}`
  }
  if (dapp.status === 'LIVE') {
    lastmodified = `Deployed ${dapp.updatedAt}`
  }

  return (
    <Card.Root variant="outline">
      <Card.Header>
        <Flex justify="space-between">
          {dapp.name}
          <DappStatus status={dapp.status} />
        </Flex>
      </Card.Header>
      <Card.Body>
        <Text textStyle="sm" color="fg.muted">
          {lastmodified}
        </Text>
      </Card.Body>
    </Card.Root>
  )
}
