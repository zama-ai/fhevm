import type { Meta, StoryObj } from '@storybook/react'

import { Logo } from './logo'

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
  title: 'Atoms/Logo',
  component: Logo,
  parameters: {
    layout: 'centered',
  },
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs'],
  argTypes: {
    width: {
      control: 'number',
    },
    height: {},
  },
} satisfies Meta<typeof Logo>

export default meta
type Story = StoryObj<typeof meta>

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Primary: Story = {}

export const Standard: Story = {
  args: {
    width: 100,
  },
}

export const Small: Story = {
  args: {
    width: 50,
  },
}

export const Custom: Story = {
  args: {
    height: 100,
  },
}
