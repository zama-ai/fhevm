import React from "react";

import { PageLayout } from "../../page-layout";
import useSubscriptions from "../../../hooks/useSubscriptions";
import useAuthCombined from "../../../hooks/useAuthCombined";
import { PageLoader } from "../../page-loader";
import { Link } from "react-router-dom";
import usePlans from "../../../hooks/usePlans";
import NoticeBox from "../../notice-box";
import noPriceIcon from "../../../images/icons/empty-state-price.svg";
import SubDisplay from "./SubDisplay";

function Subscription(props) {
  const { isAuthenticated, isLoading, user, idToken, accessToken } =
    useAuthCombined();
  const { subscriptions, finishedLoading, subscriptionsError } =
    useSubscriptions({
      user,
      idToken,
      accessToken,
    });
  const { plansLoading, plans } = usePlans();

  if (
    isLoading ||
    !finishedLoading ||
    !isAuthenticated ||
    plansLoading ||
    !idToken
  ) {
    return <PageLoader />;
  }

  return (
    <PageLayout>
      <h1>My Current Subscriptions</h1>
      {(!subscriptions || subscriptions.length <= 0) && (
        <>
          <NoticeBox
            iconSrc={noPriceIcon}
            title={subscriptionsError?.toString() || "No Subscription found"}
            description={
              <span>
                If you just purchased a plan, please{" "}
                <strong>wait at least 10 to 15 minutes</strong> for the systems
                to sync.
              </span>
            }
            actions={
              <>
                <a
                  href="https://www.moesif.com/docs/product-catalog/"
                  target="_blank"
                  rel="noreferrer noopener"
                >
                  <button className="button button__link">See Docs</button>
                </a>
                <Link
                  to="/plans"
                  rel="noreferrer noopener"
                >
                  <button className="button button--outline-secondary">
                    Go to Plans
                  </button>
                </Link>
              </>
            }
          />
        </>
      )}
      {subscriptions?.length > 0 && (
        <div className="plans--container">
          {subscriptions.map((sub) => (
            <SubDisplay sub={sub} key={sub.subscription_id} plans={plans} />
          ))}
        </div>
      )}
      <p className="text-muted">
        Newly created subscriptions may take up to 10 to 15 minutes to sync. For
        developers, if you want subscriptions to sync faster, you can locally
        cache the subscription in your system. In this example project, there is
        no local database or storage.
      </p>
    </PageLayout>
  );
}

export default Subscription;
