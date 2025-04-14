import { useCallback, useState } from 'react'
import { Box, Card, IconButton, SkeletonCircle } from '@chakra-ui/react'
import { Trash2 } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

import { SkeletonText } from '../ui/skeleton'
import { ConfirmDialog } from '../confirm-dialog/confirm-dialog'

export type ApiKeyCardProps = {
  id: string
  name: string
  description: string | null
  createdAt: number
  onDelete: (id: string) => void
}
export function ApiKeyCard({
  id,
  name,
  description,
  createdAt,
  onDelete,
}: ApiKeyCardProps) {
  const [showDialog, setShowDialog] = useState(false)

  const handleDelete = useCallback(() => {
    setShowDialog(true)
  }, [setShowDialog])

  return (
    <>
      {showDialog && (
        <ConfirmDialog
          message="This action cannot be undone. This will permanently delete your
                API Key."
          onCancel={() => setShowDialog(false)}
          onConfirm={() => {
            setShowDialog(false)
            onDelete(id)
          }}
        />
      )}
      <Card.Root
        role="listitem"
        flexDirection="row"
        overflow="hidden"
        maxW="xl"
      >
        <Box flex="1">
          <Card.Body>
            <Card.Title mb="2">{name}</Card.Title>
            {description && <Card.Description>{description}</Card.Description>}
            created {formatDistanceToNow(createdAt)}
          </Card.Body>
        </Box>
        <Card.Footer alignContent="center">
          <IconButton
            color="red.700"
            backgroundColor="transparent"
            aria-label="Delete API Key"
            onClick={handleDelete}
          >
            <Trash2 />
          </IconButton>
        </Card.Footer>
      </Card.Root>
    </>
  )
}

export function SkeletonApiKeyCard() {
  return (
    <Card.Root role="listitem" flexDirection="row" overflow="hidden" maxW="xl">
      <Box flex="1">
        <Card.Body>
          <SkeletonText noOfLines={2} />
        </Card.Body>
      </Box>
      <Card.Footer alignContent="center">
        <SkeletonCircle size="10" />
      </Card.Footer>
    </Card.Root>
  )
}
