import { getUserFromSession } from "~/features/auth/session.server";
import {
  getCustomerSubscriptions,
  getStripeCustomer,
} from "~/features/checkout/stripe.server";
import type Stripe from "stripe";
import { redirectWithFlash } from "~/features/flash-messages/flash.server";

export async function requireSubscription(
  request: Request
): Promise<{ customer: Stripe.Customer }> {
  const user = await getUserFromSession(request);

  if (!user) {
    throw await redirectWithFlash(request, "/auth/login", {
      type: "warning",
      message: "Please sign in to continue.",
    });
  }

  const customer = await getStripeCustomer(user.email);
  if (!customer) {
    throw await redirectWithFlash(request, "/pricing", {
      type: "warning",
      message:
        "We couldn't find a subscription for your account. Choose a plan to continue.",
    });
  }

  const subscriptions = await getCustomerSubscriptions(customer.id);
  if (subscriptions.length === 0) {
    throw await redirectWithFlash(request, "/pricing", {
      type: "warning",
      message: "You need an active subscription to access this page.",
    });
  }

  return { customer };
}
