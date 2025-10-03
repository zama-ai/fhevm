import "express";
import pino from "pino";

declare module "http" {
  interface IncomingMessage {
    log: pino.Logger;
    id: string;
  }
}

declare module "express-serve-static-core" {
  interface Request {
    log: pino.Logger;
    id: string;
  }
}
