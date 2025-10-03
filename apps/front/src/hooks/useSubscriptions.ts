import { useState, useEffect } from "react";
import config from "../config";

type Props = {
  // TODO: improve user type
  user?: { email?: string };
  idToken: string;
  accessToken: string;
};

// This is set up as a hook so that
// in case other pages need subscription info
// it can also be reused.
export default function useSubscriptions({
  user,
  idToken,
  accessToken: _accessToken,
}: Props) {
  const [subscriptions, setSubscriptions] = useState(null);
  const [finishedLoading, setFinishedLoading] = useState(false);
  const [subscriptionsError, setSubscriptionsError] = useState(null);

  useEffect(() => {
    let controller: AbortController | undefined;
    if (user?.email && idToken) {
      controller = new AbortController();
      fetch(
        `${config.devPortalApiServer}/subscriptions?email=${encodeURIComponent(
          user.email
        )}`,
        {
          headers: {
            // for this project, we decide to use idToken
            // for two reasons:
            // - idToken already have email in the claim, and typically for user administrative related
            //   things like subscription, where the identify of user needs to be verify,
            //   it is perfect reasonable approach to use idToken.
            // - accessToken is more used with scopes, respect the scopes in accessToken and
            //   create the scopes, it will require a lot more configuration. and it is used more for
            //   machine to machine communication.
            "Content-Type": `application/json`,
            Authorization: `Bearer ${idToken}`,
          },
          signal: controller.signal,
        }
      )
        .then((res) => {
          if (!res.ok) {
            throw new Error(`Error fetching subscriptions: ${res.status}`);
          }
          return res.json();
        })
        .then((result) => {
          setFinishedLoading(true);
          setSubscriptions(result);
        })
        .catch((err) => {
          setFinishedLoading(true);
          setSubscriptionsError(err);
          console.error("failed to load subscriptions", err);
        });
    }

    // Cleanup function to abort the request
    return () => {
      controller?.abort();
    };
  }, [user, idToken]);

  return {
    subscriptions,
    finishedLoading,
    subscriptionsError,
  };
}
