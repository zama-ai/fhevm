import {
  createSystem,
  defineConfig,
  defaultConfig,
  mergeConfigs,
} from '@chakra-ui/react'

/**
 * Don't forget to re-generate typegen after modifying this file.
 * This seems to be breaking something in chakra-ui.
 * @see https://chakra-ui.com/docs/theming/tokens
 */
const config = defineConfig({
  theme: {
    tokens: {
      colors: {
        brand: { value: '#FFD208' },
        secondary: { value: '#F4B458' },
      },
      fonts: {
        body: { value: 'Skia, system-ui, sans-serif' },
      },
    },
  },
})

export const system = createSystem(mergeConfigs(defaultConfig, config))
