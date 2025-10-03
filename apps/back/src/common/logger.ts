import pino from "pino";

const isProduction = process.env.NODE_ENV === "production";

const logger = pino({
  ...(isProduction
    ? {} // Default: JSON output for production (good for ELK ingestion)
    : {
        transport: {
          target: "pino-pretty",
          options: { colorize: true },
        },
      }),
  level: process.env.LOG_LEVEL || "info",
});

export default logger;
