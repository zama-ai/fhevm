import { Meta, StoryObj } from '@storybook/react'
import { CreateApiKey } from './create-api-key'

const meta = {
  title: 'Molecules/CreateApiKey',
  component: CreateApiKey,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof CreateApiKey>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    dappId: 'dapp_1234',
  },
}
