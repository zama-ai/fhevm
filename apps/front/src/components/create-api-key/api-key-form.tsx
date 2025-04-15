import { useFormik } from 'formik'
import { Fieldset, Input, HStack, Button } from '@chakra-ui/react'
import { Field } from '@/components/ui/field.js'
import { toFormikValidate } from '@/lib/zod-schema-validator'
import { CreateApiKeySchema } from './create-api-key.schema'
import { LinkDoc } from '../ui/link'

export type ApiKeyFormProps = {
  error?: string
  onCreate: (values: { name: string; description?: string }) => void
}
export function ApiKeyForm({ error, onCreate }: ApiKeyFormProps) {
  const formik = useFormik({
    initialValues: {
      name: '',
      description: '',
    },
    validate: toFormikValidate(CreateApiKeySchema),
    onSubmit: values => {
      const name = values.name.trim()
      const description = values.description?.trim()
      onCreate({ name, description: description ? description : undefined })
    },
  })

  const canAdd = formik.values.name.trim() !== '' && !formik.errors.name
  const invalid = Boolean(error) || Boolean(formik.errors.name)

  return (
    <Fieldset.Root size="lg" maxW="md" pt="8">
      <form onSubmit={formik.handleSubmit} role="form">
        <Fieldset.Legend>API Keys</Fieldset.Legend>
        <Fieldset.HelperText>
          Enter a unique name for your token to differentiate it from other
          tokens.
        </Fieldset.HelperText>

        <Fieldset.Content my="4">
          <HStack>
            <Field
              label="Create a new API Key"
              invalid={invalid}
              errorText={error || formik.errors.name}
              helperText={
                <span>
                  {' '}
                  Learn more about{' '}
                  <LinkDoc href="https://zama.ai" target="_blank">
                    API Keys
                  </LinkDoc>
                </span>
              }
            >
              <Input
                data-testid="api-key-name"
                name="name"
                type="text"
                value={formik.values.name}
                onChange={formik.handleChange}
              />
            </Field>

            <Button
              type="submit"
              mt="26px"
              alignSelf="start"
              disabled={!canAdd}
            >
              Create
            </Button>
          </HStack>
        </Fieldset.Content>
      </form>
    </Fieldset.Root>
  )
}
