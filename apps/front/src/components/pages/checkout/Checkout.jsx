import React, { useEffect } from "react";

import useAuthCombined from "../../../hooks/useAuthCombined";
import { PageLoader } from "../../page-loader";
import { PageLayout } from "../../page-layout";
import StripeCheckoutForm from "./StripeCheckoutForm";
import { Navigate } from "react-router-dom";
import CustomCheckoutForm from "./CustomCheckoutForm";

function Checkout(props) {
  const { isLoading, user, idToken } = useAuthCombined();

  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const urlPriceIdToPurchase = urlParams.get("price_id_to_purchase");
  const urlPlanIdToPurchase = urlParams.get("plan_id_to_purchase");
  const urlQuantity = urlParams.get("quantity");

  useEffect(() => {
    window.moesif?.track("about-to-checkout", {
      price_id: urlPriceIdToPurchase,
      plan_id: urlPlanIdToPurchase,
      quantity: urlQuantity,
    });
  }, [urlPriceIdToPurchase, urlPlanIdToPurchase, urlQuantity]);

  if (isLoading || !idToken) {
    return <PageLoader />;
  }

  if (!urlPriceIdToPurchase && !urlPlanIdToPurchase) {
    <Navigate to="/plans" />;
  }

  return (
    <PageLayout>
      <h1>Subscribe</h1>
      <div className="page-layout__focus">
        {import.meta.env.REACT_APP_PAYMENT_PROVIDER === "custom" ? (
          <CustomCheckoutForm
            key={urlPriceIdToPurchase}
            priceId={urlPriceIdToPurchase}
            planId={urlPlanIdToPurchase}
            user={user}
            idToken={idToken}
          />
        ) : (
          <StripeCheckoutForm
            key={urlPriceIdToPurchase}
            priceId={urlPriceIdToPurchase}
            quantity={urlQuantity}
            user={user}
            idToken={idToken}
          />
        )}
      </div>
    </PageLayout>
  );
}

export default Checkout;
