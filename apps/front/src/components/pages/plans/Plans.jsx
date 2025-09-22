import React, { useEffect } from "react";

import { PageLayout } from "../../page-layout";
import MoesifPlans from "./MoesifPlans";

function Plans(props) {
  useEffect(() => {
    window?.moesif?.track("viewed-plans-page");
  }, []);
  return (
    <PageLayout>
      <MoesifPlans skipExample />
    </PageLayout>
  );
}

export default Plans;
