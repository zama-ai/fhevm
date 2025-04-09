import { UiProvider } from '@/providers/ui'
import { render, screen } from '@testing-library/react'
import { beforeEach, describe, expect, Mocked, test, vi } from 'vitest'
import { ConfirmDialog, ConfirmDialogProps } from './confirm-dialog'
import { faker } from '@faker-js/faker'
import userEvent from '@testing-library/user-event'

describe('ConfirmDialog', () => {
  let title: string
  let message: string
  let onConfirm: Mocked<ConfirmDialogProps['onConfirm']>
  let onCancel: Mocked<ConfirmDialogProps['onCancel']>

  beforeEach(() => {
    title = faker.lorem.words(3)
    message = faker.lorem.paragraph()
    onConfirm = vi.fn()
    onCancel = vi.fn()

    render(
      <UiProvider>
        <ConfirmDialog
          title={title}
          message={message}
          onCancel={onCancel}
          onConfirm={onConfirm}
        />
      </UiProvider>,
    )
  })

  test('should work', async () => {
    expect(await screen.findByRole('alertdialog')).toBeInTheDocument()
  })

  test('should show the title', async () => {
    expect(await screen.findByText(title)).toBeInTheDocument()
  })

  test('should show the message', async () => {
    expect(await screen.findByText(message)).toBeInTheDocument()
  })

  test('should show the cancel button', async () => {
    expect(
      await screen.findByRole('button', { name: /cancel/i }),
    ).toBeInTheDocument()
  })

  test('should show the confirm button', async () => {
    expect(
      await screen.findByRole('button', { name: /confirm/i }),
    ).toBeInTheDocument()
  })

  describe('when the user click on cancel', () => {
    beforeEach(async () => {
      const user = userEvent.setup()
      await user.click(await screen.findByRole('button', { name: /cancel/i }))
    })

    test('then it calls the onCancel callback', () => {
      expect(onCancel).toHaveBeenCalled()
    })
  })

  describe('when the user click on confirm', () => {
    beforeEach(async () => {
      const user = userEvent.setup()
      await user.click(await screen.findByRole('button', { name: /confirm/i }))
    })

    test('then it calls the onConfirm callback', () => {
      expect(onConfirm).toHaveBeenCalled()
    })
  })
})
