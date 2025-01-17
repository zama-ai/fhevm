import type { Meta, StoryObj } from '@storybook/react'
import { withRouter } from 'storybook-addon-remix-react-router'

import { DappsListItem } from './dapps-list-item'
import { DappStatus } from '@/__generated__/graphql'

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
    status: {
      control: { type: 'select', options: DappStatus },
    },
    createdAt: {
      control: { type: 'number' },
    },
  },

  args: { id: '1', name: 'Dapp name', status: DappStatus.Draft },
} satisfies Meta<typeof DappsListItem>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    id: '1',
    name: 'Dapp name',
    status: DappStatus.Draft,
    createdAt: 1630000000000,
  },
}
