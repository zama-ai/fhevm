import { withAuthenticationRequired } from "@auth0/auth0-react";
import PropTypes from "prop-types";

import { PageLoader } from "./page-loader";

export const AuthenticationGuard = ({ component, options = {} }) => {
  const Component = withAuthenticationRequired(component, {
    ...options,
    onRedirecting: () => (
      <div className="page-layout">
        <PageLoader />
      </div>
    ),
  });

  return <Component />;
};

AuthenticationGuard.propTypes = {
  component: PropTypes.elementType.isRequired,
  options: PropTypes.object,
};
