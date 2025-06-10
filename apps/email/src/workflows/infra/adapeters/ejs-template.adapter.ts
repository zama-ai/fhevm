import { Injectable, Logger } from '@nestjs/common'
import { compile, TemplateFunction } from 'ejs'
import { readFileSync } from 'fs'
import * as path from 'path'
import { AppError, Task, unknownError } from 'utils'
import { TemplateAdapter } from '#workflows/use-cases/adapters/template.adapter.js'
import { Email } from '#domain/email.js'

@Injectable()
export class EjsTemplateAdapter implements TemplateAdapter {
  private readonly logger = new Logger(EjsTemplateAdapter.name)

  private precompiledTemplates: Record<string, TemplateFunction> = {}

  public render(email: Email): Task<string, AppError> {
    const { context, template } = email.data
    const templateBaseDir = process.cwd() + '/templates'
    const templateExt = path.extname(template) || '.ejs'
    let templateName = path.basename(template, path.extname(template))
    const templateDir = path.isAbsolute(template)
      ? path.dirname(template)
      : path.join(templateBaseDir, path.dirname(template))
    const templatePath = path.join(templateDir, templateName + templateExt)
    templateName = path
      .relative(templateBaseDir, templatePath)
      .replace(templateExt, '')

    if (!this.precompiledTemplates[templatePath]) {
      try {
        const template = readFileSync(templatePath, 'utf-8')
        this.precompiledTemplates[templatePath] = compile(template, {
          filename: templatePath,
        })
      } catch (err) {
        this.logger.warn(`failed to read template: ${templatePath}: ${err}`)
        return Task.reject<string, AppError>(
          unknownError(`failed to read template: ${templatePath}: ${err}`),
        )
      }
    }

    try {
      this.logger.debug(
        `rendering template: ${templatePath} with context: ${JSON.stringify(
          context,
        )}`,
      )
      const rendered = this.precompiledTemplates[templatePath](context)
      return Task.of(rendered)
    } catch (err) {
      this.logger.warn(`failed to render template: ${templatePath}: ${err}`)
      return Task.reject<string, AppError>(
        unknownError(`failed to render template: ${templatePath}: ${err}`),
      )
    }
  }
}
