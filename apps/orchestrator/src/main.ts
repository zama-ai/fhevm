import { NestFactory } from '@nestjs/core'
import { AppModule } from './app.module.js'
import { Logger } from 'nestjs-pino'

async function bootstrap() {
  const app = await NestFactory.create(AppModule)
  app.useLogger(app.get(Logger))
  await app.listen(process.env.PORT ?? 3000)
}
bootstrap()
