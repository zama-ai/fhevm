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
        brand: {
          50: { value: '#aaaaaa' },
          100: { value: '#fffbec' },
          200: { value: '#fff5c5' }, // to validate
          300: { value: '#f9dc5c' }, // yellow highlight
          400: { value: 'pink' },
          /** primary brand color */
          500: { value: '#ffd208' },
          600: { value: '#aaaaaa' },
          700: { value: '#aaaaaa' },
          800: { value: '#aaaaaa' },
          /** good for .subtle on dark mode */
          900: { value: '#2b250f' },
          950: { value: '#aaaaaa' },
        },
        neutral: {
          50: { value: '#f9f9f9' },
          100: { value: '#f5f5f5' },
          200: { value: '#eeeeee' },
          300: { value: '#e0e0e0' },
          400: { value: '#bdbdbd' },
          500: { value: '#9e9e9e' },
          600: { value: '#757575' },
          700: { value: '#666666' },
          800: { value: '#aaaaaa' },
          900: { value: '#333333' },
          950: { value: '#222222' },
        },
        secondary: { value: '#f4b458', description: 'Secondary brand color' },
      },
      fonts: {
        body: { value: '"Inter Variable", system-ui, sans-serif' },
        heading: { value: '"Archivo Variable", system-ui, sans-serif' },
      },
    },
    semanticTokens: {
      colors: {
        highlight: {
          value: '{colors.brand.subtle}',
          description: 'Highlight color for brand color',
        },
        brand: {
          solid: { value: '{colors.brand.500}' },
          contrast: { value: '{colors.black}' },
          fg: { value: '{colors.brand.700}' },
          muted: { value: '{colors.brand.200}' },
          subtle: {
            value: { base: '{colors.brand.100}', _dark: '{colors.brand.900}' },
          },
          emphasized: { value: '{colors.brand.300}' },
          focusRing: { value: '{colors.brand.400}' },
        },
        neutral: {
          solid: { value: '{colors.neutral.500}' },
          contrast: { value: '{colors.black}' },
          fg: { value: '{colors.neutral.700}' },
          muted: { value: '{colors.neutral.200}' },
          subtle: {
            value: {
              base: '{colors.neutral.100}',
              _dark: '{colors.neutral.900}',
            },
          },
          emphasized: { value: '{colors.neutral.300}' },
          focusRing: { value: '{colors.neutral.400}' },
        },
      },
    },
  },
})

export const system = createSystem(mergeConfigs(defaultConfig, config))
