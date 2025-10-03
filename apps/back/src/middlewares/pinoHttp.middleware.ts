import pinoHttp from "pino-http";
import logger from "../common/logger";

const pinoHttpMiddleware = pinoHttp({
  logger,
});

export default pinoHttpMiddleware;
