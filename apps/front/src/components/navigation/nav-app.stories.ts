import type { Meta, StoryObj } from '@storybook/react'

import { PureNavApp } from './nav-app'

const meta = {
  title: 'Atoms/NavApp',
  component: PureNavApp,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    color: { control: 'color' },
    name: { control: 'text' },
    isActive: { control: 'boolean' },
  },
} satisfies Meta<typeof PureNavApp>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    color: 'purple',
    name: 'Blind Auction',
    isActive: false,
  },
}

export const Active: Story = {
  args: {
    ...Default.args,
    isActive: true,
  },
}
