import { NavLink } from 'react-router'

import {
  Button,
  Fieldset,
  Heading,
  HStack,
  Input,
  Stack,
  Text,
} from '@chakra-ui/react'
import { useFormik } from 'formik'
import { Field } from '@/components/ui/field'
import { Checkbox } from '@/components/ui/checkbox'
import { PasswordInput } from '@/components/ui/password-input'

export function SigninForm() {
  const formik = useFormik({
    initialValues: {
      email: '',
      password: '',
    },
    onSubmit: values => {
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
              name="email"
              type="email"
              placeholder="bob@eth-app.net"
              onChange={formik.handleChange}
              value={formik.values.email}
            />
          </Field>
        </Fieldset.Content>

        <Fieldset.Content>
          <Field label="Password">
            <PasswordInput
              name="password"
              type="password"
              placeholder="****"
              onChange={formik.handleChange}
              value={formik.values.password}
            />
          </Field>
        </Fieldset.Content>

        <HStack justifyContent="space-between">
          <Checkbox>Remember me</Checkbox>

          <NavLink to="/forgot-password">
            <Text textStyle="sm">Forgot password?</Text>
          </NavLink>
        </HStack>

        <Button type="submit" alignSelf="flex-start">
          Login
        </Button>
      </Fieldset.Root>
    </form>
  )
}
