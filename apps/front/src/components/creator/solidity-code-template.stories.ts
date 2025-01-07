import type { Meta, StoryObj } from '@storybook/react'

import { SolidityCodeTemplate } from './solidity-code-template'

const meta = {
  title: 'Molecules/SolidityCodeTemplate',
  component: SolidityCodeTemplate,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof SolidityCodeTemplate>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {}
