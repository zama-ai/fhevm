import type { Meta, StoryObj } from '@storybook/react'
import { withRouter } from 'storybook-addon-remix-react-router'

import { DappsListItem } from './dapps-list-item'

const meta = {
  title: 'Molecules/DappsListItem',
  component: DappsListItem,
  decorators: [withRouter],
  parameters: {
    layout: 'padded',
  },

  tags: ['autodocs', 'router', 'graphql'],
  argTypes: {
    id: {
      control: { type: 'text' },
    },
    name: {
      control: { type: 'text' },
    },
    createdAt: {
      control: { type: 'number' },
    },
  },

  args: { id: '1', name: 'Dapp name' },
} satisfies Meta<typeof DappsListItem>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    id: '1',
    name: 'Dapp name',
    createdAt: 1630000000000,
  },
}
