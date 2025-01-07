import { ChakraProvider } from '@chakra-ui/react'
import {
  ColorModeProvider,
  type ColorModeProviderProps,
} from './color-mode.tsx.js'
import { system } from '#theme.ts.js'

export function UiProvider(props: ColorModeProviderProps) {
  return (
    <ChakraProvider value={system}>
      <ColorModeProvider {...props} />
    </ChakraProvider>
  )
}
