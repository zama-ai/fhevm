import {
  withAuthenticationRequired,
  type WithAuthenticationRequiredOptions,
} from "@auth0/auth0-react";
import type { ComponentType } from "react";

import { PageLoader } from "./page-loader";

type Props = {
  component: ComponentType<object>;
  options?: WithAuthenticationRequiredOptions;
};

export const AuthenticationGuard = ({ component, options = {} }: Props) => {
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
