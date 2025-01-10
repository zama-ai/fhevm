import {
  isAppDeploymentCommand,
  type AppDeploymentCommand,
} from './app-deployment.commands.js'
import {
  isAppDeploymentEvent,
  type AppDeploymentEvent,
} from './app-deployment.events.js'

export type AppDeploymentMessage = AppDeploymentEvent | AppDeploymentCommand

export function isAppDeploymentMessage(
  data: unknown,
): data is AppDeploymentMessage {
  return isAppDeploymentEvent(data) || isAppDeploymentCommand(data)
}
