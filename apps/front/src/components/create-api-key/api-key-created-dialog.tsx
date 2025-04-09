import {
  Alert,
  CloseButton,
  Dialog,
  Flex,
  Portal,
  Text,
} from '@chakra-ui/react'
import { ClipboardButton, ClipboardRoot } from '../ui/clipboard'

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
              <Alert.Root role="alert" mb="4">
                <Alert.Indicator />
                <Alert.Title>
                  Make sure to copy your personal access token now. You won't be
                  able to see it again!
                </Alert.Title>
              </Alert.Root>
              <Flex py="2" alignItems="center">
                <Text flex="1">{token}</Text>
                <ClipboardRoot value={token}>
                  <ClipboardButton />
                </ClipboardRoot>
              </Flex>
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
