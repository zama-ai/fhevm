import Details from "./Details";
import { PageLayout } from "../../page-layout";
import Subscription from "./Subscription";
import config from "../../../config";

const openStripeManagement = (email = "") => {
  window.open(
    `${config.stripe.managementUrl}?prefilled_email=${email}`,
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
