import { useFormik } from 'formik'
import { Fieldset, Heading, Input, HStack, Button } from '@chakra-ui/react'
import { Field } from '@/components/ui/field.js'
import { toFormikValidate } from '@/lib/zod-schema-validator'
import { CreateApiKeySchema } from './create-api-key.schema'

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

  return (
    <Fieldset.Root size="lg" maxW="md" py="4">
      <form onSubmit={formik.handleSubmit} role="form">
        <Fieldset.Legend>
          <Heading>Create a new API KEY</Heading>
        </Fieldset.Legend>

        <Fieldset.Content>
          <HStack>
            <Field
              label="Name"
              invalid={Boolean(formik.errors.name)}
              errorText={formik.errors.name}
            >
              <Input
                name="name"
                type="text"
                value={formik.values.name}
                onChange={formik.handleChange}
              />
            </Field>
            <Field
              label="Description"
              invalid={Boolean(formik.errors.description)}
              errorText={formik.errors.description}
            >
              <Input
                name="description"
                type="text"
                value={formik.values.description}
                onChange={formik.handleChange}
              />
            </Field>

            <Button type="submit" alignSelf="end">
              Create
            </Button>
          </HStack>
          {error && (
            <span key="remote-error" role="alert">
              {error}
            </span>
          )}
        </Fieldset.Content>
      </form>
    </Fieldset.Root>
  )
}
