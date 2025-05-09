import { useEffect, useMemo, useState } from 'react'
import {
  Fieldset,
  Input,
  SelectItemText,
  Span,
  Stack,
  createListCollection,
} from '@chakra-ui/react'
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
import { useChains } from './use-chains.js'

type OwnProps = {
  onSubmit: (values: { name: string; address: string }) => void
  onValidateAddress: ({
    chainId,
    address,
  }: {
    chainId: number
    address: string
  }) => void
  onUpdateTitle: (title: string) => void
  addressLoading: boolean
  addressError: string
  loading: boolean
  errorMessage?: string
}

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

  const { chains } = useChains()
  const chainOptions = useMemo(
    () =>
      createListCollection({
        items: chains.map(c => ({
          label: c.name,
          value: c.id.toString(),
          description: c.description,
        })),
      }),
    [chains],
  )

  const formik = useFormik({
    initialValues: {
      name: '',
      chainId: '',
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
                collection={chainOptions}
                size="sm"
                width="320px"
                value={[formik.values.chainId]}
                onValueChange={e => formik.setFieldValue('chainId', e.value[0])}
              >
                <SelectLabel>
                  Select a chain on which your dApp is deployed
                </SelectLabel>
                <SelectTrigger>
                  <SelectValueText placeholder="Select a chain" />
                </SelectTrigger>
                <SelectContent>
                  {chainOptions.items.map(chain => (
                    <SelectItem item={chain} key={chain.value}>
                      <Stack gap="0">
                        <SelectItemText>{chain.label}</SelectItemText>
                        <Span color="fg.muted" textStyle="xs">
                          {chain.description}
                        </Span>
                      </Stack>
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
                  disabled={formik.values.chainId === '' || loading}
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
                        chainId: parseInt(formik.values.chainId, 10),
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
              !(formik.isValid && formik.dirty) ||
              !!addressError ||
              loading ||
              addressLoading
            }
          >
            Create
          </SpinnerButton>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
