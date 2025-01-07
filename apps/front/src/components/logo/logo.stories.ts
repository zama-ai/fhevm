import type { Meta, StoryObj } from '@storybook/react'

import { Logo } from './logo'

const meta = {
  title: 'Atoms/Logo',
  component: Logo,
  parameters: {
    layout: 'centered',
  },
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

export const Default: Story = {}

export const Width: Story = {
  args: {
    width: 50,
  },
}

export const Height: Story = {
  args: {
    height: 300,
  },
}
