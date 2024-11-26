import { createSystem, defineConfig, defaultSystem } from '@chakra-ui/react'

/**
 * Don't forget to re-generate typegen after modifying this file.
 * @see https://chakra-ui.com/docs/theming/tokens
 */
const config = defineConfig({
  theme: {
    tokens: {
      ...defaultSystem.tokens,
      colors: {
        primary: { value: '#FFD208' },
        secondary: { value: '#F4B458' },
      },
      fonts: {
        body: { value: 'system-ui, sans-serif' },
      },
    },
  },
})

export const system = createSystem(config)
