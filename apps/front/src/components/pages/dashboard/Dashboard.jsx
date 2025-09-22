import { PageLayout } from "../../page-layout";
import { useEffect, useState } from "react";
import { PageLoader } from "../../page-loader";
import MoesifEmbeddedTemplate from "../../moesif/moesif-embedded-template";
import NoticeBox from "../../notice-box";
import dashIcon from "../../../images/icons/bar-chart.svg";
import useAuthCombined from "../../../hooks/useAuthCombined";
import fetchEmbedChartUrls from "./fetchEmbedChartUrls";

const Dashboard = (props) => {
  const { user, isLoading, idToken, userEmail } = useAuthCombined();

  const [error, setError] = useState();
  const [embedTemplateUrls, setEmbedTemplateUrls] = useState(null);

  const email = user?.email || userEmail;

  useEffect(() => {
    window?.moesif?.track('viewed-dashboard');

    if (idToken) {
      fetchEmbedChartUrls({
        authUserId: user?.user_id || user.id || user?.sub,
        idToken,
        email,
      })
        .then((embedInfos) => {
          setEmbedTemplateUrls(embedInfos);
        })
        .catch((err) => {
          console.error("failed to load embed dash", err);
          setError(err);
        });
    }
  }, [idToken, user, email]);

  if (isLoading || !idToken || (!error && !embedTemplateUrls)) {
    return <PageLoader />;
  }

  return (
    <PageLayout>
      <h1>My Dashboards</h1>
      <p>
        Please see{" "}
        <a
          className="button__link"
          target="_blank"
          href="https://www.moesif.com/docs/embedded-templates/"
        >
          Moesif Embedded Metric
        </a>{" "}
        docs to for details regarding configuration,{" "}
        <a
          className="button__link"
          target="_blank"
          href="https://www.moesif.com/docs/embedded-templates/creating-and-using-templates/#display-options"
        >
          display options
        </a>
        , and setup instructions.
      </p>
      {!error && (
        <MoesifEmbeddedTemplate embedTemplateUrls={embedTemplateUrls || []} />
      )}
      {error && (
        <NoticeBox
          iconSrc={dashIcon}
          title={error.toString()}
          description={
            <p>
              Did you set up your environment variables correctly for embedded
              dashboard and charts?
            </p>
          }
          actions={
            <>
              <a
                target="_blank"
                href="https://www.moesif.com/docs/embedded-templates/"
              >
                <button className="button button__link">
                  Embedded Metric Docs
                </button>
              </a>
              <a
                target="_blank"
                href="https://www.moesif.com/docs/developer-portal/configuring-the-dashboard/"
              >
                <button className="button">Dev Portal Docs</button>
              </a>
            </>
          }
        />
      )}
    </PageLayout>
  );
};

export default Dashboard;
