import Stripe from "stripe";

// Use Stripe SDK with your secret key
const stripe = new Stripe(process.env.STRIPE_API_KEY as string, {
  // NOTE: apiVersion 2023-10-16 refers to v14, while the last release is v18
  apiVersion: "2023-10-16",
});

/**
 * Only allow expected Stripe session IDs (e.g., cs_test_... or cs_live_...) to prevent URL manipulation.
 * Adjust this regex if Stripe changes their format.
 */
export function verifyStripeSession(checkout_session_id: string): Promise<any> {
  const validSessionId = /^cs_(test|live)_[a-zA-Z0-9]{20,}$/.test(
    checkout_session_id
  );
  if (!validSessionId) {
    return Promise.reject(
      new Error("Invalid Stripe checkout_session_id format")
    );
  }
  return fetch(
    `https://api.stripe.com/v1/checkout/sessions/${checkout_session_id}`,
    {
      headers: {
        Authorization: `bearer ${process.env.STRIPE_API_KEY}`,
      },
    }
  )
    .then(async (res) => {
      if (!res.ok) {
        console.warn(`Failed to verify stripe session: ${res.statusText}`);
        const message = await res.text();
        console.log(`Stripe error: ${message}`);
        throw new Error("Invalid session");
      }
      return res;
    })
    .then((res) => res.json());
}

export function getStripeCustomer(email: string): Promise<any> {
  return fetch(
    `https://api.stripe.com/v1/customers/search?query=email:"${encodeURIComponent(
      email
    )}"`,
    {
      headers: {
        Authorization: `bearer ${process.env.STRIPE_API_KEY}`,
      },
    }
  )
    .then(async (res) => {
      if (!res.ok) {
        console.warn(`Failed to retrieve stripe customer: ${res.statusText}`);
        const message = await res.text();
        console.log(`Stripe error: ${message}`);
        throw new Error("Invalid customer");
      }
      return res;
    })
    .then((res) => res.json());
}

// Developers: you might consider having something like redis
// make id/mapping look up easier and faster
const EMAIL_TO_STRIPE_CUSTOMER_CACHE: Record<string, string> = {};

export function getStripeCustomerIdFromCache(
  email: string
): string | undefined {
  return EMAIL_TO_STRIPE_CUSTOMER_CACHE[email];
}

export async function getStripeCustomerId(
  email: string
): Promise<string | undefined> {
  if (EMAIL_TO_STRIPE_CUSTOMER_CACHE[email]) {
    return EMAIL_TO_STRIPE_CUSTOMER_CACHE[email];
  }

  const stripeCustomer = await getStripeCustomer(email);
  const stripeCustomerId =
    stripeCustomer.data && stripeCustomer.data[0]
      ? stripeCustomer.data[0].id
      : undefined;

  if (stripeCustomerId) {
    EMAIL_TO_STRIPE_CUSTOMER_CACHE[email] = stripeCustomerId;
  }

  return stripeCustomerId;
}

export async function createStripeCheckoutSession(
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
