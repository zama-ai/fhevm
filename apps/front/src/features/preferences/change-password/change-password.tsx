import { Field } from '@/components/ui/field'
import { PasswordInput } from '@/components/ui/password-input'
import { SpinnerButton } from '@/components/ui/spinner-button'
import { toFormikValidate } from '@/lib/zod-schema-validator'
import { Button, Fieldset, Heading, Stack } from '@chakra-ui/react'
import { useFormik } from 'formik'
import { useState } from 'react'
import { ChangePasswordSchema, ChangePasswordValues } from './schema'
import { useChangePassword } from './use-change-password'
import { Alert } from '@/components/ui/alert'
import { useCallback } from 'react'
import { useEffect } from 'react'
import { ErrorMessage } from '@/components/error-message/error-message'

export function ChangePassword() {
  const { changePassword, updated, error, loading } = useChangePassword()
  const [edit, setEdit] = useState(false)
  const [showAlert, setShowAlert] = useState(false)
  const formik = useFormik<ChangePasswordValues>({
    initialValues: {
      oldPassword: '',
      newPassword: '',
      repeatPassword: '',
    },
    onSubmit: values => {
      changePassword({
        oldPassword: values.oldPassword!,
        newPassword: values.newPassword!,
      })
    },
    validate: toFormikValidate(ChangePasswordSchema),
  })
  const toggleEdit = useCallback(() => {
    setEdit(edit => !edit)
  }, [setEdit])

  useEffect(() => {
    let stale = false
    if (updated) {
      setEdit(false)
      setShowAlert(true)
      setTimeout(() => {
        if (!stale) {
          setShowAlert(false)
        }
      }, 10_000)
    }
    return () => {
      stale = true
    }
  }, [updated, setEdit, setShowAlert])

  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit} role="form" noValidate>
        <Stack gap="5">
          <Fieldset.Legend position="relative">
            <Heading size="md">Change your password</Heading>
            <Button position="absolute" right="0" top="0" onClick={toggleEdit}>
              {edit ? 'Cancel' : 'Edit'}
            </Button>
          </Fieldset.Legend>
          {edit && (
            <>
              <Fieldset.Content>
                <Field
                  label="Old Password"
                  errorText={formik.errors.oldPassword}
                  invalid={!!formik.errors.oldPassword}
                >
                  <PasswordInput
                    name="oldPassword"
                    type="password"
                    placeholder="****"
                    value={formik.values.oldPassword}
                    onChange={formik.handleChange}
                    required
                  />
                </Field>
              </Fieldset.Content>
              <Fieldset.Content>
                <Field
                  label="New Password"
                  errorText={formik.errors.newPassword}
                  invalid={!!formik.errors.newPassword}
                >
                  <PasswordInput
                    name="newPassword"
                    type="password"
                    placeholder="****"
                    value={formik.values.newPassword}
                    onChange={formik.handleChange}
                    required
                  />
                </Field>
              </Fieldset.Content>
              <Fieldset.Content>
                <Field
                  label="Repeat Password"
                  errorText={formik.errors.repeatPassword}
                  invalid={!!formik.errors.repeatPassword}
                >
                  <PasswordInput
                    name="repeatPassword"
                    type="password"
                    placeholder="****"
                    value={formik.values.repeatPassword}
                    onChange={formik.handleChange}
                    required
                  />
                </Field>
              </Fieldset.Content>
              {error && <ErrorMessage>{error}</ErrorMessage>}
              <SpinnerButton
                type="submit"
                alignSelf="flex-start"
                loading={loading}
              >
                Save
              </SpinnerButton>
            </>
          )}
        </Stack>
      </form>
      {showAlert && (
        <Alert status="success" role="alert">
          Your password has been updated
        </Alert>
      )}
    </Fieldset.Root>
  )
}
