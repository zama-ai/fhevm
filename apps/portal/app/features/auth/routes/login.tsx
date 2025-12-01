import { redirect, type LoaderFunctionArgs } from "react-router";
import { redirectToAuth0 } from "~/features/auth/auth0.server";

export async function loader({ request }: LoaderFunctionArgs) {
  const authUrl = redirectToAuth0(request);
  return redirect(authUrl);
}

export default function Login() {
  return null;
}
