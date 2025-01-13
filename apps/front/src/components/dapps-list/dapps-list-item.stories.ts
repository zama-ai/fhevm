import type { Meta, StoryObj } from '@storybook/react'
import { withRouter } from 'storybook-addon-remix-react-router'

import { DappsListItem } from './dapps-list-item'
import { Dapp, DappStatus } from '@/__generated__/graphql'

const meta = {
  title: 'Molecules/DappsListItem',
  component: DappsListItem,
  decorators: [withRouter],
  parameters: {
    layout: 'padded',
  },

  tags: ['autodocs', 'router', 'graphql'],
  argTypes: {
    dapp: {},
  },

  args: {
    dapp: Object.assign(
      {},
      { id: '1', name: 'Dapp name', status: DappStatus.Draft },
    ) as Dapp,
  },
} satisfies Meta<typeof DappsListItem>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    dapp: {
      id: '1',
      name: 'Dapp name',
      status: DappStatus.Draft,
    } as unknown as Dapp,
  },
}
