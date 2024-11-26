import { NavLink } from 'react-router'

import {
  Box,
  Button,
  Fieldset,
  Flex,
  Heading,
  HStack,
  Input,
  Stack,
  Text,
} from '@chakra-ui/react'
import { Field } from '@/components/ui/field'
import { Checkbox } from '@/components/ui/checkbox'
import { PasswordInput } from '@/components/ui/password-input'

export function SigninPage() {
  return (
    <Flex minHeight="100vh" width="100%">
      <Box flex="1" display="flex" alignItems="center" justifyContent="center">
        <Stack>
          <Heading>Welcome back!</Heading>
        </Stack>
      </Box>
      <Box
        flex="1"
        display="flex"
        alignItems="center"
        p={8}
        flexShrink={1}
        flexBasis={0}
        flexGrow={1}
      >
        <Fieldset.Root>
          <Stack>
            <Fieldset.Legend>
              <Heading>Log in to your account</Heading>
            </Fieldset.Legend>
            <Fieldset.HelperText>
              Don't have an account? <NavLink to="/register">Register</NavLink>
            </Fieldset.HelperText>
          </Stack>

          <Fieldset.Content>
            <Field label="Email">
              <Input name="email" type="email" placeholder="bob@eth-app.net" />
            </Field>
          </Fieldset.Content>

          <Fieldset.Content>
            <Field label="Password">
              <PasswordInput
                name="password"
                type="password"
                placeholder="****"
              />
            </Field>
          </Fieldset.Content>

          <HStack justifyContent="space-between">
            <Checkbox>Remember me</Checkbox>

            <NavLink to="/forgot-password">
              <Text textStyle="sm">Forgot password?</Text>
            </NavLink>
          </HStack>

          <Button asChild type="submit" alignSelf="flex-start">
            <NavLink to="/dashboard">Login</NavLink>
          </Button>
        </Fieldset.Root>
      </Box>
    </Flex>
  )
}
