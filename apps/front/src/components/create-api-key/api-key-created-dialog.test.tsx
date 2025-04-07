import { system } from '@/theme'
import { ChakraProvider } from '@chakra-ui/react'
import { faker } from '@faker-js/faker'
import { render, screen } from '@testing-library/react'
import { beforeEach, describe, expect, Mocked, test, vi } from 'vitest'
import {
  ApiKeyCreatedDialog,
  ApiKeyCreatedDialogProps,
} from './api-key-created-dialog'
import userEvent from '@testing-library/user-event'

describe('ApiKeyCreatedDialog', () => {
  let token: string
  let onClose: Mocked<ApiKeyCreatedDialogProps['onClose']>
  beforeEach(() => {
    token = faker.string.uuid()
    onClose = vi.fn()
    render(
      <ChakraProvider value={system}>
        <ApiKeyCreatedDialog token={token} onClose={onClose} />
      </ChakraProvider>,
    )
  })

  test('should render a dialog', () => {
    expect(screen.getByRole('dialog')).toBeInTheDocument()
  })

  test('should show the token code', () => {
    expect(screen.getByText(token)).toBeInTheDocument()
  })

  test('should show an alert', () => {
    const alert = screen.getByRole('alert')
    expect(alert).toBeInTheDocument()
    expect(alert).toHaveTextContent('copy your personal access token')
  })

  test('should show a close button', async () => {
    const btn = screen.getByRole('button')
    expect(btn).toBeInTheDocument()
    const user = userEvent.setup()
    await user.click(btn)
    expect(onClose).toHaveBeenCalled()
  })
})
