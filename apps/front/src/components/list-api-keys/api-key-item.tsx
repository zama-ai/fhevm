import { useCallback, useState } from 'react'
import { Table, IconButton } from '@chakra-ui/react'
import { Trash2 } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

import { SkeletonText } from '../ui/skeleton'
import { ConfirmDialog } from '../confirm-dialog/confirm-dialog'

export type ApiKeyItemProps = {
  id: string
  name: string
  description: string | null
  createdAt: number
  onDelete: (id: string) => void
}
export function ApiKeyItem({
  id,
  name,
  description,
  createdAt,
  onDelete,
}: ApiKeyItemProps) {
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
      <Table.Row key={id} role="listitem">
        <Table.Cell>
          {name} {description && `(${description})`}
        </Table.Cell>
        <Table.Cell>{formatDistanceToNow(createdAt)}</Table.Cell>
        <Table.Cell textAlign="end">
          <IconButton
            color="gray.400"
            size={'sm'}
            backgroundColor="transparent"
            aria-label="Delete API Key"
            onClick={handleDelete}
          >
            <Trash2 />
          </IconButton>
        </Table.Cell>
      </Table.Row>
    </>
  )
}

export function SkeletonApiKeyItem() {
  return (
    <Table.Row>
      <Table.Cell>
        <SkeletonText noOfLines={1} />
      </Table.Cell>
      <Table.Cell>
        <SkeletonText noOfLines={1} />
      </Table.Cell>
      <Table.Cell></Table.Cell>
    </Table.Row>
  )
}
