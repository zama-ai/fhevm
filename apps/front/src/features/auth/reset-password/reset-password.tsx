import { useFormik } from 'formik'
import { useResetPassword } from './use-reset-password'
import { z } from 'zod'
import { toFormikValidate } from '@/lib/zod-schema-validator'
import { Fieldset, Heading, Stack } from '@chakra-ui/react'
import { PasswordInput } from '@/components/ui/password-input'
import { ErrorMessage } from '@/components/error-message/error-message'
import { SpinnerButton } from '@/components/ui/spinner-button'
import { Field } from '@/components/ui/field'

type ResetPasswordPros = {
  token: string
}

export function ResetPassword({ token }: ResetPasswordPros) {
  const { resetPassword, loading, error } = useResetPassword(token)

  const formik = useFormik<ResetPasswordValues>({
    initialValues: {
      password: '',
    },
    validate: toFormikValidate(schema),
    onSubmit: resetPassword,
  })

  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Legend>
            <Heading>Reset Your Password</Heading>
          </Fieldset.Legend>
          <Fieldset.Content>
            <Field
              label="Password"
              invalid={formik.errors.password !== undefined}
              errorText={formik.errors.password}
            >
              <PasswordInput
                disabled={loading}
                name="password"
                type="password"
                placeholder="****"
                onBlur={formik.handleBlur}
                onChange={formik.handleChange}
                value={formik.values.password}
                autoComplete="new-password"
              />
            </Field>
          </Fieldset.Content>

          {error && <ErrorMessage>{error}</ErrorMessage>}

          <SpinnerButton
            loading={loading}
            loadingText="Saving..."
            type="submit"
            alignSelf="flex-start"
            disabled={loading}
          >
            Save
          </SpinnerButton>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}

const schema = z.object({
  // TODO: we should enforce password rules
  password: z.string(),
})

type ResetPasswordValues = z.infer<typeof schema>
