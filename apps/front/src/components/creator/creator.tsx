import { useEffect, useState } from 'react'
import { Fieldset, Input, Stack, createListCollection } from '@chakra-ui/react'
import { useFormik } from 'formik'

import {
  SelectContent,
  SelectItem,
  SelectLabel,
  SelectRoot,
  SelectTrigger,
  SelectValueText,
} from '@/components/ui/select'
import { Field } from '@/components/ui/field.js'

import { SpinnerButton } from '@/components/ui/spinner-button.js'
import { ErrorMessage } from '@/components/error-message/error-message.js'
import { toFormikValidate } from '@/lib/zod-schema-validator.js'

import { CreatorFormSchema, AddressSchema } from './validations.js'
import { InputGroup } from '../ui/input-group.js'
import { InputGroupDot } from '../input-group-dot/input-group-dot.js'

type OwnProps = {
  onSubmit: (values: { name: string; address: string }) => void
  onValidateAddress: ({
    chainId,
    address,
  }: {
    chainId: string
    address: string
  }) => void
  onUpdateTitle: (title: string) => void
  addressLoading: boolean
  addressError: string
  loading: boolean
  errorMessage?: string
}

const chains = createListCollection({
  items: [
    { label: 'Sepolia', value: '11155111' },
    { label: 'Base', value: '8453' },
    { label: 'ETH Mainnet', value: '1' },
    { label: 'FileCoin', value: '314' },
  ],
})

export function CreatorForm({
  onSubmit,
  onUpdateTitle,
  loading,
  errorMessage,
  onValidateAddress,
  addressLoading,
  addressError,
}: OwnProps) {
  const [addressServerError, setAddressServerError] = useState<string | null>(
    null,
  )

  const formik = useFormik({
    initialValues: {
      name: '',
      address: '',
    },
    onSubmit,
    validate: toFormikValidate(CreatorFormSchema),
  })

  // Set the server error message
  useEffect(() => {
    if (addressError && !formik.errors.address)
      setAddressServerError(addressError)
  }, [addressError, formik])

  function getDot() {
    if (addressLoading && !formik.errors.address)
      return <InputGroupDot variant="animated" />
    if (!formik.errors.address && formik.touched.address && !addressError)
      return <InputGroupDot variant="green" />
    return null
  }

  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Content w={{ base: 'full', md: '1/2' }}>
            <Field
              label="dApp name"
              invalid={!!formik.errors.name && formik.touched.name}
              errorText={
                formik.errors.name && formik.touched.name
                  ? formik.errors.name
                  : undefined
              }
            >
              <Input
                disabled={loading}
                name="name"
                type="text"
                placeholder="New dApp"
                onBlur={formik.handleBlur}
                onChange={ev => {
                  formik.handleChange(ev)
                  onUpdateTitle(ev.target.value)
                }}
                value={formik.values.name}
              />
            </Field>
          </Fieldset.Content>
          <Fieldset.Content w={{ base: 'full', md: '1/2' }}>
            <Field label="Chain">
              <SelectRoot
                collection={chains}
                size="sm"
                width="320px"
                value={['11155111']}
                disabled
              >
                <SelectLabel>
                  Select a chain on which your dApp is deployed
                </SelectLabel>
                <SelectTrigger>
                  <SelectValueText placeholder="Select a chain" />
                </SelectTrigger>
                <SelectContent>
                  {chains.items.map(chain => (
                    <SelectItem item={chain} key={chain.value}>
                      {chain.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </SelectRoot>
            </Field>
          </Fieldset.Content>

          <Fieldset.Content w={{ base: 'full', md: '1/2' }}>
            <Field
              label="Smart contract address"
              invalid={
                (!!formik.errors.address && formik.touched.address) ||
                !!addressServerError
              }
              errorText={
                formik.errors.address && formik.touched.address
                  ? formik.errors.address
                  : addressServerError
                    ? addressServerError
                    : ''
              }
            >
              <InputGroup w="full" flex="1" endElement={getDot()}>
                <Input
                  disabled={loading}
                  name="address"
                  type="text"
                  placeholder="0x1234567890abcdef"
                  onBlur={formik.handleBlur}
                  onChange={ev => {
                    formik.handleChange(ev)
                    setAddressServerError(null)

                    const isValid = AddressSchema.safeParse(
                      ev.target.value,
                    ).success

                    if (isValid) {
                      onValidateAddress({
                        chainId: '11155111',
                        address: ev.target.value,
                      })
                    }
                  }}
                  value={formik.values.address}
                />
              </InputGroup>
            </Field>
          </Fieldset.Content>
          {errorMessage && <ErrorMessage>{errorMessage}</ErrorMessage>}

          <SpinnerButton
            loading={loading}
            loadingText="Saving..."
            type="submit"
            alignSelf="flex-start"
            disabled={
              !(formik.isValid && formik.dirty && !addressLoading) ||
              !!addressError ||
              loading
            }
          >
            Create
          </SpinnerButton>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
