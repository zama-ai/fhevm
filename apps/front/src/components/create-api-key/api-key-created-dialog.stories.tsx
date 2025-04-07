import { Meta, StoryObj } from '@storybook/react'
import { action } from '@storybook/addon-actions'
import { ApiKeyCreatedDialog } from './api-key-created-dialog'

const meta = {
  title: 'Molecules/CreateApiKey/Dialog',
  component: ApiKeyCreatedDialog,
  parameters: {
    layout: 'padded',
  },
  args: {
    onClose: action('on-close'),
  },
  tags: ['autodocs'],
} satisfies Meta<typeof ApiKeyCreatedDialog>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    token: 'pk_12345-APT',
  },
}
