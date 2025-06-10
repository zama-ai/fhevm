import { Email } from '#domain/email.js'
import { AppError, Task } from 'utils'

export const TEMPLATE_ADAPTER = 'TEMPLATE_ADAPTER'

export interface TemplateAdapter {
  render(email: Email): Task<string, AppError>
}
