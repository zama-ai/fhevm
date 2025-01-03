import { Card, Text, Flex, LinkBox } from '@chakra-ui/react'
import { DappStatus } from '../dapp-status/dapp-status'
import { Dapp } from '@/__generated__/graphql'
import { LinkOverlay } from '../ui/link'

type OwnProps = {
  dapp: Dapp
}
export function DappsListItem({ dapp }: OwnProps) {
  const lastmodified = 'Created some time ago'
  const link =
    dapp.status === 'DRAFT' ? `/create/2/${dapp.id}` : `/dapp/${dapp.id}`
  return (
    <LinkBox as="article">
      <Card.Root variant="outline">
        <Card.Header>
          <Flex justify="space-between">
            <LinkOverlay to={link}>{dapp.name}</LinkOverlay>
            <DappStatus status={dapp.status} />
          </Flex>
        </Card.Header>
        <Card.Body>
          <Text textStyle="sm" color="fg.muted">
            {lastmodified}
          </Text>
        </Card.Body>
      </Card.Root>
    </LinkBox>
  )
}
