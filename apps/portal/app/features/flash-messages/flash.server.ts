import { redirect } from "react-router";
import { getSession, sessionStorage } from "~/features/auth/session.server";
import type { Auth0User } from "~/types/auth";

const FLASH_SESSION_KEY = "__flash";

export type FlashMessageLevel = "info" | "success" | "warning" | "error";

export interface FlashMessage {
  type: FlashMessageLevel;
  message: string;
  title?: string;
}

export async function redirectWithFlash(
  request: Request,
  to: string,
  flash: FlashMessage,
  init?: ResponseInit
) {
  const session = await getSession(request);
  session.flash(FLASH_SESSION_KEY, flash);

  const headers = new Headers(init?.headers);
  headers.append("Set-Cookie", await sessionStorage.commitSession(session));

  return redirect(to, {
    ...init,
    headers,
  });
}

export async function getUserAndFlash(request: Request) {
  const session = await getSession(request);
  const user = session.get("user") as Auth0User | undefined;
  const flash = session.get(FLASH_SESSION_KEY) as FlashMessage | undefined;

  let headers: HeadersInit | undefined;
  if (flash) {
    headers = {
      "Set-Cookie": await sessionStorage.commitSession(session),
    };
  }

  return { user, flash, headers };
}
