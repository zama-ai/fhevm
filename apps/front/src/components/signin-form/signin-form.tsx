import { NavLink } from 'react-router'

import { Fieldset, Heading, HStack, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'
import { Field } from '@/components/ui/field'
import { Checkbox } from '@/components/ui/checkbox'
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
    <form onSubmit={formik.handleSubmit}>
      <Fieldset.Root>
        <Stack>
          <Fieldset.Legend>
            <Heading>Log in to your account</Heading>
          </Fieldset.Legend>
        </Stack>

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

        <HStack justifyContent="space-between">
          <Checkbox>Remember me</Checkbox>

          <NavLink to="/forgot-password">
            <Text textStyle="sm">Forgot password?</Text>
          </NavLink>
        </HStack>

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
      </Fieldset.Root>
    </form>
  )
}
