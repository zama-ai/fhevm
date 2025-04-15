import { Button, Dialog, Portal } from '@chakra-ui/react'

export type DeleteApiKeyDialogProps = {
  onCancel: () => void
  onConfirm: () => void
}

export function DeleteApiKeyDialog({
  onCancel,
  onConfirm,
}: DeleteApiKeyDialogProps) {
  return (
    <Dialog.Root role="alertdialog">
      <Portal>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title>Are you sure?</Dialog.Title>
            </Dialog.Header>
            <Dialog.Body>
              <p>
                This action cannot be undone. This will permanently delete your
                API Key.
              </p>
            </Dialog.Body>
            <Dialog.Footer>
              <Dialog.ActionTrigger asChild>
                <Button variant="outline" onClick={onCancel}>
                  Cancel
                </Button>
              </Dialog.ActionTrigger>
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
