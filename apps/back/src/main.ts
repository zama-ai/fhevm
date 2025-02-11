import { NestFactory } from '@nestjs/core'
import { Logger } from 'nestjs-pino'
import { AppModule } from './app.module.js'

const PORT = process.env.PORT ?? 3000

async function bootstrap() {
  const app = await NestFactory.create(AppModule, { cors: true })
  app.useLogger(app.get(Logger))
  await app.listen(PORT)
}
bootstrap()
