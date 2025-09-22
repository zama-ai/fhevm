import { useEffect } from "react";

import { PageLayout } from "../../page-layout";
import MoesifPlans from "./MoesifPlans";

function Plans() {
  useEffect(() => {
    window?.moesif?.track("viewed-plans-page");
  }, []);
  return (
    <PageLayout>
      <MoesifPlans />
    </PageLayout>
  );
}

export default Plans;
