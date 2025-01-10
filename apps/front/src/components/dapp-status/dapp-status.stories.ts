import type { Meta, StoryObj } from '@storybook/react'

import { DappStatus } from './dapp-status'

const meta = {
  title: 'Atoms/DappStatus',
  component: DappStatus,
  parameters: {
    layout: 'centered',
  },

  tags: ['autodocs'],

  argTypes: {
    status: {
      options: ['DRAFT', 'DEPLOYING', 'LIVE'],
      control: 'select',
    },
  },
} satisfies Meta<typeof DappStatus>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    status: 'LIVE',
  },
}
