import { AsyncLocalStorage } from "async_hooks";
import pino from "pino";
import logger from "./logger";

type LoggerContext = {
  traceId: string;
  logger: pino.Logger;
};

export const loggerContext = new AsyncLocalStorage<LoggerContext>();

export function getLogger(): pino.Logger {
  const store = loggerContext.getStore();
  return store?.logger || logger;
}

export function getTraceId(): string | undefined {
  return loggerContext.getStore()?.traceId;
}
