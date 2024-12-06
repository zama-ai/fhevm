import {
  isAppDeploymentCommand,
  type AppDeploymentCommand,
} from './app-deployment.commands'
import {
  isAppDeploymentEvent,
  type AppDeploymentEvent,
} from './app-deployment.events'

export type AppDeploymentMessage = AppDeploymentEvent | AppDeploymentCommand

export function isAppDeploymentMessage(
  data: unknown,
): data is AppDeploymentMessage {
  return isAppDeploymentEvent(data) || isAppDeploymentCommand(data)
}
