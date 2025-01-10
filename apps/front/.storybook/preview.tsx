import React from 'react'
import { ChakraProvider } from '@chakra-ui/react'
import { withThemeByClassName } from '@storybook/addon-themes'
import type { Preview } from '@storybook/react'
import { system } from '../src/theme'

const preview: Preview = {
  globalTypes: {
    theme: {
      description: 'Global theme for components',
      toolbar: {
        // The label to show for this toolbar item
        // title: 'Dark / Light mode',
        icon: 'moon',
        // Array of plain string values or MenuItem shape (see below)
        items: ['light', 'dark'],
        // Change title based on selected value
        // dynamicTitle: true,
      },
    },
  },
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
    backgrounds: {
      values: [
        { name: 'dark', value: '#333' },
        { name: 'light', value: '#f8f8f8' },
        { name: 'brand', value: '#ffd208' },
      ],
      default: 'Light',
    },
  },
  decorators: [
    Story => (
      <ChakraProvider value={system}>
        <Story />
      </ChakraProvider>
    ),
    withThemeByClassName({
      defaultTheme: 'dark',
      themes: { light: '', dark: 'dark' },
    }),
  ],
}

export default preview
