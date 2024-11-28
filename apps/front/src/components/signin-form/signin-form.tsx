import { Fieldset, Heading, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'
import { Field } from '@/components/ui/field'
import { PasswordInput } from '@/components/ui/password-input'
import { SpinnerButton } from '@/components/ui/spinner-button'

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
      console.log(JSON.stringify(values, null, 2))
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

          {errorMessage && (
            <Text color="red.500" fontSize="sm">
              {errorMessage}
            </Text>
          )}

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
