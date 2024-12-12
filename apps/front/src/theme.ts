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
        brand: { value: '#ffd208', description: 'Primary brand color' },
        yellowhighlight: {
          value: '#f9dc5c',
          description: 'Highlight on brand color',
        },
        secondary: { value: '#f4b458', description: 'Secondary brand color' },
        ivory: {
          value: 'rgba(255, 210, 8, .1)',
          description: 'Light yellow tint',
        },
      },
      fonts: {
        body: { value: '"Inter Variable", system-ui, sans-serif' },
        heading: { value: '"Archivo Variable", system-ui, sans-serif' },
      },
    },
  },
})

export const system = createSystem(mergeConfigs(defaultConfig, config))
