import pinoHttp from "pino-http";
import logger from "../common/logger";

const pinoHttpMiddleware = pinoHttp({
  logger,
  redact: ["req.headers.authorization"],
});

export default pinoHttpMiddleware;
