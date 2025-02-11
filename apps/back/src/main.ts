import { NestFactory } from '@nestjs/core'
import { Logger } from 'nestjs-pino'
import { AppModule } from './app.module.js'
import { ConfigService } from '@nestjs/config'

async function bootstrap() {
  const app = await NestFactory.create(AppModule, { cors: true })
  app.useLogger(app.get(Logger))
  const port = app.get(ConfigService).get<number>('common.port') ?? 3000
  await app.listen(port)
}
bootstrap()
