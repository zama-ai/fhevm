const StripeSDK = require("stripe");
const stripe = StripeSDK(process.env.STRIPE_API_KEY);

function verifyStripeSession(checkout_session_id) {
  // Only allow expected Stripe session IDs (e.g., cs_test_... or cs_live_...) to prevent URL manipulation.
  // Adjust this regex if Stripe changes their format.
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
  ).then((res) => res.json());
}

function getStripeCustomer(email) {
  return fetch(
    `https://api.stripe.com/v1/customers/search?query=email:"${encodeURIComponent(
      email
    )}"`,
    {
      headers: {
        Authorization: `bearer ${process.env.STRIPE_API_KEY}`,
      },
    }
  ).then((res) => res.json());
}

// Developers: you might consider have something like reddis
// make id/mapping look up easier and faster
const EMAIL_TO_STRIPE_CUSTOMER_CACHE = {};

function getStripeCustomerIdFromCache(email) {
  return EMAIL_TO_STRIPE_CUSTOMER_CACHE[email];
}

async function getStripeCustomerId(email) {
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

async function createStripeCheckoutSession(email, priceId, quantity, authUser) {
  // make sure only one stripe customer per email
  let customerId = await getStripeCustomerId(email);

  if (!customerId) {
    // If no customerId exists, create a new one
    const customer = await stripe.customers.create({
      email: email,
      metadata: {
        // add the user id from identify provider to
        // stripe metadata for customer.
        // Because, an alternative approach is to tie
        // the identity provider's user id to stripe customer
        // and look up customer object using user_id instead of
        // email.
        authUserId: authUser?.sub,
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

module.exports = {
  verifyStripeSession,
  createStripeCheckoutSession,
  getStripeCustomer,
  getStripeCustomerId,
  getStripeCustomerIdFromCache,
};
