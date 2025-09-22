import React from "react";
import { Link } from "react-router-dom";

function CustomCheckoutForm({ planId, priceId, user, idToken }) {
  return (
    <div id="checkout">
      <div>
        <h3>
          For custom billing provider, please implement your custom check out
          flow here.
        </h3>
        <ul>
          <li>
            Typically, the flow involves redirecting to a payment gateway.
          </li>
          <li>
            Upon successful payment, it should come back to{" "}
            <code>
              http://127.0.0.1/return?price_id={encodeURIComponent(priceId)}
              &plan_id={encodeURIComponent(planId)}
            </code>
            , with some sort of success code or identifier
          </li>
          <li>
            On return page, it would make API call to backend:
            <ul>
              <li>to verify the payment is successful.</li>
              <li>to create the subscription.</li>
              <li>to provision any services.</li>
            </ul>
          </li>
          <li>
            For now: to simulate success return from payment provider, continue
            to:{" "}
            <strong>
              <Link
                to={`/return?price_id=${encodeURIComponent(
                  priceId
                )}&plan_id=${encodeURIComponent(planId)}`}
              >
                <button>return page</button>
              </Link>
            </strong>
          </li>
        </ul>
      </div>
    </div>
  );
}

export default CustomCheckoutForm;
