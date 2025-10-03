import { useState, useEffect, useRef, useMemo } from "react";
import { Navigate, useSearchParams } from "react-router-dom";
import { PageLayout } from "../../page-layout";
import { Link } from "react-router-dom";
import noPriceIcon from "../../../images/icons/empty-state-price.svg";
import NoticeBox from "../../notice-box";
import useAuth from "../../../hooks/useAuth";
import { moesifIdentifyUserFrontEndIfPossible } from "../../../common/utils";
import { PageLoader } from "../../page-loader";
import config from "../../../config";
import usePlans from "../../../hooks/usePlans";

// used on embedded checkout example code:
// https://docs.stripe.com/checkout/embedded/quickstart
// Purpose of this page is to
// - Confirmation for customer
// - receive the returned sessionId from Stripe and call backend API to provision services.

function registerPurchaseStripe({
  sessionId,
  idToken,
  setStatus,
  setLoading,
  setProvisionError,
}: {
  sessionId?: string;
  idToken?: string;
  setStatus: (status: string) => void;
  setLoading: (loading: boolean) => void;
  setProvisionError: (error: string) => void;
}) {
  setLoading(true);

  fetch(`${config.devPortalApiServer}/register/stripe/${sessionId}`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${idToken}`,
    },
  })
    .then(async (res) => {
      if (!res.ok) {
        const errorBody = await res.json();
        throw new Error(
          `Failed provision: ${res.status}, body: ${JSON.stringify(errorBody)}`
        );
      }
      return res.json();
    })
    .then((data) => {
      setStatus(data.status);
    })
    .catch((err) => {
      setProvisionError(err);
    })
    .finally(() => {
      setLoading(false);
      moesifIdentifyUserFrontEndIfPossible(idToken);
    });
}

function Return() {
  const [status, setStatus] = useState<string>();
  const [loading, setLoading] = useState(false);
  const [provisionError, setProvisionError] = useState<string>();
  const { idToken } = useAuth();

  const [searchParams] = useSearchParams();
  const { sessionId, planId, priceId } = useMemo(
    () => ({
      sessionId: searchParams.get("session_id"),
      planId: searchParams.get("plan_id"),
      priceId: searchParams.get("price_id"),
    }),
    [searchParams]
  );

  const { plans } = usePlans();
  const planName = useMemo(
    () => plans?.find((p) => p.id === planId)?.name || "selected",
    [plans, planId]
  );

  const isPurchaseRegistered = useRef(false);
  useEffect(() => {
    if (sessionId && idToken) {
      if (!isPurchaseRegistered.current) {
        // Prevent double calls
        isPurchaseRegistered.current = true;
        registerPurchaseStripe({
          sessionId,
          idToken,
          setStatus,
          setLoading,
          setProvisionError,
        });
      } else {
        console.debug("request already performed");
      }
    } else {
      console.error("no session id found for stripe");
    }
  }, [sessionId, idToken, setStatus, setLoading, setProvisionError]);

  useEffect(() => {
    window.moesif?.track("stripe-checkout-returned", {
      stripe_session_id: sessionId,
      price_id: priceId,
      status,
    });
  }, [sessionId, priceId, status]);

  if (status === "open") {
    return <Navigate to={`/checkout?price_id=${priceId}`} />;
  }

  // for stripe sessionId is required, but if isCustom, for developers you may have to determine
  // what is considered success.
  if (status === "complete" && sessionId) {
    return (
      <PageLayout>
        <h1>Subscribe</h1>
        <NoticeBox
          iconSrc={noPriceIcon}
          title="Success"
          description={`You are now subscribed to the ${planName} plan.`}
          actions={
            <>
              <a
                href={config.links.docs.relayerSdk}
                target="_blank"
                rel="noreferrer noopener"
              >
                <button className="button button__link">See Docs</button>
              </a>
              <Link to="/keys" rel="noreferrer noopener">
                <button className="button button--outline-secondary">
                  Get API Key
                </button>
              </Link>
            </>
          }
        />
      </PageLayout>
    );
  }

  if (loading) {
    return <PageLoader />;
  }

  return (
    <PageLayout>
      <h1>Subscribe Status</h1>
      <NoticeBox
        iconSrc={noPriceIcon}
        title={provisionError ? "Provision Service Failed" : "Checkout Failed"}
        description={
          provisionError
            ? provisionError.toString() +
              " Please check the route /register/stripe/, and see if you set up provision plugin correctly for your API gateway."
            : "Seems you didn't checkout successfully?"
        }
        actions={
          <>
            <a
              href={config.links.docs.relayerSdk}
              target="_blank"
              rel="noreferrer noopener"
            >
              <button className="button button__link">See Docs</button>
            </a>
            <Link to="/plans" rel="noreferrer noopener">
              <button className="button button--outline-secondary">
                Go to Plans
              </button>
            </Link>
          </>
        }
      />
    </PageLayout>
  );
}

export default Return;
