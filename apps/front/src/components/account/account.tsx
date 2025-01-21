import { Fieldset, Heading, Input, Stack } from '@chakra-ui/react'
import { useFormik } from 'formik'

import { Field } from '@/components/ui/field.js'
import { SpinnerButton } from '@/components/ui/spinner-button.js'
import { ErrorMessage } from '@/components/error-message/error-message.js'
import { toFormikValidate } from '@/lib/zod-schema-validator.js'

import { AccountFormSchema } from './validations.js'

type OwnProps = {
  onSubmit: (values: { name: string }) => void
  name: string
  email: string
  loading: boolean
  errorMessage?: string
}

export function Account({
  name,
  email,
  onSubmit,
  loading,
  errorMessage,
}: OwnProps) {
  const formik = useFormik({
    initialValues: {
      name,
    },
    onSubmit,
    validate: toFormikValidate(AccountFormSchema),
  })

  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Legend>
            <Heading size="md">Account Preferences</Heading>
          </Fieldset.Legend>

          <Fieldset.Content>
            <Field
              label="Email address"
              helperText="(cannot be modified for now. We need to implement the email verification mechanism first)"
              disabled
            >
              <Input disabled name="email" type="email" value={email} />
            </Field>
          </Fieldset.Content>

          <Fieldset.Content>
            <Field label="Name">
              <Input
                name="name"
                type="text"
                value={formik.values.name}
                onChange={formik.handleChange}
              />
            </Field>
          </Fieldset.Content>

          {errorMessage && <ErrorMessage>{errorMessage}</ErrorMessage>}

          <SpinnerButton
            loading={loading}
            loadingText="Saving..."
            type="submit"
            alignSelf="flex-start"
            disabled={!formik.dirty || loading}
          >
            Update
          </SpinnerButton>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
