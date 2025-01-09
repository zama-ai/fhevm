import { ChakraProvider } from '@chakra-ui/react'
import { ColorModeProvider, type ColorModeProviderProps } from './color-mode.js'
import { system } from '@/theme.js'

export function UiProvider(props: ColorModeProviderProps) {
  return (
    <ChakraProvider value={system}>
      <ColorModeProvider {...props} />
    </ChakraProvider>
  )
}
