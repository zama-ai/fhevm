import { useEffect, useMemo } from "react";

import useAuth from "../../../hooks/useAuth";
import { PageLoader } from "../../page-loader";
import { PageLayout } from "../../page-layout";
import StripeCheckoutForm from "./StripeCheckoutForm";
import { Navigate, useSearchParams } from "react-router-dom";

function Checkout() {
  const { isLoading, user, idToken } = useAuth();

  const [searchParams] = useSearchParams();
  const { priceId, planId, quantity } = useMemo(
    () => ({
      priceId: searchParams.get("price_id"),
      planId: searchParams.get("plan_id"),
      quantity: searchParams.get("quantity"),
    }),
    [searchParams]
  );

  useEffect(() => {
    window.moesif?.track("about-to-checkout", {
      price_id: priceId,
      plan_id: planId,
      quantity,
    });
  }, [priceId, planId, quantity]);

  if (isLoading || !idToken) {
    return <PageLoader />;
  }

  if (!priceId) {
    return <Navigate to="/plans" />;
  }

  return (
    <PageLayout>
      <h1>Subscribe</h1>
      <div className="page-layout__focus">
        <StripeCheckoutForm
          key={priceId}
          priceId={priceId}
          quantity={quantity}
          user={user!}
          idToken={idToken}
        />
      </div>
    </PageLayout>
  );
}

export default Checkout;
