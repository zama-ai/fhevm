import { z } from 'zod'

type CommandTypes = 'discover-sc' | 'confirm-sc' | 'register-sc'

function genSchema<Key extends CommandTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `app-deployment.${key}` as `app-deployment.${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      applicationId: z.string(),
      deploymentId: z.string(),
      ...payload,
    }),
  })
}

const commandMap = {
  'discover-sc': genSchema('discover-sc', {
    address: z.string(),
    chainId: z.string(),
  }),
  'confirm-sc': genSchema('confirm-sc', {}),
  'register-sc': genSchema('register-sc', {}),
}
type CommandMap = typeof commandMap

const schema = z
  .discriminatedUnion('type', [
    commandMap['discover-sc'],
    commandMap['confirm-sc'],
    commandMap['register-sc'],
  ])
  .and(
    z.object({
      _tag: z.literal('Command'),
      $meta: z.record(z.string(), z.string()).optional(),
    }),
  )

export type AppDeploymentCommand = z.infer<typeof schema>

/**
 * Create a factory to generate a given command
 *
 * @param type The type of the Command to generate
 * @returns the factory function for the selected command
 */
function factory<K extends CommandTypes>(type: K) {
  return function (
    payload: z.infer<CommandMap[K]>['payload'],
    $meta?: Record<string, string>,
  ) {
    return {
      _tag: 'Command',
      type: `app-deployment.${type}`,
      payload,
      $meta,
    } as AppDeploymentCommand
  }
}

export const discoverSC = factory('discover-sc')
export const confirmSC = factory('confirm-sc')
export const registerSC = factory('register-sc')

export function isAppDeploymentCommand(
  data: unknown,
): data is AppDeploymentCommand {
  return schema.safeParse(data).success
}
