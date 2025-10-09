import { JwtPayload } from "jsonwebtoken";

export interface User extends JwtPayload {
  nickname?: string;
  name?: string;
  picture?: string;
  updated_at?: string;
  // Note: The AuthPlugin rejects a JWT Payload without email
  email: string;
  email_verified?: boolean;
}
