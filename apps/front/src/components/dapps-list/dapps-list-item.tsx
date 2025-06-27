import { Card, Text, Flex, LinkBox } from '@chakra-ui/react'
import { LinkOverlay } from '../ui/link.js'
import { formatDistanceToNow } from 'date-fns'

type OwnProps = {
  id: string
  name: string
  createdAt: number
}
export function DappsListItem({ id, name, createdAt }: OwnProps) {
  return (
    <LinkBox as="article">
      <Card.Root variant="outline">
        <Card.Header>
          <Flex justify="space-between">
            <LinkOverlay to={`/dapp/${id}`}>{name}</LinkOverlay>
          </Flex>
        </Card.Header>
        <Card.Body>
          <Text textStyle="sm" color="fg.muted">
            created {formatDistanceToNow(createdAt)} ago
          </Text>
        </Card.Body>
      </Card.Root>
    </LinkBox>
  )
}
