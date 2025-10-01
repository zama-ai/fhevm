import { useState, useEffect } from "react";
import { loadStripe } from "@stripe/stripe-js";

import {
  EmbeddedCheckoutProvider,
  EmbeddedCheckout,
} from "@stripe/react-stripe-js";

import config from "../../../config";

const stripePromise = loadStripe(config.stripe.publishableKey);

type Props = {
  priceId: string;
  user: {
    email: string;
    [key: string]: any;
  };
  idToken: string;
  quantity: string;
};

// used on embedded checkout example code:
// https://docs.stripe.com/checkout/embedded/quickstart

function StripeCheckoutForm({ priceId, user, idToken, quantity }: Props) {
  const [clientSecret, setClientSecret] = useState("");

  useEffect(() => {
    // Create a Checkout Session as soon as the page loads
    if (!idToken) {
      return;
    }
    fetch(
      `${
        config.devPortalApiServer
      }/create-stripe-checkout-session?price_id=${priceId}&email=${encodeURIComponent(
        user?.email
      )}${quantity ? `&quantity=${quantity}` : ""}`,
      {
        method: "POST",
        headers: {
          Authorization: `Bearer ${idToken}`,
        },
      }
    )
      .then((res) => res.json())
      .then((data) => {
        console.log(JSON.stringify(data));
        setClientSecret(data.clientSecret);
      });
  }, [priceId, user, idToken, quantity]);

  return (
    <div id="checkout">
      {clientSecret && (
        <EmbeddedCheckoutProvider
          stripe={stripePromise}
          options={{ clientSecret }}
        >
          <EmbeddedCheckout />
        </EmbeddedCheckoutProvider>
      )}
    </div>
  );
}

export default StripeCheckoutForm;
