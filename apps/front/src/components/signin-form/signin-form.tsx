import { Fieldset, Heading, Input, Stack } from '@chakra-ui/react'
import { useFormik } from 'formik'
import { Field } from '#components/ui/field.js'
import { PasswordInput } from '#components/ui/password-input.js'
import { SpinnerButton } from '#components/ui/spinner-button.js'
import { ErrorMessage } from '../error-message/error-message.js'

type OwnProps = {
  onSubmit: (values: { email: string; password: string }) => void
  loading: boolean
  errorMessage?: string
}

export function SigninForm({ onSubmit, loading, errorMessage }: OwnProps) {
  const formik = useFormik({
    initialValues: {
      email: '',
      password: '',
    },
    onSubmit: values => {
      onSubmit(values)
    },
  })
  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Legend>
            <Heading>Log in to your account</Heading>
          </Fieldset.Legend>

          <Fieldset.Content>
            <Field label="Email address">
              <Input
                disabled={loading}
                name="email"
                type="email"
                placeholder="bob@eth-app.net"
                onChange={formik.handleChange}
                value={formik.values.email}
                autoComplete="username"
                required
              />
            </Field>
          </Fieldset.Content>

          <Fieldset.Content>
            <Field
              label="Password"
              errorText={
                formik.errors.password && formik.touched.password
                  ? formik.errors.password
                  : undefined
              }
            >
              <PasswordInput
                disabled={loading}
                name="password"
                type="password"
                placeholder="****"
                onChange={formik.handleChange}
                value={formik.values.password}
                autoComplete="current-password"
                required
              />
            </Field>
          </Fieldset.Content>

          {errorMessage && <ErrorMessage>{errorMessage}</ErrorMessage>}

          <SpinnerButton
            loading={loading}
            loadingText="Saving..."
            type="submit"
            alignSelf="flex-start"
            disabled={loading}
          >
            Login
          </SpinnerButton>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
