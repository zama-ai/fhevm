import { ChakraProvider } from '@chakra-ui/react'
import { faker } from '@faker-js/faker'
import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, Mocked, test, vi } from 'vitest'

import { system } from '@/theme'

import { ApiKeyForm, ApiKeyFormProps } from './api-key-form'

describe('ApiKeyForm', () => {
  describe('inputs', () => {
    beforeEach(() => {
      render(
        <ChakraProvider value={system}>
          <ApiKeyForm onCreate={vi.fn()} />
        </ChakraProvider>,
      )
    })

    test('should render a form', () => {
      expect(screen.getByRole('form')).toBeInTheDocument()
    })

    test('should have a name input', () => {
      expect(screen.getByLabelText(/name/i)).toBeInTheDocument()
    })

    test('should have a description input', () => {
      expect(screen.getByLabelText(/description/i)).toBeInTheDocument()
    })

    test('should have a button', () => {
      expect(screen.getByRole('button')).toBeInTheDocument()
    })
  })

  describe('actions', () => {
    let onCreate: Mocked<ApiKeyFormProps['onCreate']>
    beforeEach(() => {
      onCreate = vi.fn()
      render(
        <ChakraProvider value={system}>
          <ApiKeyForm onCreate={onCreate} />
        </ChakraProvider>,
      )
    })

    describe('when the form is compiled correctly', () => {
      let name: string
      let description: string

      beforeEach(async () => {
        name = faker.internet.displayName()
        description = faker.lorem.sentence()

        const user = userEvent.setup()
        await user.type(screen.getByLabelText(/name/i), name)
        await user.type(screen.getByLabelText(/description/i), description)
        await user.click(screen.getByRole('button'))
      })

      test('then it call onCreate', () => {
        expect(onCreate).toHaveBeenCalledExactlyOnceWith({ name, description })
      })

      test('then name should be fine', () => {
        expect(
          within(screen.getByLabelText(/name/i)).queryByRole('alert'),
        ).not.toBeInTheDocument()
      })

      test('then description should be fine', () => {
        expect(
          within(screen.getByLabelText(/description/i)).queryByRole('alert'),
        ).not.toBeInTheDocument()
      })
    })

    describe('when name is missing', () => {
      beforeEach(async () => {
        const user = userEvent.setup()
        await user.type(
          screen.getByLabelText(/description/i),
          faker.lorem.sentence(),
        )
        await user.click(screen.getByRole('button'))
      })

      test('then it show name missing error', () => {
        const error = within(
          screen.getByLabelText(/name/i).parentElement!,
        ).queryByRole('alert')
        expect(error).toBeInTheDocument()
        expect(error).toHaveTextContent(/name must be at least/i)
      })

      test(`then it doesn't call onCreate`, () => {
        expect(onCreate).not.toHaveBeenCalled()
      })
    })
  })

  describe('errors', () => {
    describe('when there is a remote error', () => {
      let error: string
      beforeEach(() => {
        error = faker.lorem.sentence()

        render(
          <ChakraProvider value={system}>
            <ApiKeyForm onCreate={vi.fn()} error={error} />
          </ChakraProvider>,
        )
      })

      test('then it shows it', async () => {
        const text = await screen.findByRole('alert')
        expect(text).toBeInTheDocument()
        expect(text).toHaveTextContent(error)
      })
    })
  })
})
