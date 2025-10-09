import "express";
import pino from "pino";
import { User } from "./user";

declare module "http" {
  interface IncomingMessage {
    log: pino.Logger;
    id: string;
  }
}

declare module "express" {
  interface Request {
    user?: User;
  }
}

declare module "express-serve-static-core" {
  interface Request {
    log: pino.Logger;
    id: string;
  }
}
