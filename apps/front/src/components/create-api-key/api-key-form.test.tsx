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
      expect(screen.getByTestId('api-key-name')).toBeInTheDocument()
    })

    test('should have a button', () => {
      expect(screen.getByRole('button')).toBeInTheDocument()
      expect(screen.getByRole('button')).toBeDisabled()
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

      beforeEach(async () => {
        name = faker.string.alpha({ length: { min: 5, max: 10 } })

        const user = userEvent.setup()
        await user.type(screen.getByTestId('api-key-name'), name)
        await user.click(screen.getByRole('button'))
      })

      test('then it call onCreate', () => {
        expect(onCreate).toHaveBeenCalledExactlyOnceWith({ name })
      })

      test('then there is no error', async () => {
        expect(
          within(screen.getByRole('form')).queryByRole('alert'),
        ).not.toBeInTheDocument()
      })
    })
  })

  describe('errors', () => {
    describe('when there is a remote error', () => {
      let errorText: string
      beforeEach(() => {
        errorText = faker.lorem.sentence()

        render(
          <ChakraProvider value={system}>
            <ApiKeyForm onCreate={vi.fn()} error={errorText} />
          </ChakraProvider>,
        )
      })

      test('then it shows it', async () => {
        expect(within(screen.getByRole('form')).getByText(errorText))
      })
    })
  })
})
