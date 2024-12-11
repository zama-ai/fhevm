import { Box, Fieldset, Grid, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'

import { Field } from '@/components/ui/field'
import { SpinnerButton } from '@/components/ui/spinner-button'
import { toFormikValidate } from '@/lib/zod-schema-validator'

import { TutorialName } from './tutorial-name'
import { SolidityCodeTemplate } from './solidity-code-template'
import { CreatorNameFormSchema } from './validations'

type OwnProps = {
  onSubmit: (values: { name: string }) => void
  loading: boolean
  errorMessage?: string
}

export function CreatorName({ onSubmit, loading, errorMessage }: OwnProps) {
  const formik = useFormik({
    initialValues: {
      name: '',
    },
    onSubmit: values => {
      onSubmit(values)
    },
    validate: toFormikValidate(CreatorNameFormSchema),
  })
  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Content w={{ base: 'full', md: '1/2' }}>
            <Field label="dApp name">
              <Input
                disabled={loading}
                name="name"
                type="text"
                placeholder="My first dApp"
                onChange={formik.handleChange}
                value={formik.values.name}
              />
            </Field>
          </Fieldset.Content>
          {errorMessage && (
            <Text color="red.500" fontSize="sm">
              {errorMessage}
            </Text>
          )}
          <Text fontSize="sm" fontWeight="medium" mb={0} pb={0}>
            Solidity Code
          </Text>
          <Grid templateColumns="repeat(2, 1fr)" gap="6">
            <SolidityCodeTemplate />
            <Box>
              <TutorialName />
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
