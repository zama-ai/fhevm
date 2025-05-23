import { Field } from '@/components/ui/field'
import { SpinnerButton } from '@/components/ui/spinner-button'
import { Fieldset, Heading, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'
import { z } from 'zod'
import { useRequestResetPassword } from './use-request-reset-password'
import { ErrorMessage } from '@/components/error-message/error-message'
import { toFormikValidate } from '@/lib/zod-schema-validator'
import { Link } from '@/components/ui/link'
import { ChevronLeft } from 'lucide-react'

export function RequestResetPassword() {
  const { requestResetPassword, loading, completed, error } =
    useRequestResetPassword()

  const formik = useFormik<RequestResetPasswordValues>({
    initialValues: {
      email: '',
    },
    validate: toFormikValidate(schema),
    onSubmit: requestResetPassword,
  })

  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Legend>
            <Heading>Forgot Password</Heading>
            <Text>
              Enter your email and we will send you a link to reset your
              password
            </Text>
          </Fieldset.Legend>
          <Fieldset.Content>
            <Field
              label="Email address"
              invalid={formik.errors.email !== undefined}
              errorText={formik.errors.email}
              disabled={completed}
            >
              <Input
                name="email"
                type="text"
                placeholder="bob@eth-app.net"
                onChange={formik.handleChange}
                value={formik.values.email}
                required
              />
            </Field>
          </Fieldset.Content>

          {error && <ErrorMessage>{error}</ErrorMessage>}

          <SpinnerButton
            loading={loading}
            loadingText="Requesting..."
            type="submit"
            alignSelf="flex-start"
            disabled={completed || loading}
          >
            Request
          </SpinnerButton>
          {completed && (
            <Text>
              You will receive an email with instructions about how to reset
              your password in a few minutes.
            </Text>
          )}
          <Text textStyle="sm">
            <Link to="/signin">
              <ChevronLeft />
              &nbsp;Back to login
            </Link>
          </Text>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}

const schema = z.object({
  email: z.string().email(),
})

type RequestResetPasswordValues = z.infer<typeof schema>
