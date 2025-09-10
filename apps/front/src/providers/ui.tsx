import { ChakraProvider } from '@chakra-ui/react'
import { system } from '@/theme'
import { ColorModeProvider, type ColorModeProviderProps } from './color-mode'

export function UiProvider(props: ColorModeProviderProps) {
  return (
    <ChakraProvider value={system}>
      <ColorModeProvider {...props}>{props.children}</ColorModeProvider>
    </ChakraProvider>
  )
}
