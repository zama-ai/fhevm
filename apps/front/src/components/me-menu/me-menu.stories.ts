import type { Meta, StoryObj } from '@storybook/react'
import { fn } from '@storybook/test'

import { PureMeMenu } from './me-menu'

const meta = {
  title: 'Molecules/MeMenu',
  component: PureMeMenu,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'brand',
    },
  },
  tags: ['autodocs'],
  argTypes: {
    name: {
      control: 'text',
      description: 'The name of the user',
    },
    image: {
      control: 'text',
      description: 'Url to the image of the user',
    },
    condensed: { control: 'boolean' },
  },
  args: {
    onLogout: fn(),
  },
} satisfies Meta<typeof PureMeMenu>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    name: 'Jane Doe',
    image: 'https://picsum.photos/seed/plop/50/50',
    condensed: false,
    onLogout: fn(),
    onPreferences: fn(),
  },
}

export const Condensed: Story = {
  args: {
    ...Default.args,
    condensed: true,
  },
}
