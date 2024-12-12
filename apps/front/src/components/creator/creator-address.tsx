import { Box, Fieldset, Grid, Input, Stack } from '@chakra-ui/react'
import { useFormik } from 'formik'

import { toFormikValidate } from '@/lib/zod-schema-validator'
import { Field } from '@/components/ui/field'
import { SpinnerButton } from '@/components/ui/spinner-button'
import { ErrorMessage } from '@/components/error-message/error-message'
import { TutorialAddress } from './tutorial-address'
import { Alert } from '../ui/alert'
import { CreatorAddressFormSchema } from './validations'

type OwnProps = {
  onSubmit: (values: { address: string }) => void
  loading: boolean
  errorMessage?: string
}

export function CreatorAddress({ onSubmit, loading, errorMessage }: OwnProps) {
  const formik = useFormik({
    initialValues: {
      address: '',
    },
    onSubmit: values => {
      onSubmit(values)
    },
    validate: toFormikValidate(CreatorAddressFormSchema),
  })
  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Content w={{ base: 'full', md: '1/2' }}>
            <Field
              label="Smart contract address"
              invalid={!!formik.errors.address && formik.touched.address}
              errorText={
                formik.errors.address && formik.touched.address
                  ? formik.errors.address
                  : undefined
              }
            >
              <Input
                disabled={loading}
                name="address"
                type="text"
                placeholder="0x1234567890abcdef"
                onBlur={formik.handleBlur}
                onChange={formik.handleChange}
                value={formik.values.address}
              />
            </Field>
          </Fieldset.Content>
          {errorMessage && <ErrorMessage>{errorMessage}</ErrorMessage>}

          <Grid templateColumns="repeat(2, 1fr)" gap="6">
            <TutorialAddress />

            <Box>
              <Alert status="info" title="Reassuring message" my="5">
                We are verifying that your contract have been successfully
                deployed on the blockchain This process may take a few minutes.
              </Alert>
            </Box>
          </Grid>

          <Box display="flex" justifyContent="flex-end">
            <SpinnerButton
              loading={loading}
              loadingText="Saving..."
              type="submit"
              alignSelf="flex-start"
              disabled={!(formik.isValid && formik.dirty) || loading}
            >
              Next step
            </SpinnerButton>
          </Box>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
