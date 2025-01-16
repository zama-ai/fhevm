import { Container } from '@chakra-ui/react'
import * as React from 'react'
import { StoryObj } from '@storybook/react'
import { Sparkline } from './sparkline'

function generateRandomData(length: number) {
  const data = []

  for (let i = 0; i < length; i++) {
    const brownNoise = (previous: number) => {
      const change = Math.random() * 2 - 1
      return previous + change
    }

    const previousValue: number =
      data.length > 0 ? data[data.length - 1].value : 500
    const previousCompareValue: number =
      data.length > 0 ? data[data.length - 1].compareValue : 500

    data.push({
      value: Math.max(0, brownNoise(previousValue)),
      compareValue: Math.max(0, brownNoise(previousCompareValue)),
    })
  }

  return data
}

export default {
  title: 'Atoms/Visualization/Sparkline',
  component: Sparkline,
  decorators: [
    (Story: React.ComponentType) => (
      <Container maxW="container.xl">
        <Story />
      </Container>
    ),
  ],
}

type Story = StoryObj<typeof Sparkline>

export const Basic: Story = {
  args: {
    data: generateRandomData(300),
    categories: ['value', 'compareValue'],
    curveType: 'natural',
    colors: ['orange', 'gray'],
    strokeWidth: 1,
    variant: 'gradient',
    connectNulls: true,
    stack: false,
  },
  render: args => {
    return <Sparkline {...args} height="60px" mx="-4" />
  },
}
