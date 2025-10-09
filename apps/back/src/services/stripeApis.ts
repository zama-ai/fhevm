import Stripe from "stripe";
import { getLogger } from "../common/logger.context";
import { withSpan } from "../decorators/span";

// Use Stripe SDK with your secret key
const stripe = new Stripe(process.env.STRIPE_API_KEY as string, {
  // NOTE: apiVersion 2023-10-16 refers to v14, while the last release is v18
  apiVersion: "2025-09-30.clover",
});

/**
 * Only allow expected Stripe session IDs (e.g., cs_test_... or cs_live_...) to prevent URL manipulation.
 * Adjust this regex if Stripe changes their format.
 */
export const verifyStripeSession = withSpan(
  {
    name: "verifyStripeSession",
  },
  async function (
    checkout_session_id: string
  ): Promise<Stripe.Checkout.Session> {
    const logger = getLogger().child({
      method: "verifyStripeSession",
      checkout_session_id,
    });
    const validSessionId = /^cs_(test|live)_[a-zA-Z0-9]{20,}$/.test(
      checkout_session_id
    );
    if (!validSessionId) {
      logger.warn("Invalid Stripe checkout_session_id format");
      throw new Error("Invalid Stripe checkout_session_id format");
    }
    logger.debug(`fetching stripe session`);
    const session = await stripe.checkout.sessions.retrieve(
      checkout_session_id
    );
    logger.trace(session);
    return session;
  }
);

export const getStripeCustomer = withSpan(
  {
    name: "getStripeCustomer",
    logArgs: true,
  },
  async function (email: string): Promise<Stripe.Customer | null> {
    const logger = getLogger().child({ method: "getStripeCustomer", email });
    logger.debug(`fetching customer by email`);
    const response = await stripe.customers.search({
      // Note: latest version of Stripe requires not UriEncoded email
      query: `email:"${email.replace(/\\/g, '\\\\').replace(/\"/g, '\\"')}"`,
    });
    if (!response.data.length) {
      logger.warn(`no customer found with email=${email}`);
      return null;
    } else if (response.data.length > 1) {
      logger.warn(
        `found ${response.data.length} customers, returning the first one`
      );
    }
    return response.data[0];
  }
);

// Developers: you might consider having something like redis
// make id/mapping look up easier and faster
const EMAIL_TO_STRIPE_CUSTOMER_CACHE: Record<string, string> = {};

export function getStripeCustomerIdFromCache(
  email: string
): string | undefined {
  return EMAIL_TO_STRIPE_CUSTOMER_CACHE[email];
}

export const getStripeCustomerId = withSpan(
  {
    name: "getStripeCustomerId",
    logArgs: true,
  },
  async function (email: string): Promise<string | undefined> {
    if (EMAIL_TO_STRIPE_CUSTOMER_CACHE[email]) {
      return EMAIL_TO_STRIPE_CUSTOMER_CACHE[email];
    }

    const stripeCustomer = await getStripeCustomer(email);
    const stripeCustomerId = stripeCustomer?.id;

    if (stripeCustomerId) {
      EMAIL_TO_STRIPE_CUSTOMER_CACHE[email] = stripeCustomerId;
    }

    return stripeCustomerId;
  }
);

export const createStripeCheckoutSession = withSpan(
  {
    name: "createStripeCheckoutSession",
    logArgs: true,
  },
  async function (
    email: string,
    priceId: string,
    quantity?: string,
    authUser?: { sub?: string }
  ): Promise<Stripe.Response<Stripe.Checkout.Session>> {
    // make sure only one stripe customer per email
    let customerId = await getStripeCustomerId(email);

    if (!customerId) {
      // If no customerId exists, create a new one
      const customer = await stripe.customers.create({
        email: email,
        metadata: {
          // add the user id from identity provider to
          // stripe metadata for customer.
          // Because, an alternative approach is to tie
          // the identity provider's user id to stripe customer
          // and look up customer object using user_id instead of
          // email.
          ...(authUser?.sub ? { authUserId: authUser.sub } : {}),
        },
      });

      customerId = customer.id;
    }

    const session = await stripe.checkout.sessions.create({
      ui_mode: "embedded",
      line_items: [
        {
          // Provide the exact Price ID (for example, pr_1234) of the product you want to sell
          price: priceId,
          // for metered billing, do NOT include quantity
          quantity: quantity ? parseInt(quantity) || 1 : undefined,
        },
      ],
      customer: customerId,
      mode: "subscription",
      return_url: `${process.env.FRONT_END_URL}/return?session_id={CHECKOUT_SESSION_ID}&price_id=${priceId}`,
    });

    return session;
  }
);
