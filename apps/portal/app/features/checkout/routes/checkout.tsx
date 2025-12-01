import { type LoaderFunctionArgs } from "react-router";
import { getUserFromSession } from "~/features/auth/session.server";
import { createStripeCheckoutSession } from "~/features/checkout/stripe.server";
import { loadStripe } from "@stripe/stripe-js";
import {
  EmbeddedCheckoutProvider,
  EmbeddedCheckout,
} from "@stripe/react-stripe-js";
import type { Route } from "./+types/checkout";
import { config } from "~/config";
import { redirectWithFlash } from "~/features/flash-messages/flash.server";

const stripePromise = loadStripe(config.stripe.publishableKey!);

export async function loader({ request }: LoaderFunctionArgs) {
  const user = await getUserFromSession(request);
  if (!user) {
    console.warn("Unauthorized access to /checkout, redirecting to /login");
    return redirectWithFlash(request, "/auth/login", {
      type: "warning",
      message: "Please sign in to continue.",
    });
  }

  const url = new URL(request.url);
  const price_id = url.searchParams.get("price_id");
  const quantity = url.searchParams.get("quantity");

  if (!price_id) {
    console.warn("Missing price_id in /checkout, redirecting to /pricing");
    return redirectWithFlash(request, "/pricing", {
      type: "warning",
      message: "Select a plan before checking out.",
    });
  }

  const session = await createStripeCheckoutSession({
    user,
    priceId: price_id,
    quantity: quantity ? parseInt(quantity, 10) : undefined,
  });
  return { session };
}

export default function Checkout({ loaderData }: Route.ComponentProps) {
  const { session } = loaderData;

  return (
    <div className="bg-gray-50 rounded-lg p-4">
      <h3 className="text-lg font-semibold mb-3">Subscribe</h3>
      <div id="checkout">
        <EmbeddedCheckoutProvider
          stripe={stripePromise}
          options={{
            clientSecret: session.client_secret,
          }}
        >
          <EmbeddedCheckout />
        </EmbeddedCheckoutProvider>
      </div>
    </div>
  );
}
