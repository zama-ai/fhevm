import { lazy } from 'react'
import { Box, Fieldset, Grid, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'

import { Field } from '@/components/ui/field.js'
import { SpinnerButton } from '@/components/ui/spinner-button.js'
import { ErrorMessage } from '@/components/error-message/error-message.js'
import { toFormikValidate } from '@/lib/zod-schema-validator.js'

import { TutorialName } from './tutorial-name.js'
import { CreatorNameFormSchema } from './validations.js'

const SolidityCodeTemplate = lazy(() =>
  import('./solidity-code-template.js').then(module => ({
    default: module.SolidityCodeTemplate,
  })),
)

type OwnProps = {
  onSubmit: (values: { name: string }) => void
  onUpdateTitle: (title: string) => void
  loading: boolean
  errorMessage?: string
}

export function CreatorName({
  onSubmit,
  onUpdateTitle,
  loading,
  errorMessage,
}: OwnProps) {
  const formik = useFormik({
    initialValues: {
      name: '',
    },
    onSubmit,
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
                placeholder="New dApp"
                onChange={ev => {
                  formik.handleChange(ev)
                  onUpdateTitle(ev.target.value)
                }}
                value={formik.values.name}
              />
            </Field>
          </Fieldset.Content>
          {errorMessage && <ErrorMessage>{errorMessage}</ErrorMessage>}
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
