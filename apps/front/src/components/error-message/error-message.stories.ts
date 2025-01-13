import type { Meta, StoryObj } from '@storybook/react'

import { ErrorMessage } from './error-message'

const meta = {
  title: 'Atoms/ErrorMessage',
  component: ErrorMessage,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    children: { control: 'text' },
  },
} satisfies Meta<typeof ErrorMessage>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    children: 'Error Message',
  },
}
