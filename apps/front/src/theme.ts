import {
  createSystem,
  defineConfig,
  defaultConfig,
  mergeConfigs,
} from '@chakra-ui/react'

import '@fontsource-variable/archivo/index.css'
import '@fontsource-variable/inter/index.css'

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
        body: { value: '"Inter Variable", system-ui, sans-serif' },
        heading: { value: '"Archivo Variable", system-ui, sans-serif' },
      },
    },
  },
})

export const system = createSystem(mergeConfigs(defaultConfig, config))
