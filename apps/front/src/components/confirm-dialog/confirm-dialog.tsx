import { Button, Dialog, Portal } from '@chakra-ui/react'

export type ConfirmDialogProps = {
  title?: string
  message: string
  onCancel: () => void
  onConfirm: () => void
}

export function ConfirmDialog({
  title = 'Are you sure?',
  message,
  onCancel,
  onConfirm,
}: ConfirmDialogProps) {
  return (
    <Dialog.Root lazyMount open role="alertdialog">
      <Portal>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title>{title}</Dialog.Title>
            </Dialog.Header>
            <Dialog.Body>
              <p>{message}</p>
            </Dialog.Body>
            <Dialog.Footer>
              <Button variant="outline" onClick={onCancel}>
                Cancel
              </Button>
              <Button colorPalette="red" onClick={onConfirm}>
                Confirm
              </Button>
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog.Positioner>
      </Portal>
    </Dialog.Root>
  )
}
