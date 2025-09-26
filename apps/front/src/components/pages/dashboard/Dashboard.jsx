import { PageLayout } from "../../page-layout";
import { useEffect, useState } from "react";
import { PageLoader } from "../../page-loader";
import MoesifEmbeddedTemplate from "../../moesif/moesif-embedded-template";
import NoticeBox from "../../notice-box";
import dashIcon from "../../../images/icons/bar-chart.svg";
import useAuth from "../../../hooks/useAuth";
import fetchEmbedChartUrls from "./fetchEmbedChartUrls";
import config from "../../../config";

const Dashboard = () => {
  const { user, isLoading, idToken, userEmail } = useAuth();

  const [error, setError] = useState();
  const [embedTemplateUrls, setEmbedTemplateUrls] = useState(null);

  const email = user?.email || userEmail;

  useEffect(() => {
    window?.moesif?.track("viewed-dashboard");

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
    return (
      <PageLayout>
        <PageLoader />
      </PageLayout>
    );
  }

  return (
    <PageLayout>
      <h1>My Dashboards</h1>
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
                href={config.links.docs.relayerSdk}
              >
                <button className="button">Relayer SDK</button>
              </a>
            </>
          }
        />
      )}
    </PageLayout>
  );
};

export default Dashboard;
