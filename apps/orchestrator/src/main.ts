import { NestFactory } from '@nestjs/core'
import { AppModule } from './app.module.js'
import { Logger } from 'nestjs-pino'
import { ConfigService } from '@nestjs/config'

async function bootstrap() {
  const app = await NestFactory.create(AppModule)
  app.useLogger(app.get(Logger))
  const port = app.get(ConfigService).get<number>('common.port') ?? 3000
  await app.listen(port)
}
bootstrap()
