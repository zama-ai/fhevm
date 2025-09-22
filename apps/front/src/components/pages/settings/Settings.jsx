import Details from "./Details";
import { PageLayout } from "../../page-layout";
import Subscription from "./Subscription";

const openStripeManagement = (email) => {
  window.open(
    `${
      import.meta.env.REACT_APP_STRIPE_MANAGEMENT_URL
    }?prefilled_email=${email}`,
    "_blank",
    "noreferrer"
  );
};

const Settings = () => {
  return (
    <PageLayout>
      <Details openStripeManagement={openStripeManagement} />
      <Subscription />
    </PageLayout>
  );
};

export default Settings;
