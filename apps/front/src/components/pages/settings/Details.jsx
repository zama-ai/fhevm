import { useAuth0 } from "@auth0/auth0-react";
import PropTypes from "prop-types";

import { PageLoader } from "../../page-loader";
import NoticeBox from "../../notice-box";
import profileIcon from "../../../images/icons/user.svg";
import config from "../../../config";

function Details(props) {
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
    <>
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
                  href={config.links.docs.relayerSdk}
                  target="_blank"
                  rel="noreferrer noopener"
                >
                  <button className="button button__link">See Docs</button>
                </a>
                <a
                  href={config.links.zama}
                  target="_blank"
                  rel="noreferrer noopener"
                >
                  <button className="button button--outline-secondary">
                    Go to Zama
                  </button>
                </a>
              </>
            }
          />
        </div>
      )}
    </>
  );
}

Details.propTypes = {
  openStripeManagement: PropTypes.func.isRequired,
};

export default Details;
