import { Fieldset, Heading, HStack, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'

import { toFormikValidate } from '@/lib/zod-schema-validator'
import { RegisterFormSchema, getPasswordStrengthScore } from './validations'

import { Checkbox } from '@/components/ui/checkbox'
import { Field } from '@/components/ui/field'
import { Link } from '@/components/ui/link'

import {
  PasswordInput,
  PasswordStrengthMeter,
} from '@/components/ui/password-input'
import { SpinnerButton } from '@/components/ui/spinner-button'

type OwnProps = {
  onSubmit: (values: { name: string; password: string }) => void
  email: string
  errorMessage?: string
  invitationKey: string
  loading: boolean
}

export function SignupForm({
  email,
  errorMessage,
  invitationKey,
  loading,
  onSubmit,
}: OwnProps) {
  const formik = useFormik({
    initialValues: {
      name: '',
      password: '',
      repeatPassword: '',
      invitationKey,
      agree: false,
    },
    onSubmit: values => {
      onSubmit(values)
    },
    validate: toFormikValidate(RegisterFormSchema),
  })
  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Legend>
            <Heading>Create a new account</Heading>
          </Fieldset.Legend>

          <Fieldset.Content>
            <Field
              label="Invited Email address"
              helperText="(cannot be modified)"
            >
              <Input
                disabled
                name="email"
                type="email"
                value={email}
                autoComplete="username"
              />
            </Field>
          </Fieldset.Content>

          <Fieldset.Content>
            <Field
              label="Name"
              invalid={!!formik.errors.name && formik.touched.name}
              errorText={
                formik.errors.name && formik.touched.name
                  ? formik.errors.name
                  : undefined
              }
            >
              <Input
                name="name"
                type="text"
                placeholder="Jane Doe"
                onBlur={formik.handleBlur}
                onChange={formik.handleChange}
                value={formik.values.name}
              />
            </Field>
          </Fieldset.Content>

          <Fieldset.Content>
            <Field
              label="New Password"
              invalid={!!formik.errors.password && formik.touched.password}
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
                onBlur={formik.handleBlur}
                onChange={formik.handleChange}
                value={formik.values.password}
                autoComplete="new-password"
              />
            </Field>
            <PasswordStrengthMeter
              max={4}
              value={getPasswordStrengthScore(formik.values.password)}
            />
          </Fieldset.Content>

          <Fieldset.Content>
            <Field
              label="Retype password"
              invalid={
                !!formik.errors.repeatPassword && formik.touched.repeatPassword
              }
              errorText={
                formik.errors.repeatPassword && formik.touched.repeatPassword
                  ? formik.errors.repeatPassword
                  : undefined
              }
            >
              <PasswordInput
                disabled={loading}
                name="repeatPassword"
                type="password"
                placeholder="****"
                onBlur={formik.handleBlur}
                onChange={formik.handleChange}
                value={formik.values.repeatPassword}
                autoComplete="new-password"
              />
            </Field>
          </Fieldset.Content>

          <HStack justifyContent="space-between">
            <Field
              flexBasis="fit-content"
              invalid={!!formik.errors.agree && formik.touched.agree}
              errorText={
                formik.errors.agree && formik.touched.agree
                  ? formik.errors.agree
                  : undefined
              }
            >
              <Checkbox
                name="agree"
                value={String(formik.values.agree)}
                onChange={formik.handleChange}
              >
                I agree the{' '}
                <Link to="https://www.zama.ai/legal-notice" variant="underline">
                  terms and conditions
                </Link>
                .
              </Checkbox>
            </Field>
            <Link to="/signin">
              <Text textStyle="sm">I already have an account</Text>
            </Link>
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
            Signup
          </SpinnerButton>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
