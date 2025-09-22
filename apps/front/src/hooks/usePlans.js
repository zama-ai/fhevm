import { useState, useEffect } from "react";

// This is set up as a hook so that
// in case other pages need subscription info
// it can also be reused.
export default function usePlans() {
  const [error, setError] = useState();
  const [loading, setLoading] = useState(true);
  const [plans, setPlans] = useState(null);

  useEffect(() => {
    fetch(`${import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER}/plans`)
      .then((res) => res.json())
      .then((result) => {
        const loadedPlans = result?.hits || [];
        const activePlans = loadedPlans.filter(
          (item) => item.status === "active"
        );
        setPlans(activePlans);
        setLoading(false);
      })
      .catch((err) => {
        console.log("failed to load plans", err);
        setLoading(false);
        setError(err);
      });
  }, []);

  return {
    plansError: error,
    plansLoading: loading,
    plans
  };
}
