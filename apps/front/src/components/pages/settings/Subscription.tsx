import { Link } from "react-router-dom";

import useSubscriptions from "../../../hooks/useSubscriptions";
import useAuth from "../../../hooks/useAuth";
import { PageLoader } from "../../page-loader";
import usePlans from "../../../hooks/usePlans";
import NoticeBox from "../../notice-box";
import noPriceIcon from "../../../images/icons/empty-state-price.svg";
import SubDisplay from "./SubDisplay";
import config from "../../../config";

function Subscription() {
  const { isAuthenticated, isLoading, user, idToken, accessToken } = useAuth();
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
    <>
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
        </>
      )}
      {subscriptions?.length > 0 && (
        <div className="plans--container">
          {subscriptions.map((sub) => (
            <SubDisplay sub={sub} key={sub.subscription_id} plans={plans} />
          ))}
        </div>
      )}
    </>
  );
}

export default Subscription;
