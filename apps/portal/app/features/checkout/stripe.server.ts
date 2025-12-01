import { Stripe } from "stripe";
import type { Auth0User } from "~/types/auth";

const stripeSecretKey = process.env.STRIPE_SECRET_KEY;

if (!stripeSecretKey) {
  throw new Error("Missing STRIPE_SECRET_KEY in environment variables");
}

const stripe = new Stripe(stripeSecretKey, { apiVersion: "2025-09-30.clover" });

export function verifyStripeSession(
  checkout_session_id: string
): Promise<Stripe.Checkout.Session> {
  const validSessionId = /^cs_(test|live)_[a-zA-Z0-9]{20,}$/.test(
    checkout_session_id
  );
  if (!validSessionId) {
    throw new Error("Invalid Stripe checkout_session_id format");
  }
  return stripe.checkout.sessions.retrieve(checkout_session_id);
}

export async function getStripeCustomer(
  email: string
): Promise<Stripe.Customer | null> {
  const response = await stripe.customers.search({
    query: `email:"${email.replace(/\\/g, "\\\\").replace(/\"/g, '\\"')}"`,
  });
  if (!response.data.length) {
    return null;
  } else if (response.data.length > 1) {
    console.warn(
      `found ${
        response.data.length
      } customers, returning the first one (${response.data
        .map((c) => c.id)
        .join(", ")})`
    );
  }
  return response.data[0];
}

export async function getCustomerSubscriptions(
  customerId: string,
  canceled = false
): Promise<Stripe.Subscription[]> {
  try {
    const subscriptions = await stripe.subscriptions.list({
      customer: customerId,
      status: canceled ? "canceled" : "active",
    });
    return subscriptions.data;
  } catch (error) {
    console.error(
      `Error fetching subscriptions for customer ${customerId}:`,
      error
    );
    return [];
  }
}

export async function createStripeCheckoutSession({
  user,
  priceId,
  quantity,
}: {
  user: Auth0User;
  priceId: string;
  quantity?: number;
}): Promise<Stripe.Checkout.Session> {
  let customer = await getStripeCustomer(user.email);
  if (!customer) {
    customer = await stripe.customers.create({
      email: user.email,
      metadata: {
        // add the user id from identity provider to
        // stripe metadata for customer.
        // Because, an alternative approach is to tie
        // the identity provider's user id to stripe customer
        // and look up customer object using user_id instead of
        // email.
        ...(user.sub ? { authUserId: user.sub } : {}),
      },
    });
  }

  const session = await stripe.checkout.sessions.create({
    ui_mode: "embedded",
    line_items: [
      {
        price: priceId,
        quantity: quantity,
      },
    ],
    customer: customer.id,
    mode: "subscription",
    return_url: `${process.env.PORTAL_APP_URL}/return?session_id={CHECKOUT_SESSION_ID}&price_id=${priceId}`,
  });

  return session;
}
