import React from "react";

import { useAuth0 } from "@auth0/auth0-react";
import { PageLayout } from "../../page-layout";
import { PageLoader } from "../../page-loader";
import NoticeBox from "../../notice-box";
import profileIcon from "../../../images/icons/user.svg";

function Auth0Settings(props) {
  const {
    user: auth0User,
    isAuthenticated: auth0IsAuthenticated,
    isLoading: auth0IsLoading,
  } = useAuth0();

  const { openStripeManagement } = props;

  let isLoading = auth0IsLoading,
    isAuthenticated = auth0IsAuthenticated,
    user = auth0User;

  if (isLoading) {
    return <PageLoader />;
  }

  return (
    <PageLayout>
      <h1>My Settings</h1>
      {isAuthenticated && user ? (
        <div className="container-box">
          <div className="user-profile">
            {user.picture && (
              <img
                className="profile-picture"
                src={user.picture}
                alt={user.name}
              />
            )}
            <h1>{user.name}</h1>
          </div>
          <div>
            <button
              disabled={!user.email}
              className="button__purp"
              onClick={() => openStripeManagement(user.email)}
            >
              Manage Billing
            </button>
          </div>
        </div>
      ) : (
        <div>
          <NoticeBox
            iconSrc={profileIcon}
            title="No Profile found"
            description="There might have been an error"
            actions={
              <>
                <a
                  href="https://www.moesif.com/docs/developer-portal/developer-portal-overview/"
                  target="_blank"
                  rel="noreferrer noopener"
                >
                  <button className="button button__link">See Docs</button>
                </a>
                <a
                  href="https://www.moesif.com"
                  target="_blank"
                  rel="noreferrer noopener"
                >
                  <button className="button button--outline-secondary">
                    Go to Moesif
                  </button>
                </a>
              </>
            }
          />
        </div>
      )}
    </PageLayout>
  );
}

export default Auth0Settings;
