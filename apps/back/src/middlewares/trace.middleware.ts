import type { Request, Response, NextFunction } from "express";
import { v4 as uuidv4 } from "uuid";

import logger from "../common/logger";
import { loggerContext } from "../common/logger.context";

/**
 * Middleware to create a per-request logger context with traceId.
 */
export function traceMiddleware(
  req: Request,
  res: Response,
  next: NextFunction
) {
  const traceId = req.headers["x-trace-id"]?.toString() || uuidv4();
  const childLogger = logger.child({ traceId });

  loggerContext.run({ traceId, logger: childLogger }, () => {
    res.setHeader("X-Trace-Id", traceId);
    next();
  });
}
