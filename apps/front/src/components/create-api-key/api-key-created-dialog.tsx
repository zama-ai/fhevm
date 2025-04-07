import { Alert, CloseButton, Dialog, Portal, Text } from '@chakra-ui/react'

export type ApiKeyCreatedDialogProps = {
  token: string
  onClose: () => void
}
export function ApiKeyCreatedDialog({
  token,
  onClose,
}: ApiKeyCreatedDialogProps) {
  return (
    <Dialog.Root lazyMount open onOpenChange={onClose}>
      <Portal>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content>
            <Dialog.Header>New API Key created</Dialog.Header>
            <Dialog.Body>
              <Alert.Root role="alert">
                <Alert.Indicator />
                <Alert.Title>
                  Make sure to copy your personal access token now. You won't be
                  able to see it again!
                </Alert.Title>
              </Alert.Root>
              <Text py="4">{token}</Text>
            </Dialog.Body>
            <Dialog.CloseTrigger asChild>
              <CloseButton size="sm" />
            </Dialog.CloseTrigger>
          </Dialog.Content>
        </Dialog.Positioner>
      </Portal>
    </Dialog.Root>
  )
}
