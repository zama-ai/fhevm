import { ChakraProvider, defaultSystem } from '@chakra-ui/react'
import {
  ColorModeProvider,
  type ColorModeProviderProps,
} from './color-mode.tsx'

export function Provider(props: ColorModeProviderProps) {
  return (
    <ChakraProvider value={defaultSystem}>
      <ColorModeProvider {...props} />
    </ChakraProvider>
  )
}
