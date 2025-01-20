import { Text } from '@chakra-ui/react'
import { ColorModeButton } from '@/components/ui/color-mode-button.js'
import { Heading } from '@chakra-ui/react'

export function DisplaySettings() {
  return (
    <>
      <Heading size="md">Display Settings</Heading>
      <Text>
        Set mode <ColorModeButton size="lg" />
      </Text>
    </>
  )
}
