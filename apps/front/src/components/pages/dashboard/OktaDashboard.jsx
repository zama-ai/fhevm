import { useOktaAuth } from "@okta/okta-react";
import { PageLayout } from "../../page-layout";
import { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { PageLoader } from "../../page-loader";
import MoesifEmbeddedTemplate from "../../moesif/moesif-embedded-template";

const OktaDashboard = (props) => {
  const { authState } = useOktaAuth();

  const { fetchEmbedInfo } = props;

  let isLoading = !authState?.isAuthenticated;
  let user = authState?.idToken?.claims;

  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const checkout_session_id = searchParams.get("checkout-session");
  const [error, setError] = useState();
  const [iFrameSrcLiveEvent, setIFrameSrcLiveEvent] = useState();
  const [iFrameSrcTimeSeries, setIFrameSrcTimeSeries] = useState();

  useEffect(() => {
    if (isLoading) {
      return <PageLoader />;
    }

    if (checkout_session_id) {
      fetch(
        `${import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER}/register/stripe/${checkout_session_id}`,
        {
          method: "POST",
          headers: {
            // 'Authorization': should be the okta access token.
          },
        }
      )
        .then((res) => res.json())
        .then((result) => {
          fetchEmbedInfo(
            result.customer,
            setIFrameSrcLiveEvent,
            setIFrameSrcTimeSeries,
            setError
          );
        });
    } else {
      fetch(
        `${import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER}/stripe/customer?email=` +
          encodeURIComponent(user.email),
        {
          headers: {
            // 'Authorization': should be the okta access token
          },
        }
      )
        .then((res) => {
          if (res.status === "404") {
            return null;
          }
          return res.json();
        })
        .then((customer) => {
          if (!customer) {
            navigate("/product-select");
          } else {
            fetchEmbedInfo(
              customer.id,
              setIFrameSrcLiveEvent,
              setIFrameSrcTimeSeries,
              setError
            );
          }
        });
    }
  }, [isLoading, navigate, checkout_session_id, user, fetchEmbedInfo]);

  return (
    <PageLayout>
      <h1>Dashboard</h1>

      <>
        <MoesifEmbeddedTemplate
          iFrameSrcLiveEvent={iFrameSrcLiveEvent}
          iFrameSrcTimeSeries={iFrameSrcTimeSeries}
          error={error}
        />
      </>
    </PageLayout>
  );
};

export default OktaDashboard;
