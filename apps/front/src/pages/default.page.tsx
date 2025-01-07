import { HStack, Separator, Text } from '@chakra-ui/react'
import { ColorModeButton } from '#components/ui/color-mode-button.js'
import { Link } from '#components/ui/link.js'
export function DefaultPage() {
  return (
    <HStack gap="4">
      <Text>
        <Link to="/signin">signin</Link>
      </Text>
      <Separator orientation="vertical" height="4" />
      <Text>
        <Link to="/signup/e7e720ef-e8d6-4e07-883d-ed93ea7a6999">signup</Link>
      </Text>
      <Separator orientation="vertical" height="4" />
      <Text as="div">
        Set mode <ColorModeButton size="lg" />
      </Text>
    </HStack>
  )
}
